use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use std::net::{IpAddr, SocketAddr};
use colored::*;
use futures::stream::{self, StreamExt}; // Çoklu işlem yönetimi için

async fn check_port(ip: IpAddr, port: u16) {
    let addr = SocketAddr::new(ip, port);
    let timeout_duration = Duration::from_millis(800);

    match timeout(timeout_duration, TcpStream::connect(&addr)).await {
        Ok(Ok(_stream)) => {
            println!("{} Port {:>5} {}", "[+]".green().bold(), port, "AÇIK".green());
        }
        _ => {} // Kapalı veya zaman aşımı olanları ekrana basıp kalabalık yapmıyoruz
    }
}

#[tokio::main]
async fn main() {
    let target_ip: IpAddr = "127.0.0.1".parse().expect("Geçersiz IP adresi");
    let ports = 1..1001; // 1 ile 1000 arasındaki tüm portlar
    
    println!("{} {} üzerinde geniş tarama başlıyor...", "[*]".blue(), target_ip);
    println!("{} {} port taranacak.", "[i]".yellow(), ports.len());

    // Eşzamanlılık sınırı: Aynı anda en fazla 100 bağlantı dene (Sistemi yormamak için)
    let concurrent_limit = 100;

    stream::iter(ports)
        .for_each_concurrent(concurrent_limit, |port| async move {
            check_port(target_ip, port).await;
        })
        .await;

    println!("{} Tarama tamamlandı.", "[*]".blue());
}