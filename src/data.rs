use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn load_csv(path: &str) -> Result<Vec<(Vec<f64>, f64)>, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut data = Vec::new();
    for line in reader.lines().skip(1) { // Skip header
        let line = line?;
        let values: Vec<f64> = line
            .split(',')
            .map(|s| {
                let trimmed = s.trim();
                trimmed.parse().unwrap_or(f64::NAN) // Handle invalid numbers
            })
            .collect();
        if values.len() >= 15 { // Expecting 14 features + 1 label
            let features = values[0..14].to_vec(); // Collect 14 features
            let label = values[14]; // The last column is the label
            // Check for NaN or Infinity in features or label
            if features.iter().all(|&x| x.is_finite()) && label.is_finite() {
                data.push((features, label));
            }
        }
    }
    if data.is_empty() {
        return Err("No valid data points found in CSV".into());
    }
    Ok(data)
}

pub fn normalize_features(data: &mut [(Vec<f64>, f64)]) -> (Vec<f64>, Vec<f64>) {
    let n_features = data[0].0.len();
    let mut min_vals = vec![f64::MAX; n_features];
    let mut max_vals = vec![f64::MIN; n_features];
    for (features, _) in data.iter() {
        for i in 0..n_features {
            min_vals[i] = min_vals[i].min(features[i]);
            max_vals[i] = max_vals[i].max(features[i]);
        }
    }
    for (features, _) in data.iter_mut() {
        for i in 0..n_features {
            let range = max_vals[i] - min_vals[i];
            features[i] = if range > 0.0 {
                (features[i] - min_vals[i]) / range
            } else {
                0.5
            };
        }
    }
    (min_vals, max_vals)
}

pub fn split_data(data: &[(Vec<f64>, f64)], ratio: f64) -> (Vec<(Vec<f64>, f64)>, Vec<(Vec<f64>, f64)>) {
    let split_idx = (data.len() as f64 * ratio) as usize;
    let mut shuffled = data.to_vec();
    shuffled.shuffle(&mut thread_rng()); // Safe shuffling
    (
        shuffled[..split_idx].to_vec(),
        shuffled[split_idx..].to_vec(),
    )
}
