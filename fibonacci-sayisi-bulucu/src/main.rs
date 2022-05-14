//Bismillahirrahmanirrahim.

//language: Turkish. This is a program that finds nth Fibonacci number.
//Bu, n. Fibonacci sayısını bulan bir programdır.

use std::io;

fn main() {
    println!(
        "Fibonacci sayısı bulucu programa hoş geldiniz! 
Çıkmak için \"çık\" yazabilirsiniz."
    );
    'ana: loop {
        let mut esk = 0;
        let mut bu = 1;
        let mut n: usize;
        loop {
            let mut giriş = String::new();
            println!("Kaçıncı Fibonacci sayısını bulmak istiyorsunuz?");
            io::stdin().read_line(&mut giriş).expect("Giriş okunamadı.");
            if giriş.trim() == "çık" {
                println!("Güle güle efendim!");
                break 'ana;
            }
            n = giriş.trim().parse().expect("Giriş sayı değil");
            if (n > 0) & (n < 46) {
                break;
            } else {
                println!("Sadece 1.'den 45.'ye kadar olan Fibonacci sayıları bulunabilir.");
            }
        }
        for _i in 0..n - 1 {
            bu += esk;
            esk = bu - esk;
        }
        println!("{}. Fibonacci sayısı: {}", n, esk);
    }
}