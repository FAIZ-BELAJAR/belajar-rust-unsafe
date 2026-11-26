use std::{thread, time::Duration};

fn main() {
    println!("Hello, world!");
}

// Unsafe 
// Semua kode yang kita tulis di rust pada dasarnya mengikuti aturan memory safety dan dijamin keamanan nya pada saat di compile
// Tapi jika kita ingin melepas aturan memory safety rust dan jaminan keamanan memory oleh rust, kita bisa menggunakan unsafe 
// Unsafe dibuat karna 2 alasan
// 1. Analisis statis keamanan memori rust pada dasarnya bersifat konservatif, kita tetap tidak akan bisa menjalankan kode program walau secara logika dan kemauan kita benar, hanya karna kita tidak bisa membuktikan keamanan memori nya pada rust, walau pada perhitungan kita, kita tau kalau program itu aman secara memory
// 2. Pada dasarnya hardware dibawah komputer itu unsafe, seperti pemrograman os low level, embedded mikrokontroller dll, 
// Untuk menggunakan unsafe rust kita hanya perlu menggunakan "unsafe" keyword
// Sebenarnya `unsafe` tidak "menghapus" memory safety secara total, melainkan memindahkan tanggung jawab pembuktian keamanan memori (dan menghindari undefined behavior / UB) dari compiler ke programmer.
// Jika terjadi pelanggaran invarian memori di dalam blok `unsafe` (seperti dangling pointer, data race, atau nilai invalid pada tipe data), perilaku program menjadi Undefined Behavior (UB) — bisa crash, menghasilkan nilai acak, atau tampak normal saat diuji tapi gagal di kondisi lain.
// Praktik terbaiknya: bungkus operasi `unsafe` seminimal mungkin dan ekspos API publik yang safe jika invarian keamanannya bisa dijamin secara ketat.

// Tapi perlu diketahui unsafe tidaklah menonaktifkan borrow checker rust atau membatalkan rust safety check
// Rust hanya memberikan keluluasaan pada 6 aspek
// Dereference Raw Pointer, Call an unsafe function or method, Calling another method or function from another languange, Access atau Modify Mutable Static Variable, Implement an unsafe trait, Access field of unions
// Borrow checker dan aturan kepemilikan/lifetime tetap aktif di dalam blok `unsafe` untuk referensi aman (`&T` dan `&mut T`).
// Di luar 6 kemampuan khusus ini, aturan pengecekan keamanan Rust tetap berlaku penuh.

// Dereference Raw Pointer
// Ada 2 tipe raw pointer 
// Mutable Raw Pointer dan Immutable Raw Pointer
// Baik keduanya tidak menjamin bahwa alamat memori yang dituju itu valid
// dangling pointer bisa saja terjadi
// Penulisan resminya di Rust adalah `*const T` (immutable/constant raw pointer) dan `*mut T` (mutable raw pointer), dengan `*` sebagai bagian dari nama tipe.
// Keunggulan raw pointer dibanding referensi biasa: boleh mengabaikan aturan aliasing/borrowing (misalnya `*const T` dan `*mut T` menunjuk alamat yang sama), boleh bernilai null/dangling, dan tidak otomatis menjalankan `Drop`.
// Membuat raw pointer (`as *const T`, `as *mut T`, `&raw const ...`, `&raw mut ...`) itu safe, tetapi me-dereference-nya (`*r1`, `*r2`) wajib di dalam blok `unsafe` karena alamatnya bisa tidak valid atau tidak selaras (unaligned).

#[test]
fn dereference_unsafe() {
    let mut num: i32 = 5;

    unsafe{
        let r1: *const i32  = &num as *const i32; // immutable raw pointer 
        let r2: *mut i32 = &mut num as *mut i32; // mutable raw pointer
        
         println!("r1 pertama: {:?}", *r1);
    
         
        *r2 = 110; 
    
        println!("r1 kedua: {:?}", *r1);// -> kita akan melihat bahwa disini akan berubah nilainya

        println!("{:?}", r2);// -> hasilnya 2 2 nya baik r1 atau r2 menunjuk ke alamat memori yang sama 
        // Pada baris `println!("{:?}", r2);`, yang dicetak adalah alamat memori (pointer) dari `r2`, bukan nilai `110` (karena tidak di-dereference dengan `*r2`). Mencampur akses referensi aman (`num`) dan raw pointer mutable (`*r2`) pada variabel yang sama juga perlu hati-hati agar tidak memicu UB terkait aliasing/mutabilitas.
    }
}

// Call an unsafe function or method
// Kita bisa memanggil unsafe functionn atau method didalam block unsafe
// Unsafe function atau method hanya bisa dipanggil di dalam block unsafe
// Menandai fungsi/method dengan `unsafe fn` berarti fungsi tersebut memiliki kontrak/syarat (precondition) yang tidak bisa dicek otomatis oleh compiler—misalnya pemanggil wajib memastikan pointer valid, slice terurut, atau thread-safety terjaga.
// Menulis `unsafe fn` tidak otomatis menjalankan isinya dalam mode `unsafe`; di dalam body `unsafe fn`, kamu tetap butuh blok `unsafe` jika melakukan operasi yang termasuk ke dalam 6 kemampuan `unsafe` (kecuali pada Rust edition 2024+ pada blok `unsafe extern`).

#[test]
fn feature() {
    unsafe fn dangerous(){}

    unsafe{
        dangerous();
    }
}

// Calling another method or function from another languange
// Kadang kita mungkin membutuhkan interaksi atau keterlibatan fungsi fungsi dari bahasa pemrograman lain
// Contohnya di pemrograman mikrokontroller, sampai saat ini kebanyakan kode ditulis dalam c atau c++
// Kita bisa memanggil dan menggunakan fungsi dari bahasa lain menggunakan unsafe
// Ini dikenal sebagai FFI (Foreign Function Interface). Compiler Rust tidak bisa memverifikasi keamanan memori di dalam kode bahasa lain (seperti C/C++), sehingga pemanggilan fungsi `extern` dianggap `unsafe`.
// Pada Rust modern (edition 2024 / blok `unsafe extern`), blok `unsafe extern "C"` dan deklarasi `unsafe fn` di dalamnya menegaskan kontrak `unsafe` saat fungsi FFI dipanggil. Pastikan juga tipe data parameter dan return value kompatibel secara ABI.

unsafe extern "C" {
    unsafe fn abs(input: i32) -> i32;
}

#[test]
fn memanggil_fungsi_c() {
    unsafe {
        println!("Memanggil fungsi dari bahasa C: {}", abs(-3))
    }
}

// Access atau Modify Mutable Static Variable
// Di unsafe kita diberikan kemambuat untuk mengubah data static
// Variabel `static` memiliki alamat memori tetap selama program berjalan (lifetime `'static`), berbeda dengan `const` yang di-inline/disalin saat kompilasi.
// Membaca maupun mengubah `static mut` bersifat `unsafe` karena mudah menyebabkan *data race* ketika diakses dari banyak thread tanpa sinkronisasi (seperti `Mutex` atau `Atomic*`).
// Pada Rust modern (terutama edition 2024), mengambil referensi langsung (`&NAMA` atau `&mut NAMA`) ke `static mut` berisiko tinggi memicu Undefined Behavior; penggunaan `&raw mut NAMA` sudah sesuai untuk mendapatkan raw pointer tanpa membuat referensi `&mut` yang aliasing.
// Baris `let namanya_ptr: *mut str = NAMA as *const str as *mut str;` pada tipe `&str` (yang merupakan *fat pointer* berisi pointer data + panjang) sebenarnya tidak mengubah `NAMA` menjadi `"Mantap"`—nilai `NAMA` sudah berubah pada baris `*nama_ptr = "Mantap";`. Untuk melihat isi terbaru `NAMA` langsung, kamu bisa mencetak `NAMA` di dalam blok `unsafe`.

static mut NAMA: &str = "Halo dunia";

#[test]
fn mengubah_static() {
    unsafe {
        // Mengambil raw pointer ke variabel static mut NAMA (bertipe &str)
        let nama_ptr: *mut &str = &raw mut NAMA;
        println!("Pointer: {:?}", nama_ptr);

        // Mengganti nilai &str yang ditunjuk oleh nama_ptr
        *nama_ptr = "Mantap";
        let namanya_ptr: *mut str = NAMA as *const str as *mut str;
        println!("isinya: {:?}", &*namanya_ptr);
        // Nilai `NAMA` sudah diperbarui lewat `*nama_ptr = "Mantap"`; bisa juga dicek langsung via `println!("NAMA sekarang: {}", NAMA);` di dalam blok `unsafe`.
    }
}

// Implement an unsafe trait
// Kita juga di perbolehkan untuk menggunakan trait unsafe pada unsafe block
// Trait ditandai `unsafe trait` ketika implementasinya harus memenuhi invarian tertentu yang tidak bisa diperiksa compiler, dan kode lain yang safe bergantung pada kebenaran invarian tersebut.
// Contoh klasik di standard library adalah `Send` dan `Sync`: implementasi yang keliru pada tipe yang tidak thread-safe dapat memicu *data race* pada kode safe sekalipun.
// Karena itu, mengimplementasikan `unsafe trait` memerlukan `unsafe impl` untuk menyatakan bahwa syarat keamanannya sudah dipenuhi.

unsafe trait Danger {

}

unsafe impl Danger for i32 {

}

// Access field of unions
// `union` di Rust mirip dengan `union` di C: semua field berbagi alamat memori yang sama, dan ukurannya mengikuti field terbesar.
// Menulis ke salah satu field (`nilai.pecahan = 3.19;`) adalah operasi safe karena sekadar menimpa byte di memori.
// Membaca field dari `union` bersifat `unsafe` karena compiler tidak melacak field mana yang terakhir ditulis/aktif. Jika pola bit di memori tidak valid untuk tipe field yang dibaca (misalnya `bool`, `char`, atau referensi), membaca field tersebut dapat menyebabkan Undefined Behavior.
// Pada contoh di bawah, `i32` dan `f32` sama-sama 32-bit dan menerima sembarang pola bit 32-bit, sehingga membaca `nilai.bulat` setelah menulis `nilai.pecahan` akan menampilkan representasi bit mentah (`raw bits`) dari `3.19_f32` sebagai `i32`, bukan hasil konversi angka `3.19`.

union Angka {
    bulat: i32, 
    pecahan: f32 
}

#[test]
fn test_mengakses_field_union(){
    let mut nilai = Angka {bulat: 42};

    nilai.pecahan = 3.19;

    unsafe {
        println!("Sebagai pecahan: {}", nilai.pecahan);
        println!("Sebagai bulat: {}", nilai.bulat);
    }
}