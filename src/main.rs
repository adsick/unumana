use bevy::{input::keyboard::KeyboardInput, prelude::*};
use bokken::{Backend, Command, Controller, Editor, Keymap};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(animate_translation)
        .add_system(input)
        .add_system(backend_update)
        .add_system(frontend_update)
        .add_event::<Command>()
        .run();
}

struct AnimateTranslation;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bold_font = asset_server.load("terminal_land/TerminalLandMono-Bold.otf");
    let regular_font = asset_server.load("writer/Writer-Regular.ttf");

    let bold = TextStyle {
        font: bold_font,
        font_size: 60.0,
        color: Color::WHITE,
    };

    let regular = TextStyle {
        font: regular_font,
        font_size: 60.0,
        color: Color::WHITE,
    };

    let text_alignment = TextAlignment::default();

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let mut text_sample = String::new();

    commands.spawn_bundle(Editor::default()); //overflow

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section(text_sample.clone(), regular.clone(), text_alignment),
            ..Default::default()
        })
        .insert(AnimateTranslation);

    commands.insert_resource(Keymap::Dvorak);
}

fn animate_translation(
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
    // println!("input_update begin",);
    //let instant = std::time::Instant::now();

    let mut controller = controller.single_mut().unwrap();
    let time = time.seconds_since_startup();

    for ki in input.iter() {
        let KeyboardInput {
            scan_code: sc,
            key_code: _,
            state,
        } = ki;
        if state == &bevy::input::ElementState::Pressed {
            let prev = controller.press(sc, time);

            if let Some(duration) =
                controller.get_pressed_duration(57, time)
            {
                if sc == &36 {
                    if controller.is_pressed(39){ //S serves as mod key here
                        evwc.send(Command::MoveCursorLeftward);
                    } else {
                        evwc.send(Command::MoveCursorDownward);
                    }

                    // if duration < 0.5{
                    //     println!("moving cursor left...");
                    //     evwc.send(Command::MoveCursorLeftward);
                    // } else {
                    //     println!("moving cursor backward...");
                    //     evwc.send(Command::MoveCursorBackward);
                    // }
                } else if sc == &37 {

                    if controller.is_pressed(39){
                        evwc.send(Command::MoveCursorRightward)
                    } else{
                        evwc.send(Command::MoveCursorUpward)
                    }

                    // if duration < 0.5 {
                    //     println!("moving cursor right...");
                    //     evwc.send(Command::MoveCursorRightward);
                    // } else {
                    //     println!("moving cursor forward...");
                    //     evwc.send(Command::MoveCursorForward);
                    // }
                } else if sc == &28 {
                    println!("adding a line before...");
                    evwc.send(Command::NewLineBefore);
                }
            } else if sc == &58 {
                //Caps
                keymap.switch();
            } else if sc == &14 {
                //Backspace
                evwc.send(Command::RemoveUnderCursor)
            } else if sc == &28 {
                println!("adding a line after...");
                evwc.send(Command::NewLineAfter)
            } else {
                let mut ch = keymap.convert(*sc);
                controller
                    .is_pressed(42)
                    .then(|| ch = ch.to_uppercase().next().unwrap());
                evwc.send(Command::PutCharAfterCursor(ch));
            }
        } else {
            if let Some(duration) = controller.release(sc, time){
                println!("duration: {:.3}", duration);
                if sc == &57 && duration < 0.2{
                    evwc.send(Command::PutCharAfterCursor(' '));
                }
            }
        }
    }
    // controller.print_dbg();

    // let el = instant.elapsed().as_nanos();
    // println!("input_update end: {}ns", el);
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
    // .lines()
    // .iter()
    // .map(|(str, _)| str.clone())
    // .collect::<Vec<String>>()
    // .concat();
}
