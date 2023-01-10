use bevy::prelude::*;
// use bevy_mouse_position::MousePosition;
use bevy_mouse_position::{MousePosition, MousePositionSystems};

pub struct SelectAreaPlugin;

#[derive(Component, Default, Debug)]
struct RegionState {
    start_ui: Vec2,
    start_world: Vec2,
    end_ui: Vec2,
    end_world: Vec2,
}

#[derive(Component)]
struct Region;

#[derive(Debug)]
struct ConfirmationEvent {
    start: Vec2,
    end: Vec2,
}

impl Plugin for SelectAreaPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_tracking_region)
            .add_event::<ConfirmationEvent>()
            .add_system(
                track_region
                    .label("track_region")
                    .after(MousePositionSystems::Track),
            )
            .add_system(draw_region.after("track_region"))
            .add_system(capture_region_confirmation.after("track_region"));
    }
}

fn setup_tracking_region(mut commands: Commands) {
    commands.spawn((
        Region,
        RegionState::default(),
        NodeBundle {
            style: Style {
                size: Size::new(Val::Px(0.0), Val::Px(0.0)),
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: Color::rgba_u8(255, 255, 255, 10).into(),
            visibility: Visibility { is_visible: false },
            ..default()
        },
    ));
}

fn track_region(
    buttons: Res<Input<MouseButton>>,
    mouse_position: Res<MousePosition>,
    mut query: Query<(&mut RegionState, &mut Visibility), With<Region>>,
    mut ev_confirmation: EventWriter<ConfirmationEvent>,
) {
    let (mut region, mut visible) = query.single_mut();

    if buttons.just_pressed(MouseButton::Right) {
        region.start_ui = mouse_position.cursor_ui;
        region.end_ui = mouse_position.cursor_ui;
        region.start_world = mouse_position.world;
        region.end_world = mouse_position.world;
        visible.is_visible = true;
    }

    if buttons.just_released(MouseButton::Right) {
        ev_confirmation.send(ConfirmationEvent {
            start: region.start_world,
            end: region.end_world,
        });

        region.start_ui = Vec2::default();
        region.end_ui = Vec2::default();
        region.start_world = Vec2::default();
        region.end_world = Vec2::default();
        visible.is_visible = false;
    }

    if buttons.pressed(MouseButton::Right) {
        region.end_ui = mouse_position.cursor_ui;
        region.end_world = mouse_position.world;
    }
}

fn draw_region(mut query: Query<(&RegionState, &mut Style), With<Region>>) {
    let (region, mut style) = query.single_mut();

    let start: Vec2 = region.start_ui;
    let end: Vec2 = region.end_ui;

    let width = end.x - start.x;
    let height = end.y - start.y;

    style.position.left = Val::Px(start.x.min(end.x));
    style.position.top = Val::Px(start.y.min(end.y));
    style.size = Size::new(Val::Px(width.abs()), Val::Px(height.abs()));
}

fn capture_region_confirmation(mut ev_confirmation: EventReader<ConfirmationEvent>) {
    for ev in ev_confirmation.iter() {
        eprintln!("Event {:?}", ev);
    }
}
