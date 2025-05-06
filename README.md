1. Instalasi Kebutuhan Projek
   `   Rust, sebagai bahasa pemrograman (back end)
   Qt, sebagai interface (front end)
   Ubuntu Linux, sebagai sistem operasi berbasis Linux

2. a) Program Sine dan Cosine dengan Metode Taylor Series dan Lookup Table
       Taylor Series: Menghitung nilai mendekati fungsi trigonometrik dengan beberapa suku (jumlah iterasi tertentu).
       Lookup Table: Mengambil nilai dari tabel yang sudah berisi data sin dan cos tanpa perhitungan ulang.
   b) Program Machine Learning dengan algoritma SVM dan kNN
       Membaca data dari file CSV.
       Mengolah data agar siap digunakan (preprocessing).
       Melakukan training model SVM dan kNN.
       Melihat hasil akurasi dan visualisasinya dalam grafik.
       Menampilkan prediksi dalam bentuk antarmuka grafis.

3. Program Rust Neural Network (NN) Algorithm
   Menyiapkan dataset dalam format CSV sesuai dengan tema.
   Merancang arsitektur NN meliputi lapisan input, hidden layer, dan output.
   Memproses data dengan melakukan normalisasi dan membaginya menjadi data training dan data testing.
   Melalukan training model NN menggunakan data latih yang telah tersedia.
   Melakukan testing model dengan data testing untuk mengevaluasi performanya.
   Menampilkan hasil prediksi dan akurasi model sebagai output dari program.

4. Integrasi NN dengan Menggunakan rust Sebagai Back End dan PyQT Sebagai Front End
   Membuat folder untuk proyek dan menjalankan perintah cargo init untuk membentuk struktur dasar proyek back end.
   membuat fungsi sebagai pengelola logika.
   Menulis fungsi untuk training model menggunakan dataset.
   Membuat tampilan GUI pada PyQt sebagai front end
   Mengatur komunikasi antara backend dan frontend
   Menjalankan proyek untuk memastikan fungsi Rust dapat dipanggil dari PyQt dan data input serta output berjalan.
   Melakukan perbaikan terhadap bug dan error yang ada, serta melakukan penyesuaian pada interface.

Hasil:
Hasil dari program Taylor Series dan Lookup Table dilakukan pegujian akurasi dua pendekatan berbeda dalam menghitung nilai sinus dan cosinus. Pada pengujian, dimasukkan nilai sudut 45° dan didapatkan hasil dari taylor series (nilai sin dan cosinus 0.7071067811865475), sedangkan untuk hasil lookup table (nilai sin 0.7071067811865475 dan cosinus 0.7071067811865476). Kedua metode memberikan hasil untuk sudut 45° yang hampir sama dan hanya berbeda digit terakhirnya untuk perhitungan cosinus. Untuk sudut 90° didapatkan hasil taylor series (nilai sin 1 dan cosinus -0.00000000000033376816414274305), sedangkan hasil lookup table (nilai sin 1 dan cosinus 0.000000000000000123233995736766). Kedua metode mampu menghitung nilai sinus secara tepat tetapi dalam perhitungan cosinus, metode lookup table menunjukkan ketepatan yang lebih tinggi dengan margin error yang jauh lebih kecil. Dan untuk sudut 135° didapatkan hasil taylor series (nilai sin 0.7071067811852791 dan cosinus -0.7071067811878429), sedangkan hasil lookup table (nilai sin 0.7071067811865476 dan cosinus -0.7071067811865475). Pada sudut ini, metode lookup table lebih unggul dengan hasil yang lebih mendekati nilai eksak, sementara taylor series mulai menunjukkan deviasi yang cukup.
Hasil dari program SVM dan kNN algorithm yaitu digunakan parameter untuk memantau kualitas air diantaranya aluminium, ammonia, arsenic, barium, cadmium, chloramine, chromium, copper yang ditampilkan berdasarkan dataset. Dataset ini sebagai pembeda untuk katergori kualitas air yang baik (safe) dan kualitas air yang buruk (unsafe). Jumlah dataset tidak seiumbang dengan jumlah data unsafe 7950 dan jumlah data safe hanya 49. Hasil dari SVM dan kNN menunjukkan akurasi tinggi yaitu pada SVM 99,13% dam kNN 99,31% yang kemungkinan terjadi karena pengaruh ketidakseimbangan data, sehingga hasil akurasi tersebut belum tentu menunjukkan peforma klasifikasi yang seimbang dan tepat. Pada hasil visualisasi decision boundary pada kNN menunjukkan bahwa hanya sebagian kecil area berwarna hijau dan menyebar yang diklasifikasikan sebagai safe, sedangkan sebagian besarnya adalah kategori unsafe. Hal ini sama dengan hasil visualisasi decision boundary pada SVM.
Hasil dari program Rust Neural Network (NN) algorithm yaitu dilakukan proses training sebanyak 1000 epoch dengan model menunjukkan tren penurunan loss yang konsisten dari 440.11 menjadi 410.49 dengan peningkatan akurasi dari 96.9% ke 97.14%. Nilai loss yang terus menurun dan akurasi yang stabil di atas 97% menandakan bahwa proses training berjalan efektif dan model mendekati konvergensi yaitu tembahan epoch hanya memberikan sedikit peningkatan terhadap peforma. Dalam proses training, tidak ditemukan gejala overfitting karena tidak ada fluktuasi besar pada loss maupun akurasi.
Hasil dari program integrasi NN dan PyQt yaitu tampilan GUI training dari dataset waterquality1.csv pada PyQt menunjukkan model bekerja dengan sangat baik. Akurasi model mengalami kenaikan akurasi (grafik merah) hingga mencapai titik stabil di angka 98%. Hal ini menunjukan bahwa model mempelajari data dengan cepat dan dapat mempertahankan kemampuannya dengan baik. Sementara itu, grafik loss atau kesalahan (grafik biru) juga menunjukkan penurunan mendekati nol, yang berarti prediksi model semakin akurat. Walaupun ada sedikit kenaikan loss di sekitar epoch ke-400, hal tersebut tidak memengaruhi model karena kembali stabil setelahnya. Hasil akurasi pada data validasi dan data uji juga tinggi, lebih dari 98%, yang menandakan model bisa mengenali data baru dengan baik. 

   
    
   
   
   
   
   
