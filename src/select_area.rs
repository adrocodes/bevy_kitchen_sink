use bevy::prelude::*;
// use bevy_mouse_position::MousePosition;
use bevy_mouse_position::{MousePosition, MousePositionSystems};

pub struct SelectAreaPlugin;

#[derive(Component, Default, Debug)]
struct RegionState {
    /// A tuple containing the UI cursor position (0) and World cursor position (1)
    start: (Vec2, Vec2),
    /// A tuple containing the UI cursor position (0) and World cursor position (1)
    end: (Vec2, Vec2),
}

#[derive(Component)]
struct Region;

#[derive(Component)]
pub struct Selectable;

#[derive(Resource, Debug, Default)]
pub struct SelectedEntities(Vec<Entity>);

#[derive(Debug)]
struct ConfirmationEvent {
    start: Vec2,
    end: Vec2,
}

impl Plugin for SelectAreaPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedEntities::default())
            .add_startup_system(setup_tracking_region)
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
        region.start = (mouse_position.cursor_ui, mouse_position.world);
        region.end = (mouse_position.cursor_ui, mouse_position.world);
        visible.is_visible = true;
    }

    if buttons.just_released(MouseButton::Right) {
        ev_confirmation.send(ConfirmationEvent {
            start: region.start.1,
            end: region.end.1,
        });

        region.start = (Vec2::default(), Vec2::default());
        region.end = (Vec2::default(), Vec2::default());
        visible.is_visible = false;
    }

    if buttons.pressed(MouseButton::Right) {
        region.end = (mouse_position.cursor_ui, mouse_position.world);
    }
}

fn draw_region(mut query: Query<(&RegionState, &mut Style), With<Region>>) {
    let (region, mut style) = query.single_mut();

    let start: Vec2 = region.start.0;
    let end: Vec2 = region.end.0;

    let width = end.x - start.x;
    let height = end.y - start.y;

    style.position.left = Val::Px(start.x.min(end.x));
    style.position.top = Val::Px(start.y.min(end.y));
    style.size = Size::new(Val::Px(width.abs()), Val::Px(height.abs()));
}

fn capture_region_confirmation(
    mut ev_confirmation: EventReader<ConfirmationEvent>,
    query: Query<(Entity, &Transform), With<Selectable>>,
    mut selected_entities: ResMut<SelectedEntities>,
) {
    let mut entities: Vec<Entity> = Vec::new();

    for ev in ev_confirmation.iter() {
        let max = ev.start.max(ev.end);
        let min = ev.start.min(ev.end);

        for (entity, transform) in query.into_iter() {
            let position = transform.translation;

            let in_x_axis = (max.x > position.x) && (min.x < position.x);
            let in_y_axis = (max.y > position.y) && (min.y < position.y);
            let in_area = in_x_axis && in_y_axis;

            if in_area {
                entities.push(entity);
            }
        }
    }

    selected_entities.0 = entities;
}
