#[cfg(all(target_os = "windows", target_arch = "x86_64", target_env = "msvc"))]
#[test]
fn invalid_files_are_reported_and_can_be_retried() {
    use cadcraft_doom::Game;
    use std::{
        thread,
        time::{Duration, Instant},
    };
    let path = std::env::temp_dir().join(format!("cadcraft-doom-invalid-{}.wad", std::process::id()));
    for exists in [false, true] {
        if exists {
            std::fs::write(&path, b"not an IWAD").unwrap();
        }
        let game = Game::start(path.clone()).unwrap();
        assert!(Game::start(path.clone()).is_err(), "must reject simultaneous workers");
        let start = Instant::now();
        loop {
            if let Some(done) = game.finished() {
                assert!(done.is_err());
                break;
            }
            assert!(start.elapsed() < Duration::from_secs(5));
            thread::sleep(Duration::from_millis(5));
        }
        drop(game); // releases singleton lease only after validation worker is gone
        thread::sleep(Duration::from_millis(10));
    }
    std::fs::remove_file(path).unwrap();
}
