use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
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
        .add_plugins(AudioPlugin)
            metronome_system,
            handle_metronome_flash,
        ))elude::*;
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
        .add_plugins(AudioPlugin)
        .insert_resource(NoteSpawnTimer {
            timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating), // Check every 0.1 seconds
            current_pattern: create_demo_pattern(),
            pattern_index: 0,
            next_beat_time: 2.0, // First note hits target at 2 seconds
            song_start_time: 1.0, // Song starts at 1 second
        })
        .insert_resource(GameScore {
            score: 0,
            streak: 0,
        })
        .insert_resource(Metronome {
            next_beat_time: 1.0, // First beat at 1 second
            song_start_time: 1.0,
            is_active: true,
            audio_handle: None, // Will be loaded in setup
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
            handle_missed_notes,
            metronome_system
        ))
        .run();
}
