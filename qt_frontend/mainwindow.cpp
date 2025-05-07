#include "mainwindow.h"
#include "ui_mainwindow.h"
#include <QString>

extern "C" {
    double get_sin_from_lookup(double angle);
    double get_cos_from_lookup(double angle);
}

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent), ui(new Ui::MainWindow) {
    ui->setupUi(this);
    connect(ui->calculateButton, &QPushButton::clicked,
            this, &MainWindow::on_calculateButton_clicked);
}

MainWindow::~MainWindow() {
    delete ui;
}

void MainWindow::on_calculateButton_clicked() {
    double input = ui->inputField->text().toDouble();
    double sin_result = get_sin_from_lookup(input);
    double cos_result = get_cos_from_lookup(input);

    QString hasil = "Sin: " + QString::number(sin_result) +
                    "\nCos: " + QString::number(cos_result);

    ui->resultLabel->setText(hasil);
}
