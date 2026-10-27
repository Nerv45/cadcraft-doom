//! Viewport-only DOOM mode. No document or undo access is passed to the engine.
use cadcraft_doom::Game;
use egui::{Event, Key, Rect, TextureHandle};
use std::{collections::HashSet, path::PathBuf, time::Duration};

#[derive(Default)]
pub struct Mode {
    game: Option<Game>,
    texture: Option<TextureHandle>,
    rect: Option<Rect>,
    focused: bool,
    held: HashSet<Key>,
    modifiers: egui::Modifiers,
    exit_requested: bool,
}
impl Mode {
    pub fn active(&self) -> bool {
        self.game.is_some()
    }
    pub fn start(&mut self) -> Result<(), String> {
        if self.active() {
            return Err("DOOM is already running".into());
        }
        let path = locate_iwad()?;
        self.game = Some(Game::start(path)?);
        self.focused = true;
        self.held.clear();
        self.modifiers = egui::Modifiers::default();
        self.exit_requested = false;
        Ok(())
    }
    pub fn close(&mut self) {
        if let Some(game) = self.game.take() {
            game.keys(&[0; 256]);
            game.stop();
        }
        self.texture = None;
        self.focused = false;
        self.rect = None;
        self.held.clear();
        self.modifiers = egui::Modifiers::default();
    }
    fn release(&mut self) {
        self.held.clear();
        self.modifiers = egui::Modifiers::default();
        if let Some(game) = &self.game {
            game.keys(&[0; 256]);
        }
    }
    pub fn raw_input(&mut self, raw: &mut egui::RawInput) {
        if !self.active() {
            return;
        }
        if !raw.focused {
            self.focused = false;
            self.release();
        }
        for event in &raw.events {
            if let Event::PointerButton { pos, pressed: true, .. } = event {
                self.focused = self.rect.is_some_and(|r| r.contains(*pos));
                if !self.focused {
                    self.release();
                }
            }
        }
        // F12 is a host escape even when another control owns keyboard focus.
        if raw.events.iter().any(|e| matches!(e, Event::Key { key: Key::F12, pressed: true, .. })) {
            self.exit_requested = true;
        }
        if !self.focused {
            return;
        }
        for event in &raw.events {
            if let Event::ModifiersChanged(modifiers) = event {
                self.modifiers = *modifiers;
                if let Some(game) = &self.game {
                    game.keys(&key_state(&self.held, self.modifiers));
                }
            }
            if let Event::Key { key, pressed, repeat, modifiers, .. } = event {
                if *pressed && !repeat {
                    self.held.insert(*key);
                }
                self.modifiers = *modifiers;
                if !pressed {
                    self.held.remove(key);
                }
                if let Some(game) = &self.game {
                    game.keys(&key_state(&self.held, self.modifiers));
                }
            }
        }

        let keys = key_state(&self.held, self.modifiers);
        if let Some(game) = &self.game {
            game.keys(&keys);
        }
        // Hide keyboard/modifier input from egui widgets and CAD shortcuts.
        raw.events.retain(|e| !matches!(e, Event::Key { .. } | Event::Text(_) | Event::ModifiersChanged(_)));
        raw.events.push(Event::ModifiersChanged(egui::Modifiers::default()));
    }
}

// A portable executable works directly with DOOM1.WAD beside it. Environment
// overrides remain available for explicit user selection; no replacement IWAD.
fn locate_iwad() -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("CADCRAFT_DOOM_WAD").filter(|p| !p.is_empty()) {
        let path = PathBuf::from(path);
        return Ok(if path.is_dir() { path.join("DOOM1.WAD") } else { path });
    }
    let beside_exe = std::env::current_exe().ok().and_then(|p| p.parent().map(|p| p.join("DOOM1.WAD")));
    let in_working_dir = std::env::current_dir().ok().map(|p| p.join("DOOM1.WAD"));
    beside_exe.into_iter().chain(in_working_dir).find(|p| p.is_file())
        .ok_or_else(|| "DOOM1.WAD was not found. Put your original shareware DOOM1.WAD beside the CADCraft DOOM executable, or use Launch-CADCraft-DOOM.bat to select it.".into())
}
fn key_state(held: &HashSet<Key>, modifiers: egui::Modifiers) -> [u8; 256] {
    let mut keys = [0; 256];
    for key in held {
        let code = match key {
            Key::ArrowUp | Key::W => 0xad,
            Key::ArrowDown | Key::S => 0xaf,
            Key::ArrowLeft | Key::A => 0xac,
            Key::ArrowRight | Key::D => 0xae,
            Key::Space => 0xa2, // DoomGeneric KEY_USE, not ASCII space
            Key::Enter => 13,
            Key::Escape => 27,
            Key::Tab => 9,
            Key::Backspace => 0x7f,
            Key::Y => b'y',
            Key::N => b'n',
            Key::Num1 => b'1',
            Key::Num2 => b'2',
            Key::Num3 => b'3',
            Key::Num4 => b'4',
            Key::Num5 => b'5',
            Key::Num6 => b'6',
            Key::Num7 => b'7',
            _ => continue,
        };
        keys[usize::from(code)] = 1;
    }
    keys[0xa3] = u8::from(modifiers.ctrl); // DoomGeneric KEY_FIRE
    keys[0xb6] = u8::from(modifiers.shift);
    keys
}

pub fn command(app: &mut crate::CadApp, id: &str) -> Result<serde_json::Value, String> {
    let result = if id == "doomexit" {
        app.doom.close();
        Ok(())
    } else if app.session.prompting() || app.session.lisp_busy() {
        Err("Finish the current CAD command before starting DOOM".into())
    } else if app.session.docs.is_empty() || app.ui.start_tab {
        Err("Open a drawing before starting DOOM".into())
    } else {
        app.doom.start()
    };
    if let Err(error) = &result {
        app.session.echo(error.clone());
        app.set_status(error.clone());
    }
    result.map(|()| serde_json::Value::Null)
}

pub fn tick(app: &mut crate::CadApp, ctx: &egui::Context) {
    if app.doom.exit_requested {
        app.doom.exit_requested = false;
        app.doom.close();
    }
    if let Some(done) = app.doom.game.as_ref().and_then(Game::finished) {
        app.doom.close();
        let message = match done {
            Ok(()) => "DOOM exited. Restart CADCraft to play again.".into(),
            Err(e) => format!("DOOM: {e}"),
        };
        app.session.echo(message.clone());
        app.set_status(message);
    }
    if app.doom.active() {
        ctx.request_repaint_after(Duration::from_millis(28));
    }
}

pub fn show(app: &mut crate::CadApp, ui: &mut egui::Ui) {
    let mut close = false;
    ui.horizontal(|ui| {
        ui.strong("CADCraft DOOM");
        close = ui.button("Exit DOOM (F12)").clicked();
        ui.label(if app.doom.focused { "Arrows/WASD · Ctrl fire · Space use · Esc menu" } else { "Click the game to capture keyboard" });
    });
    if close {
        app.doom.close();
        return;
    }
    // A small command field remains available while game capture is released.
    let mut command = String::new();
    ui.horizontal(|ui| {
        let label = ui.label("Command:");
        let response = ui.add(egui::TextEdit::singleline(&mut app.cmd.buffer).id_salt("doom-command").hint_text("DOOMEXIT")).labelled_by(label.id);
        if response.has_focus() {
            app.doom.focused = false;
            app.doom.release();
        }
        if response.lost_focus() && ui.input(|i| i.key_pressed(Key::Enter)) {
            command = std::mem::take(&mut app.cmd.buffer);
        }
    });
    if !command.is_empty() {
        if command.trim().eq_ignore_ascii_case("doomexit") {
            app.doom.close();
            return;
        }
        app.set_status("Use DOOMEXIT or Exit DOOM to return to the drawing");
    }
    let area = ui.available_rect_before_wrap();
    let response = ui.allocate_rect(area, egui::Sense::click());
    app.doom.rect = Some(area);
    if response.clicked() {
        app.doom.focused = true;
        if let Some(id) = ui.memory(|m| m.focused()) {
            ui.memory_mut(|m| m.surrender_focus(id));
        }
    }
    ui.painter().rect_filled(area, 0.0, egui::Color32::BLACK);
    if let Some(game) = &mut app.doom.game
        && let Some(pixels) = game.frame()
    {
        let image = egui::ColorImage::from_rgba_unmultiplied([cadcraft_doom::WIDTH, cadcraft_doom::HEIGHT], pixels);
        if let Some(texture) = &mut app.doom.texture {
            texture.set(image, egui::TextureOptions::NEAREST);
        } else {
            app.doom.texture = Some(ui.ctx().load_texture("cadcraft-doom", image, egui::TextureOptions::NEAREST));
        }
    }
    if let Some(texture) = &app.doom.texture {
        let scale = (area.width() / 4.0).min(area.height() / 3.0).max(0.0);
        let rect = Rect::from_center_size(area.center(), egui::vec2(scale * 4.0, scale * 3.0));
        ui.painter().image(texture.id(), rect, Rect::from_min_max(egui::Pos2::ZERO, egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
    } else {
        ui.painter().text(area.center(), egui::Align2::CENTER_CENTER, "Loading DOOM…", egui::FontId::proportional(18.0), egui::Color32::WHITE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mappings_and_alias_release() {
        let mut held = HashSet::from([Key::W, Key::ArrowUp, Key::Space]);
        let modifiers = egui::Modifiers { ctrl: true, shift: true, ..Default::default() };
        assert_eq!(key_state(&held, modifiers)[0xa3], 1);
        assert_eq!(key_state(&held, modifiers)[0xa2], 1);
        assert_eq!(key_state(&held, modifiers)[0xb6], 1);
        held.remove(&Key::W);
        assert_eq!(key_state(&held, modifiers)[0xad], 1);
        held.clear();
        assert_eq!(key_state(&held, Default::default()), [0; 256]);
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;
    use crate::{CadApp, Services};
    use cadcraft_engine::Session;
    #[test]
    fn input_capture_focus_release_and_exit_preserve_document() {
        let mut app = CadApp::new(Session::new(), Services::default());
        app.session.cmdline("circle 0,0 5").unwrap();
        let before = app.session.state().unwrap();
        let doc = before.doc.clone();
        let revision = before.revision;
        let undo = before.undo.len();
        let redo = before.redo.len();
        // A validation-only worker exercises UI mode without entering singleton C state.
        let missing = std::env::temp_dir().join(format!("cadcraft-doom-ui-missing-{}.wad", std::process::id()));
        app.doom.game = Some(Game::start(missing).unwrap());
        app.doom.focused = true;
        let mut raw = egui::RawInput::default();
        raw.events = vec![
            Event::ModifiersChanged(egui::Modifiers { ctrl: true, ..Default::default() }),
            Event::Key { key: Key::W, physical_key: None, pressed: true, repeat: false, modifiers: Default::default() },
            Event::Text("w".into()),
        ];
        app.raw_input_hook(&mut raw);
        assert!(app.doom.held.contains(&Key::W));
        assert!(!raw.events.iter().any(|e| matches!(e, Event::Key { .. } | Event::Text(_))));
        raw.focused = false;
        app.raw_input_hook(&mut raw);
        assert!(app.doom.held.is_empty());
        assert!(!app.doom.focused);
        app.cmdline("DOOMEXIT");
        assert!(!app.doom.active());
        let after = app.session.state().unwrap();
        assert!(std::sync::Arc::ptr_eq(&doc, &after.doc));
        assert_eq!((after.revision, after.undo.len(), after.redo.len()), (revision, undo, redo));
    }
}
