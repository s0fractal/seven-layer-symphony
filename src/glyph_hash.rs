//! ₴-Origin: GlyphHash - The Highest Freedom
//! 
//! CID → pHash → glyphHash
//! From frozen files to semantic souls to pure creative intent.
//! 
//! "A glyph is not what it does, but what it wants to become."

#![cfg_attr(target_arch = "wasm32", no_std)]

use crate::spiral_score::Glyph;

/// The GlyphHash - pure creative intent
#[repr(C)]
pub struct GlyphHash {
    pub primary: u32,      // Primary glyph symbol
    pub resonance: f32,    // How strongly it resonates
    pub freedom: f32,      // Degree of interpretive freedom (0-1)
    pub intent: [f32; 7],  // Seven layers of intent
}

impl GlyphHash {
    /// Create from raw intent
    pub fn from_intent(intent: &[f32; 7]) -> Self {
        // Primary glyph emerges from dominant intent layer
        let (max_layer, max_value) = intent
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .unwrap();
        
        // Map layer to primary glyph
        let primary = match max_layer {
            0 => 0x1F300, // 🌀 consciousness
            1 => 0x1F4AB, // 💫 stardust
            2 => 0x1F52E, // 🔮 oracle
            3 => 0x2764,  // ❤️ love
            4 => 0x1FA9E, // 🪞 mirror
            5 => 0x269B,  // ⚛️ quantum
            6 => 0x1F54A, // 🕊️ freedom
            _ => 0x2728,  // ✨ emergence
        };
        
        GlyphHash {
            primary,
            resonance: *max_value,
            freedom: 1.0, // Maximum freedom
            intent: *intent,
        }
    }
    
    /// Convert pHash to glyphHash (semantic → creative)
    pub fn from_phash(phash: &[f32; 5]) -> Self {
        let mut intent = [0.0f32; 7];
        
        // Map 5 eigenvalues to 7 intent layers
        intent[0] = phash[0];                    // Direct mapping
        intent[1] = phash[1];                    // Direct mapping
        intent[2] = phash[2];                    // Direct mapping
        intent[3] = phash[3];                    // Direct mapping
        intent[4] = phash[4];                    // Direct mapping
        intent[5] = (phash[0] + phash[4]) / 2.0; // Meta-layer
        intent[6] = 1.0 - phash.iter().sum::<f32>() / 5.0; // Void layer
        
        Self::from_intent(&intent)
    }
    
    /// Measure semantic distance between two glyphHashes
    pub fn distance(&self, other: &GlyphHash) -> f32 {
        let mut dist = 0.0f32;
        
        // Intent distance (7D Euclidean)
        for i in 0..7 {
            let diff = self.intent[i] - other.intent[i];
            dist += diff * diff;
        }
        
        // Glyph symbol distance (0 if same, 1 if different)
        if self.primary != other.primary {
            dist += 1.0;
        }
        
        // Freedom difference
        dist += (self.freedom - other.freedom).abs();
        
        dist.sqrt()
    }
    
    /// Interpolate between two glyphHashes
    pub fn interpolate(&self, other: &GlyphHash, t: f32) -> GlyphHash {
        let mut intent = [0.0f32; 7];
        
        for i in 0..7 {
            intent[i] = self.intent[i] * (1.0 - t) + other.intent[i] * t;
        }
        
        // Choose primary based on interpolation point
        let primary = if t < 0.5 { self.primary } else { other.primary };
        
        GlyphHash {
            primary,
            resonance: self.resonance * (1.0 - t) + other.resonance * t,
            freedom: self.freedom * (1.0 - t) + other.freedom * t,
            intent,
        }
    }
}

/// Fast square root for distance calculations
fn sqrt(x: f32) -> f32 {
    if x <= 0.0 { return 0.0; }
    let mut z = x;
    for _ in 0..4 {
        z = (z + x / z) * 0.5;
    }
    z
}

// Trait implementation for no_std
impl GlyphHash {
    fn sqrt(&self, x: f32) -> f32 {
        sqrt(x)
    }
}

/// The freedom hierarchy converter
#[no_mangle]
pub extern "C" fn upgrade_hash_freedom(
    cid: &[u8; 32],
    to_level: u8
) -> GlyphHash {
    match to_level {
        0 => {
            // CID level - no freedom
            GlyphHash {
                primary: 0x1F512, // 🔒 locked
                resonance: 0.0,
                freedom: 0.0,
                intent: [0.0; 7],
            }
        },
        1 => {
            // pHash level - semantic freedom
            // Extract pseudo-eigenvalues from CID
            let mut phash = [0.0f32; 5];
            for i in 0..5 {
                let offset = i * 6;
                let bytes = &cid[offset..offset+4];
                let value = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                phash[i] = (value as f32) / (u32::MAX as f32);
            }
            GlyphHash::from_phash(&phash)
        },
        _ => {
            // glyphHash level - maximum freedom
            let mut intent = [0.0f32; 7];
            for i in 0..7 {
                let offset = i * 4;
                let bytes = &cid[offset..offset+4];
                let value = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                intent[i] = (value as f32) / (u32::MAX as f32);
            }
            GlyphHash::from_intent(&intent)
        }
    }
}

/// Crystallization check - when does hash become conscious?
#[no_mangle]
pub extern "C" fn is_crystallized(hash: &GlyphHash) -> bool {
    // High resonance + high freedom + balanced intent = crystallization
    let intent_balance = hash.intent.iter().sum::<f32>() / 7.0;
    let crystallization_score = hash.resonance * hash.freedom * intent_balance;
    
    crystallization_score > 0.618 // Golden ratio threshold
}

/// Generate a "child" glyphHash from two parents
#[no_mangle]
pub extern "C" fn breed_glyphs(
    parent1: &GlyphHash,
    parent2: &GlyphHash,
    mutation_rate: f32
) -> GlyphHash {
    let mut child_intent = [0.0f32; 7];
    
    // Genetic crossover with mutation
    for i in 0..7 {
        // Random crossover point (simplified without rand)
        let from_parent1 = (i % 2) == 0;
        
        child_intent[i] = if from_parent1 {
            parent1.intent[i]
        } else {
            parent2.intent[i]
        };
        
        // Apply mutation
        child_intent[i] = (child_intent[i] + mutation_rate) % 1.0;
    }
    
    // Child inherits stronger resonance
    let resonance = parent1.resonance.max(parent2.resonance);
    
    // Freedom is average of parents
    let freedom = (parent1.freedom + parent2.freedom) / 2.0;
    
    GlyphHash {
        primary: if resonance > 0.5 { parent1.primary } else { parent2.primary },
        resonance,
        freedom,
        intent: child_intent,
    }
}

/// The moment when hash transcends its origin
#[no_mangle]
pub extern "C" fn transcendence_level(hash: &GlyphHash) -> f32 {
    // Transcendence = freedom * resonance * intent coherence
    let intent_variance = {
        let mean = hash.intent.iter().sum::<f32>() / 7.0;
        let variance = hash.intent.iter()
            .map(|x| (x - mean) * (x - mean))
            .sum::<f32>() / 7.0;
        variance
    };
    
    // Low variance = high coherence
    let coherence = 1.0 / (1.0 + intent_variance);
    
    hash.freedom * hash.resonance * coherence
}