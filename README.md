1. Instalasi Kebutuhan Projek
   Rust, sebagai bahasa pemrograman (back end)
   Qt, sebagai interface (front end)
   Ubuntu Linux, sebagai sistem operasi berbasis Linux

3. a) Program Sine dan Cosine dengan Metode Taylor Series dan Lookup Table
       Taylor Series: Menghitung nilai mendekati fungsi trigonometrik dengan beberapa suku (jumlah iterasi tertentu).
       Lookup Table: Mengambil nilai dari tabel yang sudah berisi data sin dan cos tanpa perhitungan ulang.
   b) Program Machine Learning dengan algoritma SVM dan kNN
       Membaca data dari file CSV.
       Mengolah data agar siap digunakan (preprocessing).
       Melakukan training model SVM dan kNN.
       Melihat hasil akurasi dan visualisasinya dalam grafik.
       Menampilkan prediksi dalam bentuk antarmuka grafis.

4. Program Rust Neural Network (NN) Algorithm
   Menyiapkan dataset dalam format CSV sesuai dengan tema.
   Merancang arsitektur NN meliputi lapisan input, hidden layer, dan output.
   Memproses data dengan melakukan normalisasi dan membaginya menjadi data training dan data testing.
   Melalukan training model NN menggunakan data latih yang telah tersedia.
   Melakukan testing model dengan data testing untuk mengevaluasi performanya.
   Menampilkan hasil prediksi dan akurasi model sebagai output dari program.

5. Integrasi NN dengan Menggunakan rust Sebagai Back End dan PyQT Sebagai Front End
   Membuat folder untuk proyek dan menjalankan perintah cargo init untuk membentuk struktur dasar proyek back end.
   membuat fungsi sebagai pengelola logika.
   Menulis fungsi untuk training model menggunakan dataset.
   Membuat tampilan GUI pada PyQt sebagai front end
   Mengatur komunikasi antara backend dan frontend
   Menjalankan proyek untuk memastikan fungsi Rust dapat dipanggil dari PyQt dan data input serta output berjalan.
   Melakukan perbaikan terhadap bug dan error yang ada, serta melakukan penyesuaian pada interface.

Hasil:
Berdasarkan hasil dari percobaan, dapat disimpulkan bahwa hasil sangat dipengaruhi oleh metode dan karakteristik data pada dataset yang digunakan. Pada perhitungan fungsi trigonometri, teknik lookup table lebih unggul dibandingkan taylor series khususnya untuk sudut-sudut tertentu. Dalam analisis kualitas air, meskipun menghasilkan akurasi klasifikasi di atas 99% untuk metode SVM dan kNN, hasil menunjukkan distribusi data yang tidak merata antara kategori safe dan unsafe karena ketidakseimbangan data. Proses training NN berjalan optimal dalam 1000 epoch dengan tren konvergensi yang stabil, meskipun peningkatan lebih lanjut memberikan dampak marginal. Sementara itu, implementasi pada dataset water quality testing menggunakan QT mencapai akurasi 98%.

   
    
   
   
   
   
   
