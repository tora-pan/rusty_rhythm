use bevy::prelude::*;
use std::time::Duration;

const LANE_WIDTH: f32 = 80.0;
const LANE_SPACING: f32 = 100.0;
const NOTE_SIZE: f32 = 60.0;
const TARGET_Y: f32 = -250.0;
const SPAWN_Y: f32 = 400.0;
const NOTE_SPEED: f32 = 200.0;

#[derive(Component)]
struct Note {
    lane: usize,
}

#[derive(Component)]
struct Target {
    lane: usize,
}

#[derive(Resource)]
struct NoteSpawnTimer {
    timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Rhythm".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(NoteSpawnTimer {
            timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Repeating),
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, spawn_notes, move_notes, cleanup_notes))
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn a camera
    commands.spawn(Camera2dBundle::default());
    
    // Define the 4 colors for the lanes
    let lane_colors = [
        Color::srgb(1.0, 0.2, 0.2), // Red
        Color::srgb(0.2, 1.0, 0.2), // Green  
        Color::srgb(0.2, 0.2, 1.0), // Blue
        Color::srgb(1.0, 1.0, 0.2), // Yellow
    ];
    
    // Create 4 target squares (outlined, at the bottom)
    for i in 0..4 {
        let x_pos = (i as f32 - 1.5) * LANE_SPACING;
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::NONE, // Transparent fill
                    custom_size: Some(Vec2::new(NOTE_SIZE, NOTE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(x_pos, TARGET_Y, 0.0),
                ..default()
            },
            Target { lane: i },
        ));
        
        // Create the outline for target squares
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: lane_colors[i],
                custom_size: Some(Vec2::new(NOTE_SIZE + 4.0, 4.0)), // Top border
                ..default()
            },
            transform: Transform::from_xyz(x_pos, TARGET_Y + NOTE_SIZE / 2.0, 0.1),
            ..default()
        });
        
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: lane_colors[i],
                custom_size: Some(Vec2::new(NOTE_SIZE + 4.0, 4.0)), // Bottom border
                ..default()
            },
            transform: Transform::from_xyz(x_pos, TARGET_Y - NOTE_SIZE / 2.0, 0.1),
            ..default()
        });
        
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: lane_colors[i],
                custom_size: Some(Vec2::new(4.0, NOTE_SIZE)), // Left border
                ..default()
            },
            transform: Transform::from_xyz(x_pos - NOTE_SIZE / 2.0, TARGET_Y, 0.1),
            ..default()
        });
        
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: lane_colors[i],
                custom_size: Some(Vec2::new(4.0, NOTE_SIZE)), // Right border
                ..default()
            },
            transform: Transform::from_xyz(x_pos + NOTE_SIZE / 2.0, TARGET_Y, 0.1),
            ..default()
        });
    }
    
    // Add instructional text
    commands.spawn(
        TextBundle::from_section(
            "Rusty Rhythm - Watch the colored squares fall!\nPress ESC to exit",
            TextStyle {
                font_size: 24.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
    
    println!("Rusty Rhythm initialized! 🎵🦀");
}

fn handle_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
}

fn spawn_notes(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<NoteSpawnTimer>,
) {
    timer.timer.tick(time.delta());
    
    if timer.timer.just_finished() {
        // Randomly choose a lane (0-3) using simple time-based randomness
        let lane = (time.elapsed_seconds() * 1000.0) as usize % 4;
        let x_pos = (lane as f32 - 1.5) * LANE_SPACING;
        
        // Define colors for each lane
        let lane_colors = [
            Color::srgb(1.0, 0.2, 0.2), // Red
            Color::srgb(0.2, 1.0, 0.2), // Green  
            Color::srgb(0.2, 0.2, 1.0), // Blue
            Color::srgb(1.0, 1.0, 0.2), // Yellow
        ];
        
        // Spawn a colored note at the top
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: lane_colors[lane],
                    custom_size: Some(Vec2::new(NOTE_SIZE, NOTE_SIZE)),
                    ..default()
                },
                transform: Transform::from_xyz(x_pos, SPAWN_Y, 0.0),
                ..default()
            },
            Note { lane },
        ));
    }
}

fn move_notes(
    mut note_query: Query<&mut Transform, With<Note>>,
    time: Res<Time>,
) {
    for mut transform in &mut note_query {
        // Move notes downward
        transform.translation.y -= NOTE_SPEED * time.delta_seconds();
    }
}

fn cleanup_notes(
    mut commands: Commands,
    note_query: Query<(Entity, &Transform), With<Note>>,
) {
    for (entity, transform) in &note_query {
        // Remove notes that have moved past the bottom of the screen
        if transform.translation.y < -400.0 {
            commands.entity(entity).despawn();
        }
    }
}
