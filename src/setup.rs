use bevy::math::vec3;
use bevy_prototype_lyon::shapes::Rectangle;

use crate::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    //let bold_font = asset_server.load("terminal_land/TerminalLandMono-Bold.otf");
    let regular_font = asset_server.load("writer/Writer-Regular.ttf");

    let regular = TextStyle {
        font: regular_font,
        font_size: 40.0,
        color: Color::WHITE,
    };

    commands.spawn(Camera2dBundle::default());

    commands.spawn(Editor::default());

    commands
        .spawn(Text2dBundle {
            text: Text::from_section("".to_string(), regular.clone()),
            ..default()
        })
        .insert(Scrollable)
        .insert(Content);

    // commands.spawn(UiCameraBundle::default());

    commands
        .spawn(TextBundle {
            text: Text::from_section("".to_string(), regular.clone()),

            ..default()
        })
        .insert(StatusLine);

    //commands.insert_resource(Keymap::Dvorak);

    commands.insert_resource(generic_keymap::KeymapList::new());

    let rect = Rectangle {
        extents: Vec2::new(10.0, 17.0),
        origin: RectangleOrigin::Center,
    };

    let geometry = GeometryBuilder::new().add(&rect);

    let draw_mode = DrawMode::Fill(FillMode::color(Color::CRIMSON));
    let transform = Transform::from_translation(Vec3::new(200.0, 0.0, 0.0));
    commands
        .spawn(geometry.build(draw_mode, transform))
        .insert(Scrollable)
        .with_children(|parent| {
            let circle = shapes::Circle {
                radius: 20.0,
                center: Vec2::new(50.0, -30.0),
            };

            let geometry = GeometryBuilder::new().add(&circle);

            let draw_mode = DrawMode::Fill(FillMode::color(Color::SEA_GREEN));
            let transform = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0));
            parent.spawn(geometry.build(draw_mode, transform));
        });

    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    let mut tr = Transform::default();
    tr.translation += vec3(0.0, 0.0, 1.0);

    commands.spawn(GeometryBuilder::build_as(
        &shape,
        DrawMode::Outlined {
            fill_mode: FillMode::color(Color::CYAN),
            outline_mode: StrokeMode::new(Color::BLACK, 10.0),
        },
        tr,
    ));
}
