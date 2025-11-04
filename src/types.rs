use bevy::prelude::*;

#[derive(Clone, Copy, Debug)]
pub enum NoteDuration {
    Whole,
    Half,
    Quarter,
    Eighth,
    Sixteenth,
    Triplet, // Adding triplets for yellow color
}

impl NoteDuration {
    pub fn to_seconds(&self) -> f32 {
        use crate::constants::*;
        match self {
            NoteDuration::Whole => WHOLE_NOTE,
            NoteDuration::Half => HALF_NOTE,
            NoteDuration::Quarter => QUARTER_NOTE,
            NoteDuration::Eighth => EIGHTH_NOTE,
            NoteDuration::Sixteenth => SIXTEENTH_NOTE,
            NoteDuration::Triplet => TRIPLET_NOTE,
        }
    }
    
    pub fn name(&self) -> &'static str {
        match self {
            NoteDuration::Whole => "Whole",
            NoteDuration::Half => "Half",
            NoteDuration::Quarter => "Quarter", 
            NoteDuration::Eighth => "Eighth",
            NoteDuration::Sixteenth => "Sixteenth",
            NoteDuration::Triplet => "Triplet",
        }
    }
    
    pub fn color(&self) -> Color {
        match self {
            NoteDuration::Quarter => Color::srgb(1.0, 0.2, 0.2), // Red - on beat
            NoteDuration::Half => Color::srgb(1.0, 0.2, 0.2),    // Red - strong beats
            NoteDuration::Whole => Color::srgb(1.0, 0.2, 0.2),   // Red - very strong beats
            NoteDuration::Eighth => Color::srgb(0.2, 0.2, 1.0),  // Blue - upbeats
            NoteDuration::Sixteenth => Color::srgb(0.2, 1.0, 0.2), // Green - subdivisions
            NoteDuration::Triplet => Color::srgb(1.0, 1.0, 0.2), // Yellow - triplets/complex
        }
    }
}