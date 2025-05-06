use std::error::Error;
use std::fs::File;
use serde::{Deserialize, Serialize};
use serde_json;

mod data;
mod model;
mod utils;

use data::{load_csv, normalize_features, split_data};
use model::NeuralNetwork;
use utils::{plot_accuracy, plot_loss};

#[derive(Serialize)]
struct TrainingResults {
    train_accuracy_history: Vec<f64>,
    train_loss_history: Vec<f64>,
    val_accuracy: f64,
    test_accuracy: f64,
}

#[derive(Serialize, Deserialize)]
struct Scaler {
    min_vals: Vec<f64>,
    max_vals: Vec<f64>,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("Usage:");
        println!("  Training: {} --train <csv_path> <epochs> <learning_rate>", args[0]);
        println!("  Prediction: {} --predict <aluminium> <ammonia> <arsenic> <barium> <chloramine> <chromium> <copper> <fluoride> <bacteria> <viruses> <mercury> <radium> <silver> <uranium>", args[0]);
        return Err("Invalid arguments".into());
    }

    if args[1] == "--train" {
        if args.len() != 5 {
            return Err("Training requires csv_path, epochs, and learning_rate".into());
        }
        let csv_path = &args[2];
        let epochs: usize = args[3].parse()?;
        let learning_rate: f64 = args[4].parse()?;
        train_model(csv_path, epochs, learning_rate)?;
    } else if args[1] == "--predict" {
        if args.len() != 16 {
            return Err("Prediction requires aluminium, ammonia, arsenic, barium, chloramine, chromium, copper, fluoride, bacteria, viruses, mercury, radium, silver, and uranium".into());
        }
        let features: Vec<f64> = args[2..16]
            .iter()
            .map(|x| x.parse().expect("Invalid feature value"))
            .collect();
        predict(&features)?;
    } else {
        return Err("Invalid command: use --train or --predict".into());
    }

    Ok(())
}

fn train_model(csv_path: &str, epochs: usize, learning_rate: f64) -> Result<(), Box<dyn Error>> {
    // Resolve relative paths to absolute, relative to project root
    let project_root = std::env::current_dir()?;
    let csv_path = if csv_path.starts_with("./") || !csv_path.starts_with("/") {
        project_root.join(csv_path).to_str().ok_or("Invalid CSV path")?.to_string()
    } else {
        csv_path.to_string()
    };

    // Load data
    let data = load_csv(&csv_path)?;
    println!("Loaded {} data points", data.len());

    // Normalize features
    let mut data = data;
    let (min_vals, max_vals) = normalize_features(&mut data);

    let scaler = Scaler { min_vals, max_vals };
    let scaler_file = File::create("scaler.bin")?;
    serde_json::to_writer(scaler_file, &scaler)?;

    // Split data
    let (train_data, temp_data) = split_data(&data, 0.7);
    let (val_data, test_data) = split_data(&temp_data, 0.5);

    println!("Training data size: {}", train_data.len());
    println!("Validation data size: {}", val_data.len());
    println!("Testing data size: {}", test_data.len());

    let input_size = 14; // 14 features for water quality dataset
    let hidden_size = 16; // Adjusted for complexity
    let output_size = 1; // Binary outcome
    let mut nn = NeuralNetwork::new(input_size, hidden_size, output_size);

    // Train the model
    let (train_accuracy_history, train_loss_history) = nn.train(&train_data, epochs, learning_rate, &val_data);

    // Evaluate
    let val_accuracy = nn.evaluate(&val_data) * 100.0;
    let test_accuracy = nn.evaluate(&test_data) * 100.0;
    println!(
        "{}",
        serde_json::json!({
            "val_accuracy": val_accuracy,
            "test_accuracy": test_accuracy
        })
    );

    // Save model
    let model_file = File::create("model.bin")?;
    serde_json::to_writer(model_file, &nn)?;

    // Save results
    let results = TrainingResults {
        train_accuracy_history,
        train_loss_history,
        val_accuracy,
        test_accuracy,
    };
    let results_file = File::create("results.json")?;
    serde_json::to_writer(results_file, &results)?;

    // Plot results
    std::fs::create_dir_all("output")?;
    plot_accuracy(&results.train_accuracy_history, "output/training_plot.png")?;
    plot_loss(&results.train_loss_history, "output/training_loss_plot.png")?;

    Ok(())
}

fn predict(features: &[f64]) -> Result<(), Box<dyn Error>> {
    let model_file = File::open("model.bin")?;
    let nn: NeuralNetwork = serde_json::from_reader(model_file)?;
    let scaler_file = File::open("scaler.bin")?;
    let scaler: Scaler = serde_json::from_reader(scaler_file)?;

    let mut normalized_features = features.to_vec();
    for i in 0..features.len() {
        let range = scaler.max_vals[i] - scaler.min_vals[i];
        normalized_features[i] = if range > 0.0 {
            (features[i] - scaler.min_vals[i]) / range
        } else {
            0.5
        };
    }

    let (probability, prediction) = nn.predict_with_probability(&normalized_features);
    let output = serde_json::json!({
        "prediction": prediction,
        "probability": probability
    });
    println!("{}", output);
    Ok(())
}