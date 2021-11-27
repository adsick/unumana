use bevy::{input::keyboard::KeyboardInput, prelude::*};
use unumana::{Backend, Command, Controller, Editor, Keymap, Mode};

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

#[derive(Component)]
struct Scrollable;

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
                "".to_string(),
                regular.clone(),
                text_alignment,
            ),
            ..Default::default()
        })
        .insert(Scrollable);

    commands.insert_resource(Keymap::Dvorak);
}

fn scroll_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, (With<Text>, With<Scrollable>)>,
    controller: Query<&Controller>,
) {
    let controller = controller.single();

    let vert = controller.is_pressed(57416) as i32 - controller.is_pressed(57424) as i32;
    let horiz = controller.is_pressed(57421) as i32 - controller.is_pressed(57419) as i32;

    for mut transform in query.iter_mut() {
        transform.translation.y += 500.0 * time.delta_seconds() * vert as f32;
        transform.translation.x += 500.0 * time.delta_seconds() * horiz as f32;
        //dbg!(transform.translation);
    }
}

fn input(
    time: Res<Time>,
    mut input: EventReader<KeyboardInput>,
    mut controller: Query<&mut Controller>,
    mut evwc: EventWriter<Command>,
    mut keymap: ResMut<Keymap>,
) {
    let mut controller = controller.single_mut();
    let time = time.seconds_since_startup();

    for ki in input.iter() {
        let KeyboardInput {
            scan_code: sc,
            key_code: _,
            state,
        } = ki;
        // in this loop the only thing we want to do is to update the keys in controller
        if state == &bevy::input::ElementState::Pressed {
            let mut space = false;
            if let Some(duration) = controller.get_pressed_duration(*sc, time) {
                if *sc == 57 {
                    if controller.mode != Mode::Normal && duration > 0.5 {
                        controller.mode = Mode::Normal;
                        println!("57 triggered Normal mode, duration: {:.3}", duration);
                        continue;
                    }
                    space = true;
                }
            } else {
                let last = controller.press(sc, time);
                println!("{} just pressed", sc);
                println!("r({})p({}) gap is {:.3}", last.0.0, sc, time - last.0.1);
                println!("p({})p({}) gap is {:.3}\n", last.1.0, sc, time - last.1.1);
            }

            if controller.mode == Mode::Normal {
                if sc == &34 {
                    controller.mode = Mode::Insert;
                } else if sc == &30 {
                    evwc.send(Command::MoveCursorRightward);
                    controller.mode = Mode::Insert;
                }

                let extra = space || *sc == 57; //controller.is_pressed(32);

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
        } else {
            if let Some((last_pressed, last_released, t)) = controller.release(sc, time) {
                let duration = time - t;
                println!("{} just released, duration: {:.3}", sc, duration);
                println!("p({})r({}) gap is {:.3}", last_pressed.0, sc, time - last_pressed.1);
                println!("r({})r({}) gap is {:.3}\n", last_released.0, sc,  time - last_released.1);
                if controller.mode == Mode::Insert && sc == &57 && duration < 0.2 {
                    evwc.send(Command::PutCharAfterCursor(' '));
                }
            }
        }
    }
}

fn backend_update(
    // mut commands: Commands,
    mut backend: Query<&mut Backend>,
    mut evrc: EventReader<Command>,
) {
    // what if we somehow take ownership of the backend,
    // mutate it asynk tasks and then spawn it again?
    // let (entity, mut backend) = backend.single_mut().unwrap();
    // commands.entity(*entity).despawn();

    let mut backend = backend.single_mut();
    for c in evrc.iter() {
        backend.execute(c);
    }
}

fn frontend_update(mut frontend: Query<&mut Text>, backend: Query<&Backend, Changed<Backend>>) {
    if backend.is_empty(){
        return;
    }
    let backend = backend.single();
        frontend
            .single_mut()
            .sections
            .first_mut()
            .unwrap()
            .value = backend.render();
    
}

fn debug_system(
    time: Res<Time>,
    controller: Query<&Controller, Changed<Controller>>,
    backend: Query<&Backend>,
    keymap: Res<Keymap>,
) {
    if controller.is_empty(){
        return;
    }
    let controller = controller.single();
        controller.print_dbg();
        println!(
            "{} mode: {:?}, keymap: {:?}, time: {:.3}\n",
            backend.single().position(),
            controller.mode,
            keymap,
            time.seconds_since_startup()
        );
    
}
