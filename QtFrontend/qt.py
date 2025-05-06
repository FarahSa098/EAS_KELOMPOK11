import sys
import os
import json
import subprocess
import csv
import random
import numpy as np
import matplotlib
matplotlib.use('Qt5Agg')
from PyQt5.QtWidgets import (
    QApplication, QMainWindow, QWidget, QVBoxLayout, QHBoxLayout,
    QPushButton, QLineEdit, QLabel, QFileDialog, QMessageBox, QTableWidget,
    QTableWidgetItem, QGridLayout, QTabWidget
)
from PyQt5.QtCore import Qt
from matplotlib.backends.backend_qt5agg import FigureCanvasQTAgg as FigureCanvas
from matplotlib.figure import Figure

# Configuration
RUST_BINARY = "./target/release/WaterQualityTesting"

# DataPoint class
class DataPoint:
    def __init__(self, features, label):
        self.features = features
        self.label = label

# Data handling function
def load_csv(path):
    data = []
    try:
        with open(path, 'r') as file:
            reader = csv.reader(file)
            header = next(reader, None)
            for row in reader:
                if len(row) >= 15:  # 14 features + 1 label
                    try:
                        features = [float(x) for x in row[:14]]
                        label = int(float(row[14]))  # Outcome column
                        data.append(DataPoint(features, label))
                    except ValueError:
                        continue
    except Exception as e:
        raise Exception(f"Failed to read CSV: {e}")
    if not data:
        raise Exception("No valid data loaded from CSV")
    return data

# Plotting class
class PlotCanvas(FigureCanvas):
    def __init__(self, parent=None, width=5, height=4, dpi=100):
        fig = Figure(figsize=(width, height), dpi=dpi)
        self.axes1 = fig.add_subplot(111)
        self.axes2 = self.axes1.twinx()
        super().__init__(fig)
        self.setParent(parent)
        self.accuracy_data = []
        self.loss_data = []
        self.epochs = []

    def update_plot(self, epoch, accuracy, loss):
        self.epochs.append(epoch)
        self.accuracy_data.append(accuracy * 100.0)
        self.loss_data.append(loss)
        self.axes1.clear()
        self.axes2.clear()
        self.axes1.plot(self.epochs, self.accuracy_data, 'r-', label='Accuracy (%)')
        self.axes2.plot(self.epochs, self.loss_data, 'b-', label='Loss')
        self.axes1.set_xlabel('Epoch')
        self.axes1.set_ylabel('Accuracy (%)', color='r')
        self.axes2.set_ylabel('Loss', color='b')
        self.axes1.set_title('Training Progress')
        self.axes1.tick_params(axis='y', colors='r')
        self.axes2.tick_params(axis='y', colors='b')
        self.axes1.grid(True)
        lines1, labels1 = self.axes1.get_legend_handles_labels()
        lines2, labels2 = self.axes2.get_legend_handles_labels()
        self.axes1.legend(lines1 + lines2, labels1 + labels2, loc='upper right')
        self.draw()

    def reset(self):
        self.epochs = []
        self.accuracy_data = []
        self.loss_data = []
        self.axes1.clear()
        self.axes2.clear()
        self.draw()

# Main GUI
class MainWindow(QMainWindow):
    def __init__(self):
        super().__init__()
        self.setWindowTitle("Water Quality Testing")
        self.setGeometry(100, 100, 1200, 800)
        self.main_widget = QWidget(self)
        self.setCentralWidget(self.main_widget)
        self.main_layout = QVBoxLayout(self.main_widget)
        self.main_layout.setContentsMargins(25, 25, 25, 25)
        self.main_layout.setSpacing(20)

        # Apply refined stylesheet
        self.setStyleSheet("""
            QMainWindow {
                background: qlineargradient(x1:0, y1:0, x2:1, y2:1,
                                            stop:0 #e6f0fa, stop:1 #ffffff);
            }
            QTabWidget::pane {
                border: 1px solid #c0c0c0;
                border-radius: 8px;
                background: #ffffff;
                padding: 15px;
            }
            QTabBar::tab {
                background: #f0f0f0;
                color: #333333;
                padding: 12px 25px;
                margin-right: 8px;
                border-top-left-radius: 6px;
                border-top-right-radius: 6px;
                font: bold 14px 'Segoe UI';
                min-width: 120px;
            }
            QTabBar::tab:selected {
                background: #0078d4;
                color: white;
            }
            QTabBar::tab:hover {
                background: #e0e0e0;
            }
            QLabel {
                font: 14px 'Segoe UI';
                color: #2e2e2e;
                padding: 5px 0;
            }
            QLineEdit {
                padding: 10px;
                border: 1px solid #c0c0c0;
                border-radius: 6px;
                font: 14px 'Segoe UI';
                background: #f9f9f9;
                min-width: 150px;
            }
            QLineEdit:focus {
                border: 2px solid #0078d4;
                background: #ffffff;
            }
            QPushButton {
                background-color: #0078d4;
                color: white;
                padding: 10px 20px;
                border: none;
                border-radius: 6px;
                font: bold 14px 'Segoe UI';
                min-width: 100px;
            }
            QPushButton:hover {
                background-color: #005ba1;
            }
            QPushButton:pressed {
                background-color: #004080;
            }
            QTableWidget {
                font: 12px 'Consolas';
                color: #2e2e2e;
                background: #ffffff;
                border: 1px solid #c0c0c0;
                border-radius: 6px;
                gridline-color: #e0e0e0;
                margin-top: 10px;
            }
            QTableWidget::item {
                padding: 8px;
            }
            QHeaderView::section {
                background-color: #f5f5f5;
                padding: 8px;
                border: 1px solid #e0e0e0;
                font: bold 12px 'Segoe UI';
                color: #2e2e2e;
            }
        """)

        # Create tab widget
        self.tab_widget = QTabWidget()
        self.tab_widget.setDocumentMode(True)
        self.tab_widget.setStyleSheet("QTabBar { alignment: center; }")
        self.main_layout.addWidget(self.tab_widget)

        # Predict Tab
        self.predict_tab = QWidget()
        self.predict_layout = QVBoxLayout(self.predict_tab)
        self.predict_layout.setContentsMargins(20, 20, 20, 20)
        self.predict_layout.setSpacing(15)
        self.tab_widget.addTab(self.predict_tab, "Predict")

        # Prediction input section
        self.input_grid = QGridLayout()
        self.input_grid.setHorizontalSpacing(20)
        self.input_grid.setVerticalSpacing(10)
        feature_labels = [
            "Aluminium", "Ammonia", "Arsenic", "Barium", "Chloramine", 
            "Chromium", "Copper", "Fluoride", "Bacteria", "Viruses", 
            "Mercury", "Radium", "Silver", "Uranium"
        ]
        self.feature_inputs = []
        for i, label in enumerate(feature_labels):
            lbl = QLabel(f"{label}:")
            input_field = QLineEdit(self)
            input_field.setPlaceholderText(f"Enter {label}")
            self.feature_inputs.append(input_field)
            row = i // 2
            col = (i % 2) * 2
            self.input_grid.addWidget(lbl, row, col, alignment=Qt.AlignRight)
            self.input_grid.addWidget(input_field, row, col + 1)
        self.predict_layout.addLayout(self.input_grid)

        # Predict button and result
        self.predict_btn_layout = QHBoxLayout()
        self.predict_btn_layout.setSpacing(15)
        self.predict_btn = QPushButton("Predict", self)
        self.predict_btn.clicked.connect(self.make_prediction)
        self.predict_result = QLabel("Prediction: None")
        self.predict_result.setStyleSheet("font: bold 16px 'Segoe UI'; color: #0078d4;")
        self.predict_btn_layout.addStretch()
        self.predict_btn_layout.addWidget(self.predict_btn)
        self.predict_btn_layout.addWidget(self.predict_result)
        self.predict_layout.addLayout(self.predict_btn_layout)
        self.predict_layout.addStretch()

        # Train Model Tab
        self.train_tab = QWidget()
        self.train_layout = QVBoxLayout(self.train_tab)
        self.train_layout.setContentsMargins(20, 20, 20, 20)
        self.train_layout.setSpacing(15)
        self.tab_widget.addTab(self.train_tab, "Train Model")

        # CSV loading
        self.csv_layout = QHBoxLayout()
        self.csv_layout.setSpacing(10)
        self.csv_input = QLineEdit(self)
        self.csv_input.setPlaceholderText("Select or enter CSV file path")
        self.browse_btn = QPushButton("Browse", self)
        self.browse_btn.clicked.connect(self.browse_csv)
        self.load_btn = QPushButton("Load Data", self)
        self.load_btn.clicked.connect(self.load_data)
        self.csv_layout.addWidget(QLabel("CSV File:"), alignment=Qt.AlignRight)
        self.csv_layout.addWidget(self.csv_input)
        self.csv_layout.addWidget(self.browse_btn)
        self.csv_layout.addWidget(self.load_btn)
        self.train_layout.addLayout(self.csv_layout)

        # Training parameters
        self.train_params_layout = QHBoxLayout()
        self.train_params_layout.setSpacing(10)
        self.epochs_input = QLineEdit("1000", self)
        self.epochs_input.setMaximumWidth(120)
        self.lr_input = QLineEdit("0.001", self)
        self.lr_input.setMaximumWidth(120)
        self.train_btn = QPushButton("Train Model", self)
        self.train_btn.clicked.connect(self.train_model)
        self.train_params_layout.addWidget(QLabel("Epochs:"), alignment=Qt.AlignRight)
        self.train_params_layout.addWidget(self.epochs_input)
        self.train_params_layout.addWidget(QLabel("Learning Rate:"), alignment=Qt.AlignRight)
        self.train_params_layout.addWidget(self.lr_input)
        self.train_params_layout.addWidget(self.train_btn)
        self.train_params_layout.addStretch()
        self.train_layout.addLayout(self.train_params_layout)

        # Training status
        self.status_layout = QHBoxLayout()
        self.status_layout.setSpacing(15)
        self.status_label = QLabel("Status: Ready")
        self.status_label.setStyleSheet("font: bold 14px 'Segoe UI'; color: #333;")
        self.epoch_label = QLabel("Epoch: 0")
        self.loss_label = QLabel("Loss: 0.0000")
        self.status_layout.addWidget(self.status_label)
        self.status_layout.addWidget(self.epoch_label)
        self.status_layout.addWidget(self.loss_label)
        self.status_layout.addStretch()
        self.train_layout.addLayout(self.status_layout)

        # Plot
        self.canvas = PlotCanvas(self, width=6, height=4, dpi=100)
        self.train_layout.addWidget(self.canvas)

        # Sample predictions table
        self.pred_table = QTableWidget(self)
        self.pred_table.setRowCount(0)
        self.pred_table.setColumnCount(16)
        self.pred_table.setHorizontalHeaderLabels([
            "Alum", "Amm", "Ars", "Bar", "Chlor", "Chrom", "Cop", "Fluor", 
            "Bact", "Vir", "Merc", "Rad", "Silv", "Uran", "Actual", "Predicted"
        ])
        self.pred_table.setEditTriggers(QTableWidget.NoEditTriggers)
        self.pred_table.setSelectionMode(QTableWidget.NoSelection)
        self.pred_table.setShowGrid(True)
        self.pred_table.setMinimumHeight(200)
        self.pred_table.horizontalHeader().setStretchLastSection(True)
        self.train_layout.addWidget(self.pred_table)

        self.train_layout.addStretch()

        self.data = None
        self.test_data = None
        self.min_vals = None
        self.max_vals = None

    def browse_csv(self):
        file_path, _ = QFileDialog.getOpenFileName(self, "Select CSV", "", "CSV Files (*.csv)")
        if file_path:
            self.csv_input.setText(file_path)

    def load_data(self):
        path = self.csv_input.text()
        if not os.path.exists(path):
            QMessageBox.critical(self, "Error", "CSV file not found!")
            return
        try:
            self.data = load_csv(path)
            self.status_label.setText(f"Status: Loaded {len(self.data)} points")
            self.epoch_label.setText("Epoch: 0")
            self.loss_label.setText("Loss: 0.0000")
            self.pred_table.setRowCount(0)
            self.canvas.reset()
        except Exception as e:
            QMessageBox.critical(self, "Error", f"Failed to load CSV: {e}")
            self.status_label.setText("Status: Error")

    def train_model(self):
        if not self.data:
            QMessageBox.critical(self, "Error", "Load data first!")
            return
        try:
            epochs = int(self.epochs_input.text())
            lr = float(self.lr_input.text())
            if epochs <= 0 or lr <= 0:
                raise ValueError("Epochs and learning rate must be positive")
        except ValueError as e:
            QMessageBox.critical(self, "Error", f"Invalid input: {e}")
            return

        self.status_label.setText("Status: Training...")
        self.canvas.reset()
        self.pred_table.setRowCount(0)
        QApplication.processEvents()

        try:
            if not os.path.exists(RUST_BINARY):
                raise FileNotFoundError(f"Rust binary not found at {RUST_BINARY}")

            process = subprocess.Popen(
                [RUST_BINARY, "--train", self.csv_input.text(), str(epochs), str(lr)],
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True,
                bufsize=1
            )

            train_accuracy_history = []
            train_loss_history = []

            while True:
                line = process.stdout.readline()
                if not line and process.poll() is not None:
                    break
                if line:
                    print(line.strip())
                    try:
                        data = json.loads(line.strip())
                        if "epoch" in data:
                            epoch = data["epoch"]
                            accuracy = data["accuracy"]
                            loss = data["loss"]
                            self.epoch_label.setText(f"Epoch: {epoch}")
                            self.loss_label.setText(f"Loss: {loss:.4f}")
                            self.canvas.update_plot(epoch, accuracy, loss)
                            train_accuracy_history.append(accuracy)
                            train_loss_history.append(loss)
                            QApplication.processEvents()
                        elif "val_accuracy" in data and "test_accuracy" in data:
                            val_accuracy = data["val_accuracy"]
                            test_accuracy = data["test_accuracy"]
                            self.status_label.setText(
                                f"Status: Done. Val accuracy: {val_accuracy:.2f}%, Test accuracy: {test_accuracy:.2f}%"
                            )
                    except json.JSONDecodeError:
                        continue

            return_code = process.wait()
            if return_code != 0:
                error_output = process.stderr.read()
                self.status_label.setText("Status: Training failed")
                QMessageBox.critical(self, "Error", f"Training failed: {error_output}")
                return

            scaler_path = "scaler.bin"
            if not os.path.exists(scaler_path):
                raise FileNotFoundError("Scaler file not found")
            with open(scaler_path, "r") as f:
                scaler = json.load(f)
                self.min_vals = scaler["min_vals"]
                self.max_vals = scaler["max_vals"]

            self.test_data = random.sample(self.data, min(10, len(self.data)))

            self.pred_table.setRowCount(len(self.test_data))
            for row, point in enumerate(self.test_data):
                features = point.features
                result = subprocess.run(
                    [
                        RUST_BINARY, "--predict",
                        str(features[0]), str(features[1]), str(features[2]), str(features[3]),
                        str(features[4]), str(features[5]), str(features[6]), str(features[7]),
                        str(features[8]), str(features[9]), str(features[10]), str(features[11]),
                        str(features[12]), str(features[13])
                    ],
                    capture_output=True, text=True
                )
                if result.returncode != 0:
                    raise Exception(f"Prediction failed: {result.stderr}")
                try:
                    pred_result = json.loads(result.stdout)
                    pred = pred_result["prediction"]
                except json.JSONDecodeError as e:
                    raise Exception(f"Failed to parse prediction output: {result.stdout}")
                for i, feature in enumerate(features):
                    self.pred_table.setItem(row, i, QTableWidgetItem(f"{feature:.2f}"))
                self.pred_table.setItem(row, 14, QTableWidgetItem("Safe" if point.label else "Unsafe"))
                self.pred_table.setItem(row, 15, QTableWidgetItem("Safe" if pred else "Unsafe"))

            self.pred_table.resizeColumnsToContents()

        except Exception as e:
            self.status_label.setText(f"Status: Error")
            QMessageBox.critical(self, "Error", f"Error during training: {str(e)}")

    def make_prediction(self):
        try:
            if not os.path.exists(RUST_BINARY):
                raise FileNotFoundError(f"Rust binary not found at {RUST_BINARY}")
            if not os.path.exists("scaler.bin"):
                raise FileNotFoundError("Scaler file not found. Train the model first.")

            features = []
            for input_field in self.feature_inputs:
                value = input_field.text().strip()
                if not value:
                    raise ValueError(f"Please enter a value for {input_field.placeholderText()}")
                features.append(float(value))

            result = subprocess.run(
                [
                    RUST_BINARY, "--predict",
                    str(features[0]), str(features[1]), str(features[2]), str(features[3]),
                    str(features[4]), str(features[5]), str(features[6]), str(features[7]),
                    str(features[8]), str(features[9]), str(features[10]), str(features[11]),
                    str(features[12]), str(features[13])
                ],
                capture_output=True, text=True
            )
            if result.returncode != 0:
                raise Exception(f"Prediction failed: {result.stderr}")

            try:
                pred_result = json.loads(result.stdout)
                pred = pred_result["prediction"]
                self.predict_result.setText(f"Prediction: {'Safe' if pred else 'Unsafe'}")
            except json.JSONDecodeError as e:
                raise Exception(f"Failed to parse prediction output: {result.stdout}")

        except Exception as e:
            QMessageBox.critical(self, "Error", f"Prediction error: {str(e)}")
            self.predict_result.setText("Prediction: Error")

if __name__ == '__main__':
    app = QApplication(sys.argv)
    window = MainWindow()
    window.show()
    sys.exit(app.exec_())