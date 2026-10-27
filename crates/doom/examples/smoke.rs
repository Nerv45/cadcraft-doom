//! Native smoke test; runs the same in-process backend, without a game window.
#[cfg(all(target_os = "windows", target_arch = "x86_64", target_env = "msvc"))]
fn main() -> Result<(), String> {
    use cadcraft_doom::Game;
    use std::{
        thread,
        time::{Duration, Instant},
    };
    let path = std::env::args_os().nth(1).ok_or("Pass an IWAD path")?;
    let mut game = Game::start(path.into())?;
    let start = Instant::now();
    let mut frames = 0;
    let mut last_hash = 0u64;
    let mut changed = 0;
    let mut enter_stage = 0;
    while start.elapsed() < Duration::from_secs(8) {
        if let Some(result) = game.finished() {
            return Err(format!("Early worker exit: {result:?}"));
        }
        if let Some(pixels) = game.frame() {
            frames += 1;
            let hash = pixels.iter().fold(0u64, |h, b| h.wrapping_mul(31).wrapping_add(u64::from(*b)));
            if hash != last_hash {
                changed += 1;
            }
            last_hash = hash;
        }
        // Start a new game using Enter; key up follows each key down.
        let stage = (start.elapsed().as_millis() / 400) as u32;
        if stage != enter_stage && (3..=10).contains(&stage) {
            let mut keys = [0; 256];
            keys[13] = u8::from(stage % 2 == 1);
            game.keys(&keys);
            enter_stage = stage;
        }
        if stage == 13 {
            let mut keys = [0; 256];
            keys[0xad] = 1;
            game.keys(&keys);
        }
        if stage == 16 {
            game.keys(&[0; 256]);
        }
        thread::sleep(Duration::from_millis(10));
    }
    if frames < 20 || changed < 10 {
        return Err(format!("Insufficient animation: {frames} frames, {changed} distinct transitions"));
    }
    game.stop();
    let stop = Instant::now();
    loop {
        if let Some(done) = game.finished() {
            done?;
            break;
        }
        if stop.elapsed() > Duration::from_secs(2) {
            return Err("Worker did not stop within 2 seconds".into());
        }
        thread::sleep(Duration::from_millis(5));
    }
    if Game::start("unused.wad".into()).is_ok() {
        return Err("Second engine invocation was incorrectly accepted".into());
    }
    println!("PASS: {frames} frames, {changed} changing frames; cooperative stop {:?}; restart rejected", stop.elapsed());
    Ok(())
}
#[cfg(not(all(target_os = "windows", target_arch = "x86_64", target_env = "msvc")))]
fn main() {
    eprintln!("Windows x64 MSVC only");
}
