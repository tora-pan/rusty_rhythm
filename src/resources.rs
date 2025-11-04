use bevy::prelude::*;
use std::time::Duration;
use crate::types::NoteDuration;

#[derive(Resource)]
pub struct NoteSpawnTimer {
    pub timer: Timer,
    pub current_pattern: Vec<(NoteDuration, usize)>, // (duration, lane) pairs
    pub pattern_index: usize,
    pub next_beat_time: f32, // When the next note should hit the target
    pub song_start_time: f32, // When the song started
}

impl Default for NoteSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::new(Duration::from_secs_f32(0.1), TimerMode::Repeating),
            current_pattern: Vec::new(),
            pattern_index: 0,
            next_beat_time: 2.0, // First note hits at 2 seconds
            song_start_time: 1.0, // Song starts at 1 second (1 second delay)
        }
    }
}

#[derive(Resource)]
pub struct GameScore {
    pub score: u32,
    pub streak: u32,
}

impl Default for GameScore {
    fn default() -> Self {
        Self {
            score: 0,
            streak: 0,
        }
    }
}

#[derive(Resource)]
pub struct Metronome {
    pub next_beat_time: f32,
    pub song_start_time: f32,
    pub is_active: bool,
    pub audio_handle: Option<Handle<bevy::prelude::AudioSource>>,
}