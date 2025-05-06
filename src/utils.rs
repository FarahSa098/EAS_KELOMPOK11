use plotters::prelude::*;

pub fn plot_accuracy(accuracy_history: &[f64], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let epochs = accuracy_history.len();
    let max_y = accuracy_history.iter().fold(f64::MIN, |a, &b| a.max(b));
    let min_y = accuracy_history.iter().fold(f64::MAX, |a, &b| a.min(b));

    let mut chart = ChartBuilder::on(&root)
        .caption("Accuracy vs Epochs", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..epochs, (min_y - 0.05).max(0.0)..(max_y + 0.05).min(1.0))?;

    chart
        .configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .x_desc("Epochs")
        .y_desc("Accuracy")
        .draw()?;

    chart.draw_series(LineSeries::new(
        (0..epochs).map(|i| (i, accuracy_history[i])),
        &RED,
    ))?;

    root.present()?;
    println!("Accuracy plot has been saved to {}", path);
    Ok(())
}

pub fn plot_loss(loss_history: &[f64], path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(path, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let epochs = loss_history.len();
    let max_y = loss_history.iter().fold(f64::MIN, |a, &b| a.max(b));
    let min_y = loss_history.iter().fold(f64::MAX, |a, &b| a.min(b));

    let mut chart = ChartBuilder::on(&root)
        .caption("Loss vs Epochs", ("sans-serif", 30).into_font())
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0..epochs, (min_y - 0.05).max(0.0)..(max_y + 0.05))?;

    chart
        .configure_mesh()
        .x_labels(20)
        .y_labels(10)
        .x_desc("Epochs")
        .y_desc("Loss")
        .draw()?;

    chart.draw_series(LineSeries::new(
        (0..epochs).map(|i| (i, loss_history[i])),
        &BLUE,
    ))?;

    root.present()?;
    println!("Loss plot has been saved to {}", path);
    Ok(())
}