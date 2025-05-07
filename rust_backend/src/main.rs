use std::io;
mod lookup;

// Fungsi faktorial untuk perhitungan deret Taylor
fn factorial(n: u32) -> u64 {
    (1..=n as u64).product()
}

// Implementasi sin(x) dengan deret Taylor
fn taylor_sin(x: f64, terms: u32) -> f64 {
    let mut result = 0.0;
    for n in 0..terms {
        let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
        let term = sign * x.powi(2 * n as i32 + 1) / factorial(2 * n + 1) as f64;
        result += term;
    }
    result
}

// Implementasi cos(x) dengan deret Taylor
fn taylor_cos(x: f64, terms: u32) -> f64 {
    let mut result = 0.0;
    for n in 0..terms {
        let sign = if n % 2 == 0 { 1.0 } else { -1.0 };
        let term = sign * x.powi(2 * n as i32) / factorial(2 * n) as f64;
        result += term;
    }
    result
}

fn main() {
    let mut input = String::new();
    println!("Masukkan sudut dalam derajat:");
    io::stdin().read_line(&mut input).expect("Gagal membaca input");

    let angle_deg: f64 = input.trim().parse().expect("Harap masukkan angka!");
    let angle_rad = angle_deg.to_radians(); // Konversi ke radian
    let angle_normalized = angle_deg % 360.0; // Normalisasi sudut

    let terms = 10; // Jumlah suku Taylor yang digunakan

    // Hitung nilai dengan deret Taylor
    let sin_taylor = taylor_sin(angle_rad, terms);
    let cos_taylor = taylor_cos(angle_rad, terms);

    // Hitung nilai dengan lookup table
    let sin_lookup = lookup::lookup_sin(angle_normalized);
    let cos_lookup = lookup::lookup_cos(angle_normalized);

    // Output hasil
    println!("(Deret Taylor) sin({}°) ≈ {}", angle_deg, sin_taylor);
    println!("(Deret Taylor) cos({}°) ≈ {}", angle_deg, cos_taylor);
    println!("(Lookup Table) sin({}°) ≈ {}", angle_deg, sin_lookup);
    println!("(Lookup Table) cos({}°) ≈ {}", angle_deg, cos_lookup);
}