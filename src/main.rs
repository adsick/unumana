use unumana::*;

fn main() {
    //todo run frontend after backend
    App::new()
        .insert_resource(WindowDescriptor {
            width: 1280.0,
            height: 720.0,
            position: WindowPosition::Automatic,
            title: "unumana".to_string(),
            present_mode: bevy::window::PresentMode::Fifo, // note, use Immediate or Mailbox for more smooth (and cpu intense) experience
            decorations: true,
            cursor_visible: true,
            mode: bevy::window::WindowMode::Windowed,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_system(input_system.label("input"))
        .add_system(backend_update.label("backend")) //
        .add_system(scroll_system.label("frontend"))
        .add_system(frontend_update.label("frontend").after("backend"))
        .add_system(debug_system.after("backend"))
        .add_system(status_line_system.after("backend"))
        .add_event::<Command>()
        .run();
}

fn scroll_system(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Scrollable>>,
    controller: Query<&Controller>,
) {
    let controller = controller.single();

    let vert = controller.is_pressed(103) as i32 - controller.is_pressed(108) as i32;
    let horiz = controller.is_pressed(106) as i32 - controller.is_pressed(105) as i32;

    for mut transform in query.iter_mut() {
        transform.translation.y += 500.0 * time.delta_seconds() * vert as f32;
        transform.translation.x += 500.0 * time.delta_seconds() * horiz as f32;
        // dbg!(transform.translation);
    }
}

fn backend_update(
    // mut commands: Commands,
    mut backend: Query<&mut Backend>,
    mut evrc: EventReader<Command>,
) {
    // what if we somehow take ownership of the backend,
    // mutate it with async tasks and then spawn it again?
    // let (entity, mut backend) = backend.single_mut().unwrap();
    // commands.entity(*entity).despawn();

    let mut backend = backend.single_mut();
    for c in evrc.iter() {
        backend.execute(c);
    }
}

fn frontend_update(
    mut frontend: Query<&mut Text, With<Content>>,
    backend: Query<&Backend, Changed<Backend>>,
) {
    if backend.is_empty() {
        return;
    }
    let backend = backend.single();
    frontend.single_mut().sections.first_mut().unwrap().value = backend.render();
}

fn debug_system(controller: Query<&Controller, Changed<Controller>>) {
    if let Ok(controller) = controller.get_single() {
        controller.print_dbg();
    }
}

#[cfg(debug_assertions)]
fn status_line_system(
    mut status_line: Query<&mut Text, With<StatusLine>>,
    time: Res<Time>,
    controller: Query<&Controller>,
    backend: Query<&Backend>,
    keymap: Res<KeymapList>,
) {
    if let Ok(controller) = controller.get_single() {
        status_line.single_mut().sections[0].value = format!(
            "{} mode: {:?}, keymap: {}, DBG, time: {:.3}\n",
            backend.single().position(),
            controller.mode,
            keymap.name(),
            time.seconds_since_startup()
        );
    }
}

#[cfg(not(debug_assertions))]
fn status_line_system(
    mut status_line: Query<&mut Text, With<StatusLine>>,
    time: Res<Time>,
    controller: Query<&Controller>,
    backend: Query<&Backend>,
    keymap: Res<KeymapList>,
) {
    if let Ok(controller) = controller.get_single() {
        status_line.single_mut().sections[0].value = format!(
            "{} mode: {:?}, keymap: {}, time: {:.3}\n",
            backend.single().position(),
            controller.mode,
            keymap.name(),
            time.seconds_since_startup()
        );
    }
}
