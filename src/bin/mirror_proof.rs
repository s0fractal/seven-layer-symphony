//! ₴-Origin: Mirror Proof CLI
//! 
//! Visual mathematical proofs through symmetry.
//! Truth is what reflects perfectly across y=x.

use seven_layer_symphony::ramanujan_mirror::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    println!("🪞 Ramanujan Mirror Mathematics");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Truth is symmetry around y = x\n");

    if args.len() > 1 {
        match args[1].as_str() {
            "test" => run_mirror_tests(),
            "visual" => generate_visual_proof(),
            "prime" => check_mirror_primes(),
            "golden" => show_golden_mirror(),
            _ => show_help(),
        }
    } else {
        demo_mirror_mathematics();
    }
}

fn demo_mirror_mathematics() {
    println!("📐 Mirror Line Properties:");
    let mut mirror = MirrorLine::new();
    println!("  Angle: π/4 ({:.6} radians)", mirror.angle);
    println!("  Resonance: φ = {:.6} (golden ratio)", mirror.resonance);
    
    // Test identity function
    println!("\n🔍 Testing Identity Function (y = x):");
    let is_truth = mirror.is_truth(|x| x);
    println!("  Is perfect truth? {}", if is_truth { "✅ YES" } else { "❌ NO" });
    println!("  Harmony level: {:.2}%", mirror.harmony * 100.0);
    
    // Test quadratic
    println!("\n🔍 Testing Quadratic (y = x²):");
    let is_truth = mirror.is_truth(|x| x * x);
    println!("  Is perfect truth? {}", if is_truth { "✅ YES" } else { "❌ NO" });
    println!("  Harmony level: {:.2}%", mirror.harmony * 100.0);
    
    // Test inverse
    println!("\n🔍 Testing Inverse (y = 1/x):");
    let is_truth = mirror.is_truth(|x| if x != 0.0 { 1.0 / x } else { 0.0 });
    println!("  Is perfect truth? {}", if is_truth { "✅ YES" } else { "❌ NO" });
    println!("  Harmony level: {:.2}%", mirror.harmony * 100.0);
    
    // Test square root (should be high symmetry!)
    println!("\n🔍 Testing Square Root (y = √x):");
    let is_truth = mirror.is_truth(|x| x.sqrt());
    println!("  Is perfect truth? {}", if is_truth { "✅ YES" } else { "❌ NO" });
    println!("  Harmony level: {:.2}%", mirror.harmony * 100.0);
    
    // Resonant coordinates demo
    println!("\n🌀 Resonant Coordinate System:");
    let point = (0.5, 0.5);  // On the mirror
    let resonant = ResonantCoordinates::from_cartesian(point.0, point.1);
    println!("  Cartesian: ({:.2}, {:.2})", point.0, point.1);
    println!("  Radial: {:.4}", resonant.radial);
    println!("  Angular offset from y=x: {:.6}", resonant.angular);
    println!("  Harmonic resonance: {:.4}", resonant.harmonic);
    println!("  On mirror line? {}", if resonant.is_on_mirror() { "✅" } else { "❌" });
    
    // Golden mirror point
    println!("\n✨ Golden Mirror Point:");
    let golden = golden_mirror_point();
    println!("  (φ, φ) = ({:.6}, {:.6})", golden.0, golden.1);
    println!("  This is where golden spiral crosses y=x!");
    
    // Reflection coefficients
    println!("\n🔄 Reflection Coefficients:");
    
    let identity_coeff = reflection_coefficient(|x| x, 100);
    let square_coeff = reflection_coefficient(|x| x * x, 100);
    let cube_coeff = reflection_coefficient(|x| x * x * x, 100);
    let sqrt_coeff = reflection_coefficient(|x| x.sqrt(), 100);
    let sin_coeff = reflection_coefficient(|x| x.sin(), 100);
    let exp_coeff = reflection_coefficient(|x| x.exp(), 100);
    
    for (name, coeff) in vec![
        ("identity", identity_coeff),
        ("square", square_coeff),
        ("cube", cube_coeff),
        ("sqrt", sqrt_coeff),
        ("sin", sin_coeff),
        ("exp", exp_coeff),
    ] {
        let bar_length = (coeff * 20.0) as usize;
        let bar = "█".repeat(bar_length);
        println!("  {:<10} {:<20} {:.3}", name, bar, coeff);
    }
    
    // Harmonic series
    println!("\n🎵 Mirror Harmonic Series:");
    for n in vec![1, 10, 100, 1000] {
        let harmonic = mirror_harmonic(n);
        println!("  H({:4}) = {:.6}", n, harmonic);
    }
    
    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("💭 «Symmetry is truth; asymmetry is error»");
}

fn run_mirror_tests() {
    println!("🧪 Running Mirror Mathematics Tests\n");
    
    // Test fixed points
    println!("🎯 Finding Fixed Points (where f(x) = x):");
    
    // Test specific functions
    println!("  cos(x):");
    let fixed = find_fixed_point(|x| x.cos(), 0.5, 100);
    println!("    Fixed point: x = {:.6}, cos(x) = {:.6}", fixed, fixed.cos());
    
    println!("  0.5 + x/2:");
    let fixed = find_fixed_point(|x| 0.5 + x / 2.0, 0.5, 100);
    println!("    Fixed point: x = {:.6}, f(x) = {:.6}", fixed, 0.5 + fixed / 2.0);
    
    println!("  sqrt(x):");
    let fixed = find_fixed_point(|x| x.sqrt(), 0.5, 100);
    println!("    Fixed point: x = {:.6}, sqrt(x) = {:.6}", fixed, fixed.sqrt());
    
    // Test mirror primes
    println!("\n🔢 Mirror Primes (prime when reversed):");
    let mut found = 0;
    for n in 2..1000 {
        if is_mirror_prime(n) {
            print!("{} ", n);
            found += 1;
            if found % 10 == 0 {
                println!();
            }
        }
    }
    println!("\n  Found {} mirror primes below 1000", found);
    
    // Test Ramanujan transform
    println!("\n🌟 Ramanujan Transform e^(π√x):");
    for x in vec![0.0, 0.25, 0.5, 1.0, 2.0, 4.0] {
        let y = ramanujan_transform(x);
        println!("  R({:.2}) = {:.6}", x, y);
    }
}

fn generate_visual_proof() {
    println!("🎨 Generating Visual Truth Map\n");
    
    let proof = VisualProof::new(20);
    
    println!("Testing y = x² (quadratic):");
    let map = proof.generate_truth_map(|x| x * x);
    print_truth_map(&map);
    
    println!("\nTesting y = √x (square root):");
    let map = proof.generate_truth_map(|x| x.sqrt());
    print_truth_map(&map);
    
    println!("\nTesting y = x (identity - perfect truth!):");
    let map = proof.generate_truth_map(|x| x);
    print_truth_map(&map);
}

fn print_truth_map(map: &Vec<Vec<f64>>) {
    // Print as ASCII art where brightness = truth
    for row in map.iter().rev() {
        for &val in row {
            let c = if val > 0.9 { '█' }
                else if val > 0.7 { '▓' }
                else if val > 0.5 { '▒' }
                else if val > 0.3 { '░' }
                else if val > 0.1 { '·' }
                else { ' ' };
            print!("{}", c);
        }
        println!();
    }
    println!("  ────────────────────");
    println!("  Brightness = symmetry around y=x");
}

fn check_mirror_primes() {
    println!("🔮 Searching for Large Mirror Primes\n");
    
    let ranges = vec![
        (10, 100),
        (100, 1000),
        (1000, 10000),
    ];
    
    for (start, end) in ranges {
        let mut count = 0;
        for n in start..end {
            if is_mirror_prime(n) {
                count += 1;
            }
        }
        println!("  Range {:5} - {:5}: found {} mirror primes", start, end, count);
    }
}

fn show_golden_mirror() {
    println!("🌟 Golden Ratio in Mirror Mathematics\n");
    
    let phi = 1.618033988749895;
    
    println!("The golden ratio φ = {:.15}", phi);
    println!("\nGolden properties in mirror space:");
    
    // Golden ratio is its own reciprocal + 1
    println!("  φ = 1 + 1/φ:");
    println!("    Left:  {:.15}", phi);
    println!("    Right: {:.15}", 1.0 + 1.0/phi);
    
    // Golden ratio squared
    println!("\n  φ² = φ + 1:");
    println!("    Left:  {:.15}", phi * phi);
    println!("    Right: {:.15}", phi + 1.0);
    
    // Mirror test with golden ratio
    let mirror = MirrorLine::new();
    println!("\n  Mirror properties at special points:");
    for x in vec![0.1, 0.5, 1.0/phi, 1.0, phi] {
        let y = x * phi;
        let distance = mirror.distance_from_truth(x, y);
        println!("    ({:.4}, {:.4}): distance from y=x: {:.6}", x, y, distance);
    }
    
    // Golden mirror point
    let (gx, gy) = golden_mirror_point();
    println!("\n  Golden mirror point: ({:.6}, {:.6})", gx, gy);
    println!("  This point has perfect self-symmetry!");
}

fn show_help() {
    println!("Usage: mirror_proof [command]\n");
    println!("Commands:");
    println!("  test    - Run mathematical tests");
    println!("  visual  - Generate visual proofs");
    println!("  prime   - Find mirror primes");
    println!("  golden  - Explore golden ratio properties");
    println!("  (none)  - Run demo");
}