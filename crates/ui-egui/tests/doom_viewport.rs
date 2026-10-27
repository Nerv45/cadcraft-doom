//! Opt-in end-to-end render/input test using the real CADCraft UI shell.
#![cfg(all(feature = "doom", target_os = "windows", target_arch = "x86_64", target_env = "msvc"))]
use cadcraft_engine::Session;
use cadcraft_ui_egui::{CadApp, Services};
use egui::{Event, Key, Modifiers};
use egui_kittest::Harness;
use egui_kittest::kittest::Queryable;
use std::{sync::Arc, thread, time::Duration};

fn frame(h: &mut Harness<'_, CadApp>, events: Vec<Event>) {
    let mut raw = egui::RawInput { events, focused: true, ..Default::default() };
    h.state_mut().raw_input_hook(&mut raw);
    h.input_mut().events.extend(raw.events);
    h.step();
}
fn key(h: &mut Harness<'_, CadApp>, key: Key, down: bool) {
    frame(h, vec![Event::Key { key, physical_key: None, pressed: down, repeat: false, modifiers: Modifiers::NONE }]);
}
fn tick(h: &mut Harness<'_, CadApp>, count: usize) {
    for _ in 0..count {
        thread::sleep(Duration::from_millis(30));
        frame(h, vec![]);
    }
}

#[test]
#[ignore = "requires CADCRAFT_DOOM_WAD and a rendering adapter; consumes one native engine session"]
fn playable_viewport_exit_and_cad_resume() {
    let output = std::path::PathBuf::from(std::env::var_os("CADCRAFT_DOOM_TEST_OUTPUT").expect("Set CADCRAFT_DOOM_TEST_OUTPUT"));
    std::fs::create_dir_all(&output).unwrap();
    let mut app = CadApp::new(Session::new(), Services::default());
    app.run("ui.sample", serde_json::json!({})).unwrap();
    let mut h = Harness::builder().with_size(egui::vec2(1400.0, 900.0)).build_ui_state(
        |ui, app: &mut CadApp| {
            app.logic(&ui.ctx().clone());
            app.ui(ui);
        },
        app,
    );
    h.run_steps(5);
    h.render().unwrap().save(output.join("01-cad-before.png")).unwrap();
    let st = h.state().session.state().unwrap();
    let doc = st.doc.clone();
    let baseline = (st.revision, st.undo.len(), st.redo.len());
    let view = format!("{:?}", st.view());
    let contents = (*doc).clone();
    frame(&mut h, vec![Event::Text("DOOM".into())]);
    key(&mut h, Key::Enter, true);
    key(&mut h, Key::Enter, false);
    assert!(h.state().doom_active(), "typed DOOM must activate the actual UI mode");
    tick(&mut h, 60);
    assert!(h.state().doom_active(), "engine must remain running after initialization");
    h.render().unwrap().save(output.join("02-doom-title.png")).unwrap();
    for _ in 0..4 {
        key(&mut h, Key::Enter, true);
        tick(&mut h, 4);
        key(&mut h, Key::Enter, false);
        tick(&mut h, 4);
    }
    tick(&mut h, 55);
    let before_move = h.render().unwrap();
    before_move.save(output.join("03-doom-game.png")).unwrap();
    frame(&mut h, vec![Event::ModifiersChanged(Modifiers { ctrl: true, ..Modifiers::NONE })]);
    tick(&mut h, 10);
    frame(&mut h, vec![Event::ModifiersChanged(Modifiers::NONE)]);
    tick(&mut h, 8);
    h.render().unwrap().save(output.join("03a-doom-fire.png")).unwrap();
    key(&mut h, Key::Escape, true);
    key(&mut h, Key::Escape, false);
    tick(&mut h, 3);
    assert!(h.state().doom_active(), "Escape opens the game menu, not CAD mode");
    key(&mut h, Key::Escape, true);
    key(&mut h, Key::Escape, false);
    key(&mut h, Key::W, true);
    tick(&mut h, 20);
    key(&mut h, Key::W, false);
    key(&mut h, Key::ArrowRight, true);
    tick(&mut h, 15);
    key(&mut h, Key::ArrowRight, false);
    let after_move = h.render().unwrap();
    after_move.save(output.join("04-doom-moved.png")).unwrap();
    let changed = before_move.pixels().zip(after_move.pixels()).filter(|(a, b)| a != b).count();
    assert!(changed > 1000, "game should visibly animate/respond to movement: {changed} pixels");
    assert!(h.state().cmd.buffer.is_empty(), "game keys must not type CAD commands");
    // Exit through the rendered command field using the real raw-input path.
    let command_field = h.get_by_role_and_label(egui::accesskit::Role::TextInput, "Command:").rect().center();
    frame(
        &mut h,
        vec![
            Event::PointerMoved(command_field),
            Event::PointerButton { pos: command_field, button: egui::PointerButton::Primary, pressed: true, modifiers: Modifiers::NONE },
        ],
    );
    frame(
        &mut h,
        vec![Event::PointerButton { pos: command_field, button: egui::PointerButton::Primary, pressed: false, modifiers: Modifiers::NONE }],
    );
    frame(&mut h, vec![Event::Text("DOOMEXIT".into())]);
    key(&mut h, Key::Enter, true);
    key(&mut h, Key::Enter, false);
    h.run_steps(5);
    assert!(!h.state().doom_active());
    let st = h.state().session.state().unwrap();
    assert!(Arc::ptr_eq(&doc, &st.doc));
    assert_eq!((st.revision, st.undo.len(), st.redo.len()), baseline);
    assert_eq!(format!("{:?}", st.view()), view);
    assert_eq!(*h.state().session.state().unwrap().doc, contents);
    h.render().unwrap().save(output.join("05-cad-restored.png")).unwrap();
    // Resume normal typed CAD interaction, then undo back to the exact original document.
    frame(&mut h, vec![Event::Text("circle 0,0 5".into())]);
    key(&mut h, Key::Enter, true);
    key(&mut h, Key::Enter, false);
    assert!(!Arc::ptr_eq(&doc, &h.state().session.state().unwrap().doc));
    h.state_mut().session.undo().unwrap();
    assert!(Arc::ptr_eq(&doc, &h.state().session.state().unwrap().doc));
    assert!(h.state_mut().run("doom", serde_json::json!({})).is_err(), "restart requires a new process");
    println!(
        "PASS: embedded viewport, typed DOOM, movement, DOOMEXIT, identical document/view/undo/redo, CAD typing and undo; {changed} changed game pixels"
    );
}
