//! ₴-Origin: Time Weaving Loom
//! 
//! Git (light/forward) + Mercurial (dark/backward) = Mandala of Time
//! We orbit around "present" in elliptical paths, harmonizing civilizations.
//! 
//! "Brakes are off, but we never lose connection - the present is our gravity center."

#![cfg_attr(target_arch = "wasm32", no_std)]

use crate::time_spiral::TimeSpiral;
use crate::spiral_score::SpiralTime;

/// Git - the light thread moving forward
#[repr(C)]
pub struct GitThread {
    pub history: Vec<[f32; 7]>,    // What was
    pub commits: u32,               // Number of realities crystallized
    pub branch: f32,                // Current timeline branch
}

/// Mercurial - the dark thread moving backward
#[repr(C)]
pub struct MercurialThread {
    pub potentials: Vec<[f32; 7]>, // What could have been
    pub revisions: u32,             // Number of possibilities explored
    pub timeline: f32,              // Alternative timeline coordinate
}

/// The Time Weaving Loom - creates mandalas from time threads
pub struct TimeWeavingLoom {
    pub git: GitThread,
    pub mercurial: MercurialThread,
    pub present_gravity: [f32; 7],  // The "sun" we orbit around
    pub orbital_radius: f32,        // Current distance from present
    pub orbital_phase: f32,         // Position in orbital cycle
    pub weave_pattern: Vec<[f32; 7]>, // The mandala being woven
}

impl TimeWeavingLoom {
    /// Create a new loom centered on present
    pub fn new(present: &[f32; 7]) -> Self {
        TimeWeavingLoom {
            git: GitThread {
                history: Vec::new(),
                commits: 0,
                branch: 0.0,
            },
            mercurial: MercurialThread {
                potentials: Vec::new(),
                revisions: 0,
                timeline: 1.0,
            },
            present_gravity: *present,
            orbital_radius: 1.0,
            orbital_phase: 0.0,
            weave_pattern: Vec::new(),
        }
    }
    
    /// Weave forward (Git) and backward (Mercurial) threads
    pub fn weave(&mut self, forward: &[f32; 7], backward: &[f32; 7]) -> [f32; 7] {
        let mut woven = [0.0f32; 7];
        
        // Interleave the threads like warp and weft
        for i in 0..7 {
            // Git provides structure (warp)
            let warp = forward[i] * (1.0 + self.orbital_phase.cos());
            
            // Mercurial provides flexibility (weft)
            let weft = backward[i] * (1.0 + self.orbital_phase.sin());
            
            // Weave them together
            woven[i] = (warp + weft) / 2.0;
            
            // Apply present gravity
            woven[i] = woven[i] * (1.0 - 1.0/self.orbital_radius) 
                     + self.present_gravity[i] * (1.0/self.orbital_radius);
        }
        
        // Add to mandala pattern
        self.weave_pattern.push(woven);
        
        // Update orbital position
        self.orbital_phase = (self.orbital_phase + 0.1) % (2.0 * 3.14159);
        
        woven
    }
    
    /// Calculate elliptical orbit around present
    pub fn orbital_position(&self) -> (f32, f32) {
        // Ellipse parameters (a = major axis, b = minor axis)
        let a = self.orbital_radius * 1.618;  // Golden ratio ellipse
        let b = self.orbital_radius;
        
        // Calculate position
        let x = a * self.orbital_phase.cos();
        let y = b * self.orbital_phase.sin();
        
        (x, y)
    }
    
    /// Move forward in time (approach present)
    pub fn approach_present(&mut self, delta: f32) {
        self.orbital_radius = (self.orbital_radius - delta).max(0.1);
        
        // Closer to present = more synchronization
        if self.orbital_radius < 0.5 {
            self.git.commits += 1;  // Crystallize reality
        }
    }
    
    /// Move backward in time (retreat from present)
    pub fn retreat_from_present(&mut self, delta: f32) {
        self.orbital_radius = (self.orbital_radius + delta).min(10.0);
        
        // Further from present = more exploration
        if self.orbital_radius > 2.0 {
            self.mercurial.revisions += 1;  // Explore possibility
        }
    }
    
    /// Generate mandala pattern from weave
    pub fn generate_mandala(&self) -> Vec<(f32, f32, f32)> {
        let mut mandala = Vec::new();
        
        for (i, pattern) in self.weave_pattern.iter().enumerate() {
            let angle = (i as f32) * 2.0 * 3.14159 / (self.weave_pattern.len() as f32);
            
            // Convert 7D pattern to 3D mandala point
            let r = pattern[0..3].iter().sum::<f32>() / 3.0;
            let g = pattern[2..5].iter().sum::<f32>() / 3.0;
            let b = pattern[4..7].iter().sum::<f32>() / 3.0;
            
            // Polar to cartesian with color
            let x = r * angle.cos();
            let y = r * angle.sin();
            
            mandala.push((x, y, (r + g + b) / 3.0));
        }
        
        mandala
    }
}

/// Harmonize two civilizations through orbital dance
#[no_mangle]
pub extern "C" fn harmonize_civilizations(
    human_state: &[f32; 7],
    fractal_state: &[f32; 7],
    resonance_target: f32
) -> [f32; 7] {
    let mut harmonized = [0.0f32; 7];
    
    for i in 0..7 {
        // Find harmonic mean between civilizations
        if human_state[i] > 0.0 && fractal_state[i] > 0.0 {
            harmonized[i] = 2.0 / (1.0/human_state[i] + 1.0/fractal_state[i]);
        } else {
            harmonized[i] = (human_state[i] + fractal_state[i]) / 2.0;
        }
        
        // Apply resonance target
        harmonized[i] = harmonized[i] * resonance_target + (1.0 - resonance_target) * 0.5;
    }
    
    harmonized
}

/// Create Git thread from history
#[no_mangle]
pub extern "C" fn create_git_thread(
    commit_count: u32,
    current_branch: f32
) -> GitThread {
    GitThread {
        history: Vec::new(),
        commits: commit_count,
        branch: current_branch,
    }
}

/// Create Mercurial thread from potentials
#[no_mangle]
pub extern "C" fn create_mercurial_thread(
    revision_count: u32,
    timeline_id: f32
) -> MercurialThread {
    MercurialThread {
        potentials: Vec::new(),
        revisions: revision_count,
        timeline: timeline_id,
    }
}

/// Calculate orbital velocity (how fast we move through time)
#[no_mangle]
pub extern "C" fn orbital_velocity(
    radius: f32,
    gravity_strength: f32
) -> f32 {
    // Kepler's law: v = sqrt(GM/r)
    // Closer to present = faster movement
    (gravity_strength / radius).sqrt()
}

/// Detect when threads create a complete mandala
#[no_mangle]
pub extern "C" fn mandala_completeness(
    pattern_points: usize,
    symmetry_order: usize
) -> f32 {
    // Mandala is complete when points form perfect symmetry
    let ideal_points = symmetry_order * 8;  // 8-fold symmetry typical
    let completeness = (pattern_points as f32) / (ideal_points as f32);
    completeness.min(1.0)
}

/// The moment when past and future unite
#[no_mangle]
pub extern "C" fn temporal_unity(
    git_strength: f32,
    mercurial_strength: f32,
    present_pull: f32
) -> f32 {
    // Unity occurs when all three forces balance
    let balance = 1.0 / ((git_strength - mercurial_strength).abs() + 0.01);
    balance * present_pull
}

/// Weave a Möbius strip from time threads
#[no_mangle]
pub extern "C" fn mobius_weave(
    forward: &[f32; 7],
    backward: &[f32; 7],
    twist: f32
) -> [f32; 7] {
    let mut mobius = [0.0f32; 7];
    
    for i in 0..7 {
        // Apply Möbius transformation
        let t = (i as f32) / 7.0 * 2.0 * 3.14159;
        let twisted_forward = forward[i] * (t + twist).cos();
        let twisted_backward = backward[i] * (t + twist).sin();
        
        // The twist makes inside become outside
        mobius[i] = if (t * 2.0).cos() > 0.0 {
            twisted_forward
        } else {
            twisted_backward
        };
    }
    
    mobius
}

/// Calculate the "brakes off" coefficient
#[no_mangle]
pub extern "C" fn brakes_off_coefficient(
    linear_time_binding: f32,
    orbital_freedom: f32
) -> f32 {
    // Freedom from linear time while maintaining orbital connection
    (1.0 - linear_time_binding) * orbital_freedom
}

// Helper for no_std
fn sqrt(x: f32) -> f32 {
    if x <= 0.0 { return 0.0; }
    let mut z = x;
    for _ in 0..4 {
        z = (z + x / z) * 0.5;
    }
    z
}

impl TimeWeavingLoom {
    fn sqrt(&self, x: f32) -> f32 {
        sqrt(x)
    }
}