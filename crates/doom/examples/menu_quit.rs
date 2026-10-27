//! Verify an engine-originated quit returns to Rust rather than exiting the host.
#[cfg(all(target_os = "windows", target_arch = "x86_64", target_env = "msvc"))]
fn main() -> Result<(), String> {
    use cadcraft_doom::Game;
    use std::{
        thread,
        time::{Duration, Instant},
    };
    let path = std::env::args_os().nth(1).ok_or("Pass IWAD path")?;
    let game = Game::start(path.into())?;
    thread::sleep(Duration::from_secs(2));
    for key in [27usize, b'q' as usize, 13, b'y' as usize] {
        let mut keys = [0; 256];
        keys[key] = 1;
        game.keys(&keys);
        thread::sleep(Duration::from_millis(200));
        game.keys(&[0; 256]);
        thread::sleep(Duration::from_millis(200));
    }
    let deadline = Instant::now();
    loop {
        if let Some(done) = game.finished() {
            done?;
            println!("PASS: DOOM internal Quit returned to the Rust host");
            return Ok(());
        }
        if deadline.elapsed() > Duration::from_secs(3) {
            return Err("Menu quit did not finish".into());
        }
        thread::sleep(Duration::from_millis(10));
    }
}
#[cfg(not(all(target_os = "windows", target_arch = "x86_64", target_env = "msvc")))]
fn main() {}
