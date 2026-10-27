use crate::{FRAME_BYTES, wad};
use std::{
    ffi::CString,
    io::Read,
    path::PathBuf,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc,
    },
    thread,
};

static CLAIMED: AtomicBool = AtomicBool::new(false);
unsafe extern "C" {
    fn cc_prepare();
    fn cc_stop();
    fn cc_keys(keys: *const u8);
    fn cc_frame(sequence: *mut u64, out: *mut u8) -> i32;
    fn cc_run(argc: i32, argv: *mut *mut std::ffi::c_char, error: *mut std::ffi::c_char, capacity: u32) -> i32;
}

struct Lease {
    entered: AtomicBool,
}
impl Drop for Lease {
    fn drop(&mut self) {
        if !self.entered.load(Ordering::Acquire) {
            CLAIMED.store(false, Ordering::Release);
        }
    }
}

pub struct Game {
    _lease: Arc<Lease>,
    finished: mpsc::Receiver<Result<(), String>>,
    sequence: u64,
    pixels: Vec<u8>,
}
impl Game {
    pub fn start(path: PathBuf) -> Result<Self, String> {
        if CLAIMED.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return Err("DOOM already ran or is running. Restart CADCraft to play again.".into());
        }
        // SAFETY: singleton claimed before reset; no previous engine can still run.
        unsafe {
            cc_prepare();
        }
        let lease = Arc::new(Lease { entered: AtomicBool::new(false) });
        let worker_lease = lease.clone();
        let (tx, finished) = mpsc::channel();
        if let Err(e) = thread::Builder::new().name("cadcraft-doom".into()).spawn(move || {
            let prepared = prepare(path);
            let outcome = match prepared {
                Ok(args) => {
                    worker_lease.entered.store(true, Ordering::Release);
                    run(args)
                }
                Err(e) => Err(e),
            };
            let _ = tx.send(outcome);
        }) {
            return Err(format!("Cannot create DOOM worker: {e}"));
        }
        Ok(Self { _lease: lease, finished, sequence: 0, pixels: vec![0; FRAME_BYTES] })
    }
    pub fn keys(&self, keys: &[u8; 256]) {
        // SAFETY: exactly 256 initialized bytes; C copies under its key lock.
        unsafe {
            cc_keys(keys.as_ptr());
        }
    }
    pub fn stop(&self) {
        // SAFETY: only sets an interlocked flag, does not touch engine globals.
        unsafe {
            cc_stop();
        }
    }
    pub fn frame(&mut self) -> Option<&[u8]> {
        // SAFETY: exact-sized writable output; C copies under frame lock.
        let fresh = unsafe { cc_frame(&mut self.sequence, self.pixels.as_mut_ptr()) };
        (fresh != 0).then_some(self.pixels.as_slice())
    }
    pub fn finished(&self) -> Option<Result<(), String>> {
        match self.finished.try_recv() {
            Ok(result) => Some(result),
            Err(mpsc::TryRecvError::Empty) => None,
            Err(mpsc::TryRecvError::Disconnected) => Some(Err("DOOM worker disconnected".into())),
        }
    }
}
impl Drop for Game {
    fn drop(&mut self) {
        self.stop();
    }
}

fn prepare(path: PathBuf) -> Result<Vec<CString>, String> {
    let file = std::fs::File::open(&path).map_err(|e| format!("Cannot open IWAD {}: {e}", path.display()))?;
    let mut bytes = Vec::new();
    file.take(wad::MAX_WAD_BYTES + 1).read_to_end(&mut bytes).map_err(|e| format!("Cannot read IWAD: {e}"))?;
    wad::validate(&bytes)?;
    // Stage the validated bytes to avoid a validation/use race and narrow-path encoding loss.
    let dir = std::env::var_os("CADCRAFT_DOOM_DATA")
        .map(PathBuf::from)
        .unwrap_or_else(|| std::env::temp_dir().join(format!("cadcraft-doom-{}", std::process::id())));
    let dir = std::path::absolute(dir).map_err(|e| e.to_string())?;
    let Some(root) = dir.to_str().filter(|s| s.is_ascii()) else {
        return Err("Set CADCRAFT_DOOM_DATA to an ASCII-only writable directory (native engine limitation)".into());
    };
    std::fs::create_dir_all(&dir).map_err(|e| format!("Cannot create DOOM data directory: {e}"))?;
    let staged_name = if path.file_name().is_some_and(|name| name.eq_ignore_ascii_case("doom1.wad")) { "DOOM1.WAD" } else { "game.wad" };
    std::fs::write(dir.join(staged_name), bytes).map_err(|e| format!("Cannot stage IWAD: {e}"))?;
    let values = vec![
        "cadcraft-doom".into(),
        "-iwad".into(),
        format!("{root}/{staged_name}"),
        "-nosound".into(),
        "-nogui".into(),
        "-config".into(),
        format!("{root}/default.cfg"),
        "-extraconfig".into(),
        format!("{root}/extra.cfg"),
        "-savedir".into(),
        root.into(),
    ];
    values.into_iter().map(|s: String| CString::new(s).map_err(|_| "NUL in DOOM path".into())).collect()
}
fn run(args: Vec<CString>) -> Result<(), String> {
    // Engine retains argv until cc_run returns; C may edit the pointer array.
    let mut storage: Vec<Vec<u8>> = args.into_iter().map(CString::into_bytes_with_nul).collect();
    let mut argv: Vec<_> = storage.iter_mut().map(|s| s.as_mut_ptr().cast()).collect();
    let argc = argv.len() as i32;
    argv.push(std::ptr::null_mut());
    let mut error = [0u8; 512];
    // SAFETY: owned writable NUL-terminated arguments outlive the synchronous C call;
    // only this worker enters C engine code. longjmp/SEH remain entirely within C.
    let result = unsafe { cc_run(argc, argv.as_mut_ptr(), error.as_mut_ptr().cast(), error.len() as u32) };
    if result == 0 {
        Ok(())
    } else {
        let end = error.iter().position(|b| *b == 0).unwrap_or(error.len());
        Err(String::from_utf8_lossy(error.get(..end).unwrap_or_default()).into_owned())
    }
}
