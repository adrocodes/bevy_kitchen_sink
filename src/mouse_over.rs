use bevy::prelude::*;
use bevy_mouse_position::WorldPosition;

use crate::bounds::Bounds2;

pub struct MouseOverPlugin;

impl Plugin for MouseOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_square)
            .add_system(hover_square);
    }
}

fn spawn_square(mut commands: Commands) {
    let size = Vec2::new(200.0, 100.0);
    let position = Vec2::new(0.0, 0.0);
    let transform = Transform::from_xyz(position.x, position.y, 0.0);

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                custom_size: Some(size),
                ..default()
            },
            transform,
            ..default()
        },
        Bounds2 { position, size },
    ));
}

fn hover_square(mut query: Query<(&mut Sprite, &Bounds2)>, mouse_position: Res<WorldPosition>) {
    for (mut sprite, bounds) in query.iter_mut() {
        if bounds.in_bounds_centered(mouse_position.0) {
            sprite.color = Color::GREEN;
        } else {
            sprite.color = Color::RED;
        }
    }
}
