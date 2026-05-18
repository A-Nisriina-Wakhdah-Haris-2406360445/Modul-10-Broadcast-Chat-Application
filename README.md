Nama: Nisriina Wakhdah Haris<br>
NPM: 2406360445<br>
Kelas: A<br>

<details>
<Summary><b>Reflection</b></Summary>

### 2.1 Original code of broadcast chat
- Gambar server
![Gambar server](/images/server.png)

- Gambar client 1
![Gambar client 1](/images/client%201.png)

- Gambar client 2
![Gambar client 2](/images/client%202.png)

- Gambar client 3
![Gambar client 3](/images/client%203.png)

#### Cara menjalankan aplikasi
1. Buka terminal pada folder project dan jalankan perintah `cargo run --bin server` untuk menjalankan server
2. Buka tiga terminal baru secara terpisah untuk mensimulasikan tiga user yang berbeda dan di setiap terminal tersebut jalankan `cargo run --bin client`. Jika berhasil, maka pada terminal server, terdapat log koneksi baru bertuliskan `New connection from 127.0.0.1:xxxxx`

#### Yang terjadi ketika mengetik pesan pada salah satu client
Ketika mengetik pesan pada salah satu terminal client dan menekan enter, fungsi `tokio::select!` di bagian `stdin.next_line()` akan menangkap input tersebut. Kemudian, client membungkus teks tersebut ke dalam `Message::text(text)` lalu mengirimkannya ke server lewat jaringan WebSocket. Setelah itu, server akan menerima pesan tersebut melalui fungsi `handle_connection` dan membaca teksnya. Lalu, membuat format baru yang menyertakan alamat IP dan Port asal pengirim pesan tersebut dan mencetak pesannya pada terminalnya sendiri. Server memasukkan pesan tersebut ke dalam channel broadcast `bcast_tx.send(full_msg)` sehingga pesannya otomatis disebarkan ke semua koneksi client yang sedang aktif karena menggunakan `tokio::sync::broadcast`. Di dalam server, task yang memegang koneksi client 1, 2, dan 3 mendengarkan channel broadcast melalui `bcast_rx.recv()` dan ketika pesan broadcast tiba, server langsung meneruskannya kembali ke aplikasi client masing-masing menggunakan `ws_stream.send()`. Di terminal setiap client, bagian `msg = ws_stream.next()` akan mendeteksi adanya pesan masuk dari server dan mencetaknya ke layar.

### 2.2 Modifying the websocket port
Ketika mengubah port aplikasi WebSocket, file yang harus diubah adalah `src/bin/server.rs` dan `src/bin/client.rs`. Hal ini dikarenakan, kedua sisi koneksi, yaitu client dan server, harus menggunakan host, port, dan protocol yang sama agar koneksi dapat berjalan dengan baik. Jika salah satunya berbeda, maka client akan gagal terhubung ke server karena alamat tujuan koneksi tidak sesuai. Pada sisi server, perubahan dilakukan pada bagian `TcpListener::bind("127.0.0.1:8080")` sedangkan pada sisi client, perubahan dilakukan pada bagian `Uri::from_static("ws://127.0.0.1:8080")`. Aplikasi ini menggunakan protokol WebSocket yang sama dan ditandai dengan awalan `ws://` pada URI. Prefix `ws://` menunjukkan bahwa koneksi menggunakan WebSocket tanpa enkripsi mirip seperti `http://` pada protokol HTTP
</details>