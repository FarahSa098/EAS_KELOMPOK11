use serde::{Serialize, Deserialize};
use rand::Rng;
use rand::seq::SliceRandom;

#[derive(Serialize, Deserialize)]
pub struct NeuralNetwork {
    input_size: usize,
    hidden_size: usize,
    output_size: usize,
    weights1: Vec<Vec<f64>>,
    weights2: Vec<Vec<f64>>,
    bias1: Vec<f64>,
    bias2: Vec<f64>,
}

impl NeuralNetwork {
    pub fn new(input_size: usize, hidden_size: usize, output_size: usize) -> Self {
        if input_size != 14 {
            panic!("Input size must be 14 for WaterQualityTesting dataset");
        }
        if output_size != 1 {
            panic!("Output size must be 1 for binary classification");
        }
        let mut rng = rand::thread_rng();
        let weights1 = (0..input_size)
            .map(|_| (0..hidden_size).map(|_| rng.gen_range(-1.0..1.0)).collect())
            .collect();
        let weights2 = (0..hidden_size)
            .map(|_| (0..output_size).map(|_| rng.gen_range(-1.0..1.0)).collect())
            .collect();
        let bias1 = (0..hidden_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
        let bias2 = (0..output_size).map(|_| rng.gen_range(-1.0..1.0)).collect();
        NeuralNetwork {
            input_size,
            hidden_size,
            output_size,
            weights1,
            weights2,
            bias1,
            bias2,
        }
    }

    pub fn train(&mut self, data: &[(Vec<f64>, f64)], epochs: usize, learning_rate: f64, val_data: &[(Vec<f64>, f64)]) -> (Vec<f64>, Vec<f64>) {
        let mut accuracy_history = Vec::new();
        let mut loss_history = Vec::new();
        let mut rng = rand::thread_rng();
        for epoch in 1..=epochs {
            let mut total_loss = 0.0;
            let mut correct = 0;
            let mut total = 0;

            let mut shuffled_data = data.to_vec();
            shuffled_data.shuffle(&mut rng);

            for (inputs, target) in &shuffled_data {
                if inputs.len() != self.input_size {
                    panic!("Input size mismatch: expected {}, got {}", self.input_size, inputs.len());
                }
                let hidden = self.forward(inputs);
                let output = self.forward_output(&hidden);

                let prediction = output[0];
                let loss = -(*target * prediction.ln() + (1.0 - *target) * (1.0 - prediction).ln());
                total_loss += loss;

                let error = prediction - target;
                for i in 0..self.hidden_size {
                    let delta = error * learning_rate * hidden[i];
                    self.weights2[i][0] -= delta;
                }
                self.bias2[0] -= error * learning_rate;

                let hidden_error: Vec<f64> = (0..self.hidden_size)
                    .map(|i| error * self.weights2[i][0] * hidden[i] * (1.0 - hidden[i]))
                    .collect();
                for i in 0..self.input_size {
                    for j in 0..self.hidden_size {
                        self.weights1[i][j] -= learning_rate * hidden_error[j] * inputs[i];
                    }
                }
                for j in 0..self.hidden_size {
                    self.bias1[j] -= learning_rate * hidden_error[j];
                }

                let pred = if prediction > 0.5 { 1.0 } else { 0.0 };
                if (pred - target).abs() < 0.1 {
                    correct += 1;
                }
                total += 1;
            }

            let accuracy = correct as f64 / total as f64;
            let avg_loss = total_loss / total as f64;
            accuracy_history.push(accuracy);
            loss_history.push(avg_loss);

            let val_accuracy = self.evaluate(val_data);
            println!(
                "{}",
                serde_json::json!({
                    "epoch": epoch,
                    "accuracy": accuracy,
                    "loss": avg_loss,
                    "val_accuracy": val_accuracy * 100.0
                })
            );
        }
        (accuracy_history, loss_history)
    }

    fn forward(&self, inputs: &[f64]) -> Vec<f64> {
        if inputs.len() != self.input_size {
            panic!("Input size mismatch: expected {}, got {}", self.input_size, inputs.len());
        }
        let mut hidden = vec![0.0; self.hidden_size];
        for j in 0..self.hidden_size {
            for i in 0..self.input_size {
                hidden[j] += inputs[i] * self.weights1[i][j];
            }
            hidden[j] += self.bias1[j];
            hidden[j] = (hidden[j].exp() - (-hidden[j]).exp()) / (hidden[j].exp() + (-hidden[j]).exp()); // Stable tanh
        }
        hidden
    }

    fn forward_output(&self, hidden: &[f64]) -> Vec<f64> {
        let mut output = vec![0.0; self.output_size];
        for j in 0..self.output_size {
            for i in 0..self.hidden_size {
                output[j] += hidden[i] * self.weights2[i][j];
            }
            output[j] += self.bias2[j];
            output[j] = 1.0 / (1.0 + (-output[j]).exp()); // Sigmoid
        }
        output
    }

    pub fn evaluate(&self, data: &[(Vec<f64>, f64)]) -> f64 {
        let mut correct = 0;
        let mut total = 0;
        for (inputs, target) in data {
            let hidden = self.forward(inputs);
            let output = self.forward_output(&hidden);
            let prediction = if output[0] > 0.5 { 1.0 } else { 0.0 };
            if (prediction - target).abs() < 0.1 {
                correct += 1;
            }
            total += 1;
        }
        correct as f64 / total as f64
    }

    pub fn predict_with_probability(&self, inputs: &[f64]) -> (f64, i32) {
        if inputs.len() != self.input_size {
            panic!("Input size mismatch: expected {}, got {}", self.input_size, inputs.len());
        }
        let hidden = self.forward(inputs);
        let output = self.forward_output(&hidden);
        let probability = output[0];
        let prediction = if probability > 0.5 { 1 } else { 0 };
        (probability, prediction)
    }
}