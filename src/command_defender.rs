use bevy::prelude::*;

pub struct CommandDefenderPlugin;

#[derive(Resource)]
struct CommandInput(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum Systems {
    RecordInput,
}

impl Plugin for CommandDefenderPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CommandInput(String::new()))
            .add_startup_system(setup)
            .add_system(text_input.label(Systems::RecordInput))
            .add_system(render_text_input.after(Systems::RecordInput));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let text_field = TextSection {
        value: "Whats up".to_string(),
        style: TextStyle {
            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
            font_size: 24.,
            color: Color::BLACK,
        },
    };

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Px(40.)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::rgba(0., 0., 0., 0.2).into(),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn(TextBundle {
                text: Text {
                    sections: vec![text_field],
                    ..default()
                },
                ..default()
            });
        });
}

/// prints every char coming in; press enter to echo the full string
fn text_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut command_input: ResMut<CommandInput>,
) {
    if keys.just_pressed(KeyCode::Return) {
        println!("Text input: {}", command_input.0.to_string());
        command_input.0.clear();
    } else if keys.pressed(KeyCode::Back) || keys.just_pressed(KeyCode::Back) {
        command_input.0.pop();
    } else {
        for ev in char_evr.iter() {
            if ev.char.is_alphabetic() || ev.char.is_numeric() || ev.char == ':' {
                command_input.0.push(ev.char);
            }
        }
    }
}

fn render_text_input(command_input: Res<CommandInput>, mut query: Query<&mut Text>) {
    for mut text in query.iter_mut() {
        text.sections[0].value = command_input.0.to_string().to_uppercase();
    }
}
