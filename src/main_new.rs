use bevy::prelude::*;
use rusty_rhythm::*;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Rusty Rhythm".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(NoteSpawnTimer {
            timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating), // Check every 0.1 seconds
            lane_counter: 0,
            current_pattern: create_demo_pattern(),
            pattern_index: 0,
            next_spawn_time: 1.0, // Start after 1 second
        })
        .insert_resource(GameScore {
            score: 0,
            streak: 0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (
            handle_input, 
            spawn_notes, 
            move_notes, 
            cleanup_notes, 
            animate_button_press, 
            cleanup_score_text, 
            update_ui, 
            handle_missed_notes
        ))
        .run();
}