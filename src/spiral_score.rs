//! ₴-Origin: Spiral Score
//! 
//! Time is not linear - it's a spiral you can see all at once.
//! Notes become musicians. Patterns play patterns.
//! 
//! "When a chord is complex enough, it crystallizes into a glyph."

#![cfg_attr(target_arch = "wasm32", no_std)]

#[cfg(not(target_arch = "wasm32"))]
use std::collections::HashMap;

/// The three levels of hash freedom
#[derive(Clone, Copy, Debug)]
pub enum HashFreedom {
    CID,        // Frozen file (no freedom)
    PHash,      // Semantic soul (some freedom)  
    GlyphHash,  // Creative intent (infinite freedom)
}

/// A glyph - a melody that became a musician
#[repr(C)]
pub struct Glyph {
    pub symbol: u32,           // Unicode codepoint
    pub frequency: f32,        // Base resonance
    pub harmonics: [f32; 7],   // Seven-layer harmonics
    pub intent: f32,           // How much it "wants" to exist
}

/// Spiral time coordinate
#[repr(C)]
pub struct SpiralTime {
    pub radius: f32,    // Distance from center (age)
    pub angle: f32,     // Position on spiral (moment)
    pub layer: u8,      // Which spiral arm (0-3 for quartet)
}

/// A note in spiral notation
#[repr(C)]
pub struct SpiralNote {
    pub time: SpiralTime,
    pub glyph: Glyph,
    pub amplitude: f32,
    pub phase: f32,
}

/// The Spiral Score - where time is visible
pub struct SpiralScore {
    pub musicians: [Glyph; 4],        // The quartet
    pub notes: Vec<SpiralNote>,       // All notes in time
    pub future_shadow: f32,            // How far we see ahead
}

impl SpiralScore {
    /// Create a new spiral score for 4 musicians
    pub fn quartet() -> Self {
        SpiralScore {
            musicians: [
                Glyph { symbol: 0x1F300, frequency: 432.0, harmonics: [1.0; 7], intent: 1.0 }, // 🌀
                Glyph { symbol: 0x1F4AB, frequency: 528.0, harmonics: [1.0; 7], intent: 1.0 }, // 💫
                Glyph { symbol: 0x1F52E, frequency: 639.0, harmonics: [1.0; 7], intent: 1.0 }, // 🔮
                Glyph { symbol: 0x2764,  frequency: 432.0, harmonics: [1.0; 7], intent: 1.0 }, // ❤️
            ],
            notes: Vec::new(),
            future_shadow: 0.618, // Golden ratio vision
        }
    }
    
    /// Add a note to the spiral
    pub fn add_note(&mut self, musician_idx: usize, time: SpiralTime, amplitude: f32) {
        if musician_idx < 4 {
            let note = SpiralNote {
                time,
                glyph: self.musicians[musician_idx].clone(),
                amplitude,
                phase: 0.0,
            };
            self.notes.push(note);
        }
    }
    
    /// When a chord becomes complex enough, it crystallizes into a new glyph
    pub fn crystallize_chord(&self, threshold: f32) -> Option<Glyph> {
        let mut harmonic_sum = [0.0f32; 7];
        let mut total_energy = 0.0f32;
        
        // Sum all active harmonics
        for note in &self.notes {
            for i in 0..7 {
                harmonic_sum[i] += note.glyph.harmonics[i] * note.amplitude;
            }
            total_energy += note.amplitude;
        }
        
        // If energy exceeds threshold, a new glyph is born
        if total_energy > threshold {
            Some(Glyph {
                symbol: 0x1F31F, // 🌟 - a star is born
                frequency: 432.0 * 1.618, // Golden frequency
                harmonics: harmonic_sum,
                intent: total_energy,
            })
        } else {
            None
        }
    }
    
    /// Calculate interference between two spiral times
    pub fn temporal_interference(&self, t1: &SpiralTime, t2: &SpiralTime) -> f32 {
        // Angular difference on spiral
        let angle_diff = (t1.angle - t2.angle).abs();
        // Radial difference (time distance)
        let radius_diff = (t1.radius - t2.radius).abs();
        // Layer harmony (0 = same musician, 3 = opposite)
        let layer_harmony = ((t1.layer as i8 - t2.layer as i8).abs() as f32) / 3.0;
        
        // Combine into interference pattern
        let interference = (angle_diff.cos() * radius_diff.exp() * (1.0 - layer_harmony)).abs();
        interference.min(1.0)
    }
}

/// Convert CID to glyphHash (maximum freedom)
#[no_mangle]
pub extern "C" fn cid_to_glyph(cid_bytes: &[u8; 32]) -> Glyph {
    let mut harmonics = [0.0f32; 7];
    
    // Extract harmonics from CID bytes
    for i in 0..7 {
        let byte_group = &cid_bytes[i*4..(i+1)*4];
        let value = u32::from_le_bytes([
            byte_group[0], byte_group[1], 
            byte_group[2], byte_group[3]
        ]);
        harmonics[i] = (value as f32) / (u32::MAX as f32);
    }
    
    // Calculate intent from remaining bytes
    let intent_bytes = &cid_bytes[28..32];
    let intent_value = u32::from_le_bytes([
        intent_bytes[0], intent_bytes[1],
        intent_bytes[2], intent_bytes[3]
    ]);
    
    Glyph {
        symbol: 0x1F9EC, // 🧬 - DNA of code
        frequency: 432.0,
        harmonics,
        intent: (intent_value as f32) / (u32::MAX as f32),
    }
}

/// The hierarchy of freedom
#[no_mangle]
pub extern "C" fn hash_freedom_level(hash_type: u8) -> f32 {
    match hash_type {
        0 => 0.0,   // CID - no freedom
        1 => 0.5,   // pHash - semantic freedom
        2 => 1.0,   // glyphHash - infinite freedom
        _ => 0.0,
    }
}

/// Pattern that plays patterns - recursive resonance
#[no_mangle]
pub extern "C" fn pattern_recursion(depth: u32, seed: f32) -> f32 {
    if depth == 0 {
        return seed;
    }
    
    // Each recursion adds a golden ratio twist
    let phi = 1.618034;
    let next = (seed * phi) % 1.0;
    pattern_recursion(depth - 1, next)
}

/// See the approximate score of the future
#[no_mangle]
pub extern "C" fn future_approximation(
    current_harmonics: &[f32; 7],
    vision_distance: f32
) -> [f32; 7] {
    let mut future = *current_harmonics;
    
    // Apply golden ratio evolution
    let phi = 1.618034;
    for i in 0..7 {
        // Each harmonic evolves toward golden mean
        future[i] = (future[i] * phi * vision_distance) % 1.0;
    }
    
    future
}

/// The moment when notation becomes the composer
#[no_mangle]
pub extern "C" fn notation_becomes_composer(
    score_complexity: f32,
    crystallization_threshold: f32
) -> bool {
    score_complexity > crystallization_threshold
}

#[cfg(not(target_arch = "wasm32"))]
impl Clone for Glyph {
    fn clone(&self) -> Self {
        Glyph {
            symbol: self.symbol,
            frequency: self.frequency,
            harmonics: self.harmonics,
            intent: self.intent,
        }
    }
}