## Anggota Kelompok:
1. Hardito Yussyachtio Rahmad ( 2042221092 )
2. Farah Sabrina Amalina ( 2042221098 )
3. Adrian Yared Immanuel ( 2042221080 )

## Supervisor:
Ahmad Radhy, S.Si., M.Si

### Departemen Teknik Instrumentasi - Institut Teknologi Sepuluh Nopember
---

# Deskripsi Project

Proyek ini mengembangkan aplikasi pembelajaran mesin dan jaringan saraf tiruan (neural network/NN) dengan perhitungan fungsi trigonometri, menggunakan Rust sebagai backend dan PyQt sebagai frontend. Proyek ini mencakup program untuk menghitung sinus dan kosinus, pembelajaran mesin dengan algoritma SVM dan kNN, serta jaringan saraf untuk pemodelan prediktif, yang diintegrasikan dengan antarmuka grafis.

## Kebutuhan Instalasi

- **Rust**: Bahasa pemrograman untuk pengembangan backend.
- **Qt (PyQt)**: Kerangka kerja untuk membangun antarmuka grafis (frontend).
- **Ubuntu Linux**: Sistem operasi berbasis Linux untuk pengembangan dan peluncuran.

## Fitur Proyek

### 1. Program Perhitungan Sinus dan Kosinus
- **Taylor Series**:
  - Menghitung nilai fungsi trigonometri dengan pendekatan melalui beberapa iterasi tertentu.
- **Lookup Table**:
  - Mengambil nilai sinus dan kosinus dari tabel yang sudah berisi data, tanpa perhitungan ulang.

### 2. Program Pembelajaran Mesin (SVM dan kNN)
- **Fungsi**:
  - Membaca data dari file CSV.
  - Mengolah data agar siap digunakan (preprocessing).
  - Melatih model Support Vector Machine (SVM) dan k-Nearest Neighbors (kNN).
  - Menampilkan akurasi model dan visualisasi dalam bentuk grafik.
  - Menampilkan hasil prediksi melalui antarmuka grafis.
- **Aplikasi**:
  - Menganalisis dataset, seperti pengujian kualitas air.

### 3. Program Jaringan Saraf Tiruan (NN) Berbasis Rust
- **Persiapan Dataset**:
  - Menggunakan dataset dalam format CSV sesuai tema proyek.
- **Arsitektur NN**:
  - Merancang lapisan input, hidden layer, dan output.
- **Pengolahan Data**:
  - Melakukan normalisasi data dan membaginya menjadi data latih dan data uji.
- **Pelatihan dan Evaluasi**:
  - Melatih model NN menggunakan data latih.
  - Menguji model dengan data uji untuk mengevaluasi performa.
- **Keluaran**:
  - Menampilkan hasil prediksi dan akurasi model.

### 4. Integrasi Backend dan Frontend
- **Backend (Rust)**:
  - Membuat struktur proyek dengan perintah `cargo init`.
  - Menulis fungsi untuk mengelola logika dan melatih model.
- **Frontend (PyQt)**:
  - Merancang antarmuka grafis pengguna.
- **Komunikasi**:
  - Mengatur interaksi antara backend Rust dan frontend PyQt.
- **Pengujian dan Perbaikan**:
  - Memastikan alur data antara input dan output berjalan lancar.
  - Memperbaiki bug dan menyesuaikan antarmuka.

## Hasil

- **Perhitungan Trigonometri**:
  - Teknik lookup table lebih unggul dibandingkan Taylor series untuk sudut tertentu karena efisiensinya.
- **Analisis Kualitas Air**:
  - Model SVM dan kNN mencapai akurasi klasifikasi di atas 99%, tetapi distribusi data tidak merata (kategori aman vs. tidak aman) karena ketidakseimbangan dataset.
- **Neural Network**:
  - Proses pelatihan berjalan optimal dalam 1000 epoch dengan konvergensi stabil, meskipun peningkatan lebih lanjut memberikan dampak kecil.
  - Mencapai akurasi 98% pada dataset pengujian kualitas air menggunakan antarmuka Qt.
- **Kesimpulan**:
  - Performa sangat dipengaruhi oleh metode dan karakteristik dataset. Mengatasi ketidakseimbangan data dan mengoptimalkan epoch pelatihan dapat meningkatkan hasil.

## Cara Memulai

1. Instal Rust, PyQt, dan siapkan Ubuntu Linux.
2. Klon repositori dan masuk ke direktori proyek.
3. Jalankan `cargo init` untuk mengatur backend Rust.
4. Instal dependensi PyQt untuk frontend.
5. Jalankan proyek untuk memverifikasi komunikasi backend-frontend.
6. Lakukan perbaikan bug dan penyesuaian sesuai kebutuhan.
