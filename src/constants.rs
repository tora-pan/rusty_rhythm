// Musical timing constants (60 BPM for testing - slower tempo)
pub const BPM: f32 = 60.0; // Slowed down from 120.0 for testing
pub const BEAT_INTERVAL: f32 = 60.0 / BPM; // 1.0 seconds per beat at 60 BPM

// Note durations in seconds
pub const WHOLE_NOTE: f32 = BEAT_INTERVAL * 4.0; // 2.0 seconds
pub const HALF_NOTE: f32 = BEAT_INTERVAL * 2.0;  // 1.0 seconds
pub const QUARTER_NOTE: f32 = BEAT_INTERVAL;     // 0.5 seconds
pub const EIGHTH_NOTE: f32 = BEAT_INTERVAL / 2.0; // 0.25 seconds
pub const SIXTEENTH_NOTE: f32 = BEAT_INTERVAL / 4.0; // 0.125 seconds
pub const TRIPLET_NOTE: f32 = BEAT_INTERVAL * 2.0 / 3.0; // 0.333 seconds (triplet eighth)

// Game constants
pub const WINDOW_WIDTH: f32 = 800.0;
pub const WINDOW_HEIGHT: f32 = 600.0;

pub const LANES: [f32; 4] = [-150.0, -50.0, 50.0, 150.0];
pub const NOTE_SIZE: f32 = 40.0;
pub const TARGET_Y: f32 = -200.0;
pub const NOTE_SPEED: f32 = 150.0; // Faster than 50, but not as fast as original 200
pub const HIT_TOLERANCE: f32 = 0.15; // 150ms tolerance

// Travel time calculation - recalculated for exact timing
pub const SPAWN_Y: f32 = WINDOW_HEIGHT/2.0 + 100.0;
pub const TRAVEL_DISTANCE: f32 = SPAWN_Y - TARGET_Y; // Distance from spawn to target
// Calculate exact travel time: 400 - (-200) = 600 pixels, 600/150 = 4.0 seconds exactly
pub const TRAVEL_TIME: f32 = 4.0; // Back to calculated value for precision

// Scoring constants
pub const PERFECT_SCORE: i32 = 100;
pub const GOOD_SCORE: i32 = 50;
pub const OKAY_SCORE: i32 = 20;

pub const PERFECT_TOLERANCE: f32 = 0.05; // 50ms for perfect
pub const GOOD_TOLERANCE: f32 = 0.10;    // 100ms for good
pub const OKAY_TOLERANCE: f32 = HIT_TOLERANCE; // 150ms for okay

// UI constants
pub const SCORE_FONT_SIZE: f32 = 30.0;
pub const COMBO_FONT_SIZE: f32 = 20.0;