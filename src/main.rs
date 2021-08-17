use bevy::{core::FixedTimestep, input::keyboard::KeyboardInput, prelude::*};
use bokken::{Backend, Command, Controller, Editor, Keymap, Mode};

fn main() {
    //todo run frontend after backend
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(input.label("input"))
        .add_system(backend_update.label("backend")) //
        .add_system(scroll_system.label("frontend"))
        .add_system(frontend_update.label("frontend").after("backend"))
        .add_system(debug_system.after("backend"))
        .add_event::<Command>()
        .run();
}

struct AnimateTranslation;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //let bold_font = asset_server.load("terminal_land/TerminalLandMono-Bold.otf");
    let regular_font = asset_server.load("writer/Writer-Regular.ttf");

    // let bold = TextStyle {
    //     font: bold_font,
    //     font_size: 60.0,
    //     color: Color::WHITE,
    // };

    let regular = TextStyle {
        font: regular_font,
        font_size: 60.0,
        color: Color::WHITE,
    };

    let text_alignment = TextAlignment::default();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(Editor::default());

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(
                "writer regular font.".to_string(),
                regular.clone(),
                text_alignment,
            ),
            ..Default::default()
        })
        .insert(AnimateTranslation);

    commands.insert_resource(Keymap::Dvorak);
}

fn scroll_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<AnimateTranslation>)>,
    controller: Query<&Controller>,
) {
    let controller = controller.single().unwrap();

    let vert = controller.is_pressed(57416) as i32 - controller.is_pressed(57424) as i32;
    let horiz = controller.is_pressed(57421) as i32 - controller.is_pressed(57419) as i32;

    for mut transform in query.iter_mut() {
        transform.translation.y += 500.0 * time.delta_seconds() * vert as f32;
        transform.translation.x += 500.0 * time.delta_seconds() * horiz as f32;
        // dbg!(transform.translation);
    }
}

fn input(
    time: Res<Time>,
    mut input: EventReader<KeyboardInput>,
    mut controller: Query<&mut Controller>,
    mut evwc: EventWriter<Command>,
    mut keymap: ResMut<Keymap>,
) {
    let mut controller = controller.single_mut().unwrap();
    let time = time.seconds_since_startup();

    for ki in input.iter() {
        let KeyboardInput {
            scan_code: sc,
            key_code: _,
            state,
        } = ki;
        if state == &bevy::input::ElementState::Pressed {
            if let Some(duration) = controller.get_pressed_duration(*sc, time) {
                if *sc == 57 && controller.mode != Mode::Normal && duration > 0.5 {
                    controller.mode = Mode::Normal;
                    println!("57 triggered Normal mode, duration: {:.3}", duration);
                    // return;
                }
            } else {
                controller.press(sc, time);
                println!("{} just pressed", sc);

                if controller.mode == Mode::Normal {
                    if sc == &34 {
                        controller.mode = Mode::Insert;
                    } else if sc == &30 {
                        evwc.send(Command::MoveCursorRightward);
                        controller.mode = Mode::Insert;
                    }

                    let extra = controller.is_pressed(32);

                    if sc == &36 {
                        if controller.is_pressed(33) {
                            //U serves as mod key here
                            if extra {
                                evwc.send(Command::MoveCursorToTheFirstChar)
                            } else {
                                evwc.send(Command::MoveCursorLeftward);
                            }
                        } else {
                            evwc.send(Command::MoveCursorDownward);
                            //todo extra for moving to the end of file
                        }
                    } else if sc == &37 {
                        if controller.is_pressed(33) {
                            if extra {
                                evwc.send(Command::MoveCursorToTheEndOfTheLine)
                            } else {
                                evwc.send(Command::MoveCursorRightward)
                            }
                        } else {
                            evwc.send(Command::MoveCursorUpward)
                            //todo extra for moving to the beginning of the file
                        }
                    }
                } else if controller.mode == Mode::Insert {
                    if sc == &57 {
                        return;
                    }
                    if sc == &58 {
                        //Caps
                        keymap.switch();
                    } else if sc == &14 {
                        //Backspace
                        evwc.send(Command::RemoveCharBeforeCursor)
                    } else if sc == &28 {
                        evwc.send(Command::NewLineAfter)
                    } else {
                        let mut ch = keymap.convert(*sc);
                        controller
                            .is_pressed(42)
                            .then(|| ch = ch.to_uppercase().next().unwrap());
                        evwc.send(Command::PutCharAfterCursor(ch));
                    }
                }
            }
        } else {
            if let Some(duration) = controller.release(sc, time) {
                println!("{} just released, duration: {:.3}", sc, duration);
                if controller.mode == Mode::Insert && sc == &57 && duration < 0.2 {
                    evwc.send(Command::PutCharAfterCursor(' '));
                }
            }
        }
    }
}

fn backend_update(
    // time: Res<Time>,
    mut backend: Query<&mut Backend>,
    mut evrc: EventReader<Command>,
) {
    // println!("backend_update begin ");
    // let instant = std::time::Instant::now();
    let mut backend = backend.single_mut().unwrap();

    for c in evrc.iter() {
        backend.execute(c);
    }

    // std::thread::sleep(std::time::Duration::from_millis(100));
}

fn frontend_update(mut frontend: Query<&mut Text>, backend: Query<&Backend>) {
    let backend = backend.single().unwrap();
    frontend
        .single_mut()
        .unwrap()
        .sections
        .first_mut()
        .unwrap()
        .value = backend.render();
}

fn debug_system(
    time: Res<Time>,
    controller: Query<&Controller, Changed<Controller>>,
    backend: Query<&Backend>,
) {
    if let Ok(controller) = controller.single() {
        controller.print_dbg();
        println!(
            "{} mode: {:?}, time: {:.3}\n",
            backend.single().unwrap().position(),
            controller.mode,
            time.seconds_since_startup()
        );
    }
}
