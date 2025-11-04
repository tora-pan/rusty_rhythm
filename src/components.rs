use bevy::prelude::*;

#[derive(Component)]
pub struct Note {
    pub lane: usize,
}

#[derive(Component)]
pub struct NoteTargetTriggered; // Marker to prevent duplicate target sounds

#[derive(Component)]
pub struct Target {
    pub lane: usize,
}

#[derive(Component)]
pub struct TargetBorder {
    pub lane: usize,
    pub border_type: BorderType,
    pub original_size: Vec2,
}

#[derive(Component)]
pub enum BorderType {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Component)]
pub struct PressAnimation {
    pub timer: Timer,
    pub lane: usize,
}

#[derive(Component)]
pub struct ScoreText {
    pub timer: Timer,
}

#[derive(Component)]
pub struct ScoreUI;

#[derive(Component)]
pub struct StreakUI;

#[derive(Component)]
pub struct MetronomeFlash;

#[derive(Component)]
pub struct FlashTimer {
    pub timer: Timer,
}