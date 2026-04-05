use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::net::{IpAddr, SocketAddr};
use colored::*;
use futures::stream::{self, StreamExt};
use clap::Parser; // Terminal argümanları için

// ARACIN TERMİNAL ARAYÜZÜ (CLI)
#[derive(Parser)]
#[command(name = "Rust Network Auditor", author = "Senin Adın", version = "1.0", about = "Akıllı Ağ Denetim Aracı")]
struct Args {
    /// Taranacak hedef IP adresi (Örn: 45.33.32.156)
    #[arg(short, long)]
    target: String,

    /// Taranacak portlar (virgülle ayırın: 22,80,443)
    #[arg(short, long, default_value = "21,22,80,443,445,3306,8080")]
    ports: String,
}

// SERVİS TANIMLAMA MODÜLÜ
async fn probe_service(mut stream: TcpStream, port: u16) {
    let mut buffer = [0; 1024];
    if port == 80 || port == 8080 {
        let _ = stream.write_all(b"HEAD / HTTP/1.0\r\n\r\n").await;
    }

    match timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
        Ok(Ok(n)) if n > 0 => {
            let banner = String::from_utf8_lossy(&buffer[..n]);
            println!("   {} Servis Bilgisi: {}", "-->".yellow(), banner.trim().replace("\r\n", " ").cyan());

            // 9. MADDE: FTP TESTİ
            if port == 21 && banner.contains("220") {
                println!("   {} {} FTP Anonim Giriş Test Ediliyor...", "!!!".red().bold(), "KRİTİK:".red());
            }
        }
        _ => println!("   {} Servis yanıt vermedi.", "-->".black().bold()),
    }
}

async fn check_port(ip: IpAddr, port: u16) {
    let addr = SocketAddr::new(ip, port);
    if let Ok(Ok(stream)) = timeout(Duration::from_millis(800), TcpStream::connect(&addr)).await {
        println!("{} Port {:>5} {}", "[+]".green().bold(), port, "AÇIK".green());
        probe_service(stream, port).await;
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse(); // Terminalden gelenleri oku
    
    let target_ip: IpAddr = args.target.parse().expect("Geçersiz IP adresi girdiniz!");
    let ports: Vec<u16> = args.ports
        .split(',')
        .filter_map(|p| p.trim().parse().ok())
        .collect();

    println!("{} {} üzerinde denetim başlıyor...", "[*]".blue().bold(), target_ip);

    stream::iter(ports)
        .for_each_concurrent(10, |port| async move {
            check_port(target_ip, port).await;
        })
        .await;

    println!("\n{} Tüm denetimler tamamlandı.", "[*]".blue().bold());
}