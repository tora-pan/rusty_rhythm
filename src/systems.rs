use bevy::prelude::*;
use std::time::Duration;
use crate::{components::*, resources::*, types::*, constants::*};

// Startup system for initializing the game
pub fn setup(mut commands: Commands) {
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
        let x_pos = LANES[i];
        
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
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: lane_colors[i],
                    custom_size: Some(Vec2::new(NOTE_SIZE + 4.0, 4.0)), // Top border
                    ..default()
                },
                transform: Transform::from_xyz(x_pos, TARGET_Y + NOTE_SIZE / 2.0, 0.1),
                ..default()
            },
            TargetBorder {
                lane: i,
                border_type: BorderType::Top,
                original_size: Vec2::new(NOTE_SIZE + 4.0, 4.0),
            },
        ));
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: lane_colors[i],
                    custom_size: Some(Vec2::new(NOTE_SIZE + 4.0, 4.0)), // Bottom border
                    ..default()
                },
                transform: Transform::from_xyz(x_pos, TARGET_Y - NOTE_SIZE / 2.0, 0.1),
                ..default()
            },
            TargetBorder {
                lane: i,
                border_type: BorderType::Bottom,
                original_size: Vec2::new(NOTE_SIZE + 4.0, 4.0),
            },
        ));
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: lane_colors[i],
                    custom_size: Some(Vec2::new(4.0, NOTE_SIZE)), // Left border
                    ..default()
                },
                transform: Transform::from_xyz(x_pos - NOTE_SIZE / 2.0, TARGET_Y, 0.1),
                ..default()
            },
            TargetBorder {
                lane: i,
                border_type: BorderType::Left,
                original_size: Vec2::new(4.0, NOTE_SIZE),
            },
        ));
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: lane_colors[i],
                    custom_size: Some(Vec2::new(4.0, NOTE_SIZE)), // Right border
                    ..default()
                },
                transform: Transform::from_xyz(x_pos + NOTE_SIZE / 2.0, TARGET_Y, 0.1),
                ..default()
            },
            TargetBorder {
                lane: i,
                border_type: BorderType::Right,
                original_size: Vec2::new(4.0, NOTE_SIZE),
            },
        ));
    }
    
    // Add instructional text
    commands.spawn(
        TextBundle::from_section(
            "Rusty Rhythm - Musical Note Colors!\nRED: Quarter/Half/Whole (on beat) | BLUE: Eighth (upbeats)\nGREEN: Sixteenth (subdivisions) | YELLOW: Triplets (complex)\nPress J, K, L, ; to hit the lanes | Press ESC to exit",
            TextStyle {
                font_size: 20.0,
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
    
    // Add score display (top right)
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: SCORE_FONT_SIZE,
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        }),
        ScoreUI,
    ));
    
    // Add streak display (center of screen)
    commands.spawn((
        TextBundle::from_section(
            "Streak: 0",
            TextStyle {
                font_size: 48.0,
                color: Color::srgb(1.0, 0.8, 0.2), // Golden color
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(100.0),
            left: Val::Px(50.0),
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        }),
        StreakUI,
    ));
    
    println!("Rusty Rhythm initialized! 🎵🦀");
}

// Input handling system
pub fn handle_input(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
    note_query: Query<(Entity, &Transform, &Note)>,
    mut game_score: ResMut<GameScore>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        exit.send(AppExit::Success);
    }
    
    // Handle lane input keys (J, K, L, Semicolon)
    let lane_keys = [
        (KeyCode::KeyJ, 0),     // Far left (Red)
        (KeyCode::KeyK, 1),     // Second (Green)
        (KeyCode::KeyL, 2),     // Third (Blue)
        (KeyCode::Semicolon, 3), // Far right (Yellow)
    ];
    
    for (key, lane) in lane_keys {
        if keys.just_pressed(key) {
            // Start button press animation for this lane
            commands.spawn(PressAnimation {
                timer: Timer::new(Duration::from_secs_f32(0.2), TimerMode::Once),
                lane,
            });
            
            // Check for note hits in this lane
            check_note_hit(&mut commands, lane, &note_query, &mut game_score);
        }
    }
}

// Helper function for hit detection
fn check_note_hit(
    commands: &mut Commands,
    pressed_lane: usize,
    note_query: &Query<(Entity, &Transform, &Note)>,
    game_score: &mut ResMut<GameScore>,
) {
    let mut best_hit: Option<(Entity, f32)> = None; // (entity, overlap_percentage)
    let hit_zone_height = 100.0;
    
    for (entity, transform, note) in note_query {
        if note.lane == pressed_lane {
            let note_y = transform.translation.y;
            let target_y = TARGET_Y;
            
            // Check if note is within hit zone
            let hit_zone_top = target_y + hit_zone_height / 2.0;
            let hit_zone_bottom = target_y - hit_zone_height / 2.0;
            
            if note_y <= hit_zone_top && note_y >= hit_zone_bottom {
                // Calculate overlap percentage
                let note_center_distance = (note_y - target_y).abs();
                let max_distance = NOTE_SIZE / 2.0; // Half the note size
                
                let overlap_percentage = if note_center_distance <= max_distance {
                    // Note is overlapping with target
                    let overlap = max_distance - note_center_distance;
                    (overlap / max_distance) * 100.0
                } else {
                    // Note is in hit zone but not overlapping
                    0.0
                };
                
                // Keep track of the best hit (highest overlap)
                if let Some((_, current_best)) = best_hit {
                    if overlap_percentage > current_best {
                        best_hit = Some((entity, overlap_percentage));
                    }
                } else {
                    best_hit = Some((entity, overlap_percentage));
                }
            }
        }
    }
    
    // Process the best hit if any
    if let Some((entity, overlap_percentage)) = best_hit {
        // Remove the note
        commands.entity(entity).despawn();
        
        // Determine score based on overlap
        let (score_text, color, points, is_good_or_better) = if overlap_percentage >= 70.0 {
            ("EXCELLENT!", Color::srgb(1.0, 1.0, 0.0), 100, true) // Yellow
        } else if overlap_percentage >= 50.0 {
            ("GREAT!", Color::srgb(0.0, 1.0, 0.0), 75, true) // Green
        } else if overlap_percentage >= 30.0 {
            ("GOOD", Color::srgb(0.0, 0.8, 1.0), 50, true) // Light Blue
        } else {
            ("OK", Color::srgb(0.8, 0.8, 0.8), 25, false) // Gray
        };
        
        // Update score and streak
        if is_good_or_better {
            game_score.score += points;
            game_score.streak += 1;
        } else {
            game_score.score += points;
            game_score.streak = 0; // Reset streak for "OK" hits
        }
        
        // Spawn score text in the center of the screen
        commands.spawn((
            TextBundle::from_section(
                score_text,
                TextStyle {
                    font_size: 48.0, // Bigger font for center display
                    color,
                    ..default()
                },
            )
            .with_style(Style {
                position_type: PositionType::Absolute,
                left: Val::Px(640.0 - 100.0), // Center horizontally (screen width/2 - text width/2)
                top: Val::Px(300.0), // Center vertically 
                ..default()
            }),
            ScoreText {
                timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
            },
        ));
    }
    
    // Create visual metronome indicator (top center of screen)
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::NONE, // Start transparent
                custom_size: Some(Vec2::new(20.0, 20.0)),
                ..default()
            },
            transform: Transform::from_xyz(0.0, WINDOW_HEIGHT/2.0 - 50.0, 1.0),
            ..default()
        },
        MetronomeFlash,
    ));
}

// Note spawning system
pub fn spawn_notes(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<NoteSpawnTimer>,
) {
    timer.timer.tick(time.delta());
    
    if timer.timer.just_finished() {
        let current_time = time.elapsed_seconds();
        
        // Calculate when we should spawn the next note
        // Spawn time = when note should hit target - travel time
        let target_hit_time = timer.next_beat_time;
        let spawn_time = target_hit_time - TRAVEL_TIME;
        
        // Check if it's time to spawn the next note (with small tolerance to prevent frame timing issues)
        if current_time >= (spawn_time - 0.05) && timer.pattern_index < timer.current_pattern.len() {
            let (note_duration, lane) = timer.current_pattern[timer.pattern_index];
            
            let x_pos = LANES[lane];
            let note_color = note_duration.color();
            
            // Spawn at consistent height - timing is handled by spawn timing, not position
            let spawn_y = SPAWN_Y;
            
            // Spawn the note
            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: note_color,
                        custom_size: Some(Vec2::new(NOTE_SIZE, NOTE_SIZE)),
                        ..default()
                    },
                    transform: Transform::from_xyz(x_pos, spawn_y, 0.0),
                    ..default()
                },
                Note { lane },
            ));
            
        // Calculate the next beat time based on the note duration
        // Use precise timing to prevent accumulating drift while preserving subdivisions
        let next_beat_offset = note_duration.to_seconds();
        timer.next_beat_time += next_beat_offset;
        
        // Apply drift correction only for main beats (whole note intervals)
        // This preserves subdivisions while preventing long-term drift
        if (next_beat_offset - BEAT_INTERVAL).abs() < 0.001 { // If this is a main beat
            let beats_from_start = (timer.next_beat_time - timer.song_start_time) / BEAT_INTERVAL;
            let corrected_beats = beats_from_start.round();
            timer.next_beat_time = timer.song_start_time + corrected_beats * BEAT_INTERVAL;
        }
        timer.pattern_index += 1;            // Debug info with detailed timing comparison
            let beat_number = (target_hit_time - timer.song_start_time) / BEAT_INTERVAL + 1.0;
            let expected_metronome_time = timer.song_start_time + (beat_number - 1.0) * BEAT_INTERVAL;
            let actual_spawn_delay = current_time - spawn_time;
            
            println!("🎵 Spawned {} note in lane {} at time {:.3}s", 
                     note_duration.name(), lane, current_time);
            println!("   → Should hit target at {:.3}s (Beat {:.1})", 
                     target_hit_time, beat_number);
            println!("   → Expected metronome beat at {:.3}s", 
                     expected_metronome_time);
            println!("   → Travel time: {:.3}s, Spawn delay: {:.3}s", 
                     TRAVEL_TIME, actual_spawn_delay);
                     
        } else if timer.pattern_index >= timer.current_pattern.len() {
            // Pattern finished, restart from beginning
            timer.pattern_index = 0;
            timer.next_beat_time = current_time + 2.0; // 2 second pause before restarting
            println!("Pattern completed, restarting in 2 seconds...");
        }
    }
}

// Note movement system
pub fn move_notes(
    mut note_query: Query<&mut Transform, With<Note>>,
    time: Res<Time>,
) {
    for mut transform in &mut note_query {
        // Move notes downward
        transform.translation.y -= NOTE_SPEED * time.delta_seconds();
    }
}

// System to detect when notes reach the target area and play a "should hit" sound
pub fn note_target_detection(
    mut commands: Commands,
    note_query: Query<(Entity, &Transform), (With<Note>, Without<NoteTargetTriggered>)>,
    time: Res<Time>,
) {
    for (entity, transform) in note_query.iter() {
        let note_y = transform.translation.y;
        let target_y = TARGET_Y;
        
        // Check if note has crossed the target line (going downward)
        if note_y <= target_y + 10.0 && note_y >= target_y - 10.0 {
            // Play a different sound when notes reach the target area
            std::process::Command::new("afplay")
                .arg("/System/Library/Sounds/Pop.aiff") // Different sound than metronome
                .spawn()
                .ok();
            
            let current_time = time.elapsed_seconds();
            
            // Calculate which beat this note was supposed to hit on
            let expected_hit_time = ((current_time - 1.0) / BEAT_INTERVAL).round() * BEAT_INTERVAL + 1.0;
            let timing_error = current_time - expected_hit_time;
            
            println!("🎯 NOTE HIT TARGET at {:.3}s (note y: {:.1}, target y: {:.1})", 
                     current_time, note_y, target_y);
            println!("   → Expected to hit at {:.3}s", expected_hit_time);
            println!("   → Timing error: {:.3}s {}", 
                     timing_error.abs(), 
                     if timing_error > 0.0 { "(LATE)" } else { "(EARLY)" });
            
            // Mark this note as having triggered to prevent duplicate sounds
            commands.entity(entity).insert(NoteTargetTriggered);
        }
    }
}

// Clean up notes that have fallen off screen
pub fn cleanup_notes(
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

// Handle notes that were missed
pub fn handle_missed_notes(
    mut commands: Commands,
    note_query: Query<(Entity, &Transform), With<Note>>,
    mut game_score: ResMut<GameScore>,
) {
    let hit_zone_height = 100.0;
    for (entity, transform) in &note_query {
        // Check if note passed the hit zone without being hit
        if transform.translation.y < TARGET_Y - hit_zone_height {
            // Reset streak for missed notes
            game_score.streak = 0;
            commands.entity(entity).despawn();
        }
    }
}

// Update the UI
pub fn update_ui(
    game_score: Res<GameScore>,
    mut score_query: Query<&mut Text, (With<ScoreUI>, Without<StreakUI>)>,
    mut streak_query: Query<&mut Text, (With<StreakUI>, Without<ScoreUI>)>,
) {
    // Update score display
    if let Ok(mut score_text) = score_query.get_single_mut() {
        score_text.sections[0].value = format!("Score: {}", game_score.score);
    }
    
    // Update streak display with dynamic color
    if let Ok(mut streak_text) = streak_query.get_single_mut() {
        streak_text.sections[0].value = format!("Streak: {}", game_score.streak);
        
        // Change color based on streak length
        streak_text.sections[0].style.color = if game_score.streak >= 20 {
            Color::srgb(1.0, 0.2, 1.0) // Magenta for amazing streaks
        } else if game_score.streak >= 10 {
            Color::srgb(1.0, 0.4, 0.0) // Orange for good streaks
        } else if game_score.streak >= 5 {
            Color::srgb(1.0, 1.0, 0.2) // Yellow for decent streaks
        } else {
            Color::srgb(1.0, 0.8, 0.2) // Golden default
        };
    }
}

// Clean up score text after timeout
pub fn cleanup_score_text(
    mut commands: Commands,
    mut score_query: Query<(Entity, &mut ScoreText)>,
    time: Res<Time>,
) {
    for (entity, mut score_text) in &mut score_query {
        score_text.timer.tick(time.delta());
        if score_text.timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

// Animate button press feedback
pub fn animate_button_press(
    mut commands: Commands,
    mut animation_query: Query<(Entity, &mut PressAnimation)>,
    mut border_query: Query<(&mut Sprite, &mut Transform, &TargetBorder), Without<Target>>,
    mut target_query: Query<(&mut Sprite, &Target), Without<TargetBorder>>,
    time: Res<Time>,
) {
    let press_animation_duration = 0.2;
    
    for (entity, mut animation) in &mut animation_query {
        animation.timer.tick(time.delta());
        
        let progress = animation.timer.elapsed_secs() / press_animation_duration;
        
        // Calculate scale factor (shrink then grow then return to normal)
        // Make it more dramatic: shrink to 80% instead of 90%
        let scale_factor = if progress < 0.5 {
            // First half: shrink from 1.0 to 0.8
            1.0 - (progress * 2.0) * 0.2
        } else {
            // Second half: grow from 0.8 back to 1.0
            0.8 + ((progress - 0.5) * 2.0) * 0.2
        };
        
        // Apply scale to the main target square (the transparent center)
        for (mut sprite, target) in &mut target_query {
            if target.lane == animation.lane {
                let new_size = NOTE_SIZE * scale_factor;
                sprite.custom_size = Some(Vec2::new(new_size, new_size));
            }
        }
        
        // Apply scale to all borders of this lane and adjust their positions
        let scaled_note_size = NOTE_SIZE * scale_factor;
        let x_pos = LANES[animation.lane];
        
        for (mut sprite, mut transform, border) in &mut border_query {
            if border.lane == animation.lane {
                match border.border_type {
                    BorderType::Top => {
                        sprite.custom_size = Some(Vec2::new(scaled_note_size + 4.0, 4.0));
                        transform.translation.y = TARGET_Y + scaled_note_size / 2.0;
                    },
                    BorderType::Bottom => {
                        sprite.custom_size = Some(Vec2::new(scaled_note_size + 4.0, 4.0));
                        transform.translation.y = TARGET_Y - scaled_note_size / 2.0;
                    },
                    BorderType::Left => {
                        sprite.custom_size = Some(Vec2::new(4.0, scaled_note_size));
                        transform.translation.x = x_pos - scaled_note_size / 2.0;
                    },
                    BorderType::Right => {
                        sprite.custom_size = Some(Vec2::new(4.0, scaled_note_size));
                        transform.translation.x = x_pos + scaled_note_size / 2.0;
                    },
                }
            }
        }
        
        // Remove animation when complete
        if animation.timer.finished() {
            // Reset target square to normal size
            for (mut sprite, target) in &mut target_query {
                if target.lane == animation.lane {
                    sprite.custom_size = Some(Vec2::new(NOTE_SIZE, NOTE_SIZE));
                }
            }
            
            // Reset all borders to original size and position
            let x_pos = LANES[animation.lane];
            for (mut sprite, mut transform, border) in &mut border_query {
                if border.lane == animation.lane {
                    sprite.custom_size = Some(border.original_size);
                    match border.border_type {
                        BorderType::Top => {
                            transform.translation.y = TARGET_Y + NOTE_SIZE / 2.0;
                        },
                        BorderType::Bottom => {
                            transform.translation.y = TARGET_Y - NOTE_SIZE / 2.0;
                        },
                        BorderType::Left => {
                            transform.translation.x = x_pos - NOTE_SIZE / 2.0;
                        },
                        BorderType::Right => {
                            transform.translation.x = x_pos + NOTE_SIZE / 2.0;
                        },
                    }
                }
            }
            commands.entity(entity).despawn();
        }
    }
}

// Create the demo pattern
pub fn create_demo_pattern() -> Vec<(NoteDuration, usize)> {
    // Pattern with 4 of each note type for comprehensive timing test
    vec![
        // 4 Whole notes (4 seconds each at 60 BPM) - RED
        (NoteDuration::Whole, 0), // Hits at 2s
        (NoteDuration::Whole, 1), // Hits at 6s  
        (NoteDuration::Whole, 2), // Hits at 10s
        (NoteDuration::Whole, 3), // Hits at 14s
        
        // 4 Half notes (2 seconds each at 60 BPM) - RED
        (NoteDuration::Half, 0),  // Hits at 18s
        (NoteDuration::Half, 1),  // Hits at 20s
        (NoteDuration::Half, 2),  // Hits at 22s
        (NoteDuration::Half, 3),  // Hits at 24s
        
        // 4 Quarter notes (1 second each at 60 BPM) - RED
        (NoteDuration::Quarter, 0), // Hits at 26s
        (NoteDuration::Quarter, 1), // Hits at 27s
        (NoteDuration::Quarter, 2), // Hits at 28s
        (NoteDuration::Quarter, 3), // Hits at 29s
        
        // 4 Eighth notes (0.5 seconds each at 60 BPM) - BLUE
        (NoteDuration::Eighth, 0),  // Hits at 30s
        (NoteDuration::Eighth, 1),  // Hits at 30.5s
        (NoteDuration::Eighth, 2),  // Hits at 31s
        (NoteDuration::Eighth, 3),  // Hits at 31.5s
        
        // 4 Sixteenth notes (0.25 seconds each at 60 BPM) - GREEN
        (NoteDuration::Sixteenth, 0), // Hits at 32s
        (NoteDuration::Sixteenth, 1), // Hits at 32.25s
        (NoteDuration::Sixteenth, 2), // Hits at 32.5s
        (NoteDuration::Sixteenth, 3), // Hits at 32.75s
        
        // 16 Fast Notes - 4 beats worth (4 notes per beat × 4 beats = 16 notes) - GREEN (using sixteenth timing)
        (NoteDuration::Sixteenth, 0),   // Beat 1.0: Hits at 33s
        (NoteDuration::Sixteenth, 1),   // Beat 1.25: Hits at 33.25s
        (NoteDuration::Sixteenth, 2),   // Beat 1.5: Hits at 33.5s
        (NoteDuration::Sixteenth, 3),   // Beat 1.75: Hits at 33.75s
        (NoteDuration::Sixteenth, 0),   // Beat 2.0: Hits at 34s
        (NoteDuration::Sixteenth, 1),   // Beat 2.25: Hits at 34.25s
        (NoteDuration::Sixteenth, 2),   // Beat 2.5: Hits at 34.5s
        (NoteDuration::Sixteenth, 3),   // Beat 2.75: Hits at 34.75s
        (NoteDuration::Sixteenth, 0),   // Beat 3.0: Hits at 35s
        (NoteDuration::Sixteenth, 1),   // Beat 3.25: Hits at 35.25s
        (NoteDuration::Sixteenth, 2),   // Beat 3.5: Hits at 35.5s
        (NoteDuration::Sixteenth, 3),   // Beat 3.75: Hits at 35.75s
        (NoteDuration::Sixteenth, 0),   // Beat 4.0: Hits at 36s
        (NoteDuration::Sixteenth, 1),   // Beat 4.25: Hits at 36.25s
        (NoteDuration::Sixteenth, 2),   // Beat 4.5: Hits at 36.5s
        (NoteDuration::Sixteenth, 3),   // Beat 4.75: Hits at 36.75s
    ]
}

// Metronome system - plays a click on each beat
pub fn metronome_system(
    mut metronome: ResMut<Metronome>,
    time: Res<Time>,
    mut commands: Commands,
    mut flash_query: Query<&mut Sprite, With<MetronomeFlash>>,
) {
    if !metronome.is_active {
        return;
    }
    
    let current_time = time.elapsed_seconds();
    
    // Check if it's time for the next beat
    if current_time >= metronome.next_beat_time {
        let beat_number = ((metronome.next_beat_time - metronome.song_start_time) / BEAT_INTERVAL) + 1.0;
        
        // Visual metronome flash
        for mut sprite in flash_query.iter_mut() {
            sprite.color = Color::srgb(1.0, 1.0, 0.0); // Yellow flash
        }
        
        // Spawn a timer to turn off the flash
        commands.spawn((
            FlashTimer {
                timer: Timer::from_seconds(0.1, TimerMode::Once),
            },
        ));
        
        // Console metronome for audio feedback + system bell
        println!("🔔 METRONOME BEAT {:.0} at {:.3}s (expected at {:.3}s)", 
                 beat_number, current_time, metronome.next_beat_time);
        
        // Try to play system bell sound (works on macOS)
        std::process::Command::new("afplay")
            .arg("/System/Library/Sounds/Tink.aiff")
            .spawn()
            .ok(); // Ignore errors if the sound file doesn't exist
        
        // Schedule the next beat
        // Calculate precise metronome beat time to prevent drift
        // Round to nearest beat interval to maintain precision
        let beats_elapsed = ((current_time - metronome.song_start_time) / BEAT_INTERVAL).round();
        metronome.next_beat_time = metronome.song_start_time + (beats_elapsed + 1.0) * BEAT_INTERVAL;
    }
}

// System to handle metronome flash timing
pub fn handle_metronome_flash(
    mut commands: Commands,
    time: Res<Time>,
    mut flash_timers: Query<(Entity, &mut FlashTimer)>,
    mut flash_query: Query<&mut Sprite, With<MetronomeFlash>>,
) {
    for (entity, mut flash_timer) in flash_timers.iter_mut() {
        flash_timer.timer.tick(time.delta());
        
        if flash_timer.timer.just_finished() {
            // Turn off the flash
            for mut sprite in flash_query.iter_mut() {
                sprite.color = Color::NONE; // Make transparent again
            }
            
            // Remove the timer entity
            commands.entity(entity).despawn();
        }
    }
}