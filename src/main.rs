use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Semaphore;
use std::net::{IpAddr, SocketAddr};
use std::sync::Arc;
use colored::*;
use futures::stream::{self, StreamExt};
use clap::Parser;

#[derive(Parser)]
#[command(name = "RustAuditor", author = "Senin Adın", version = "1.1", about = "Profesyonel Ağ ve Zafiyet Denetçisi")]
struct Args {
    #[arg(short, long)]
    target: String,

    #[arg(short, long, default_value = "21,22,80,443,445,3306")]
    ports: String,

    #[arg(short, long, default_value = "100")]
    concurrent: usize, // Aynı anda kaç port taranacak?
}

// 9. MADDE: Gelişmiş FTP Anonim Giriş Analizi
async fn test_ftp_anonymous(mut stream: TcpStream) -> bool {
    let mut buffer = [0; 512];
    let _ = stream.write_all(b"USER anonymous\r\n").await;
    let _ = timeout(Duration::from_secs(1), stream.read(&mut buffer)).await;
    let _ = stream.write_all(b"PASS guest@example.com\r\n").await;
    
    if let Ok(Ok(n)) = timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
        let res = String::from_utf8_lossy(&buffer[..n]);
        return res.contains("230");
    }
    false
}

async fn probe_service(mut stream: TcpStream, port: u16) {
    let mut buffer = [0; 1024];
    if port == 80 { let _ = stream.write_all(b"HEAD / HTTP/1.0\r\n\r\n").await; }

    match timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
        Ok(Ok(n)) if n > 0 => {
            let banner = String::from_utf8_lossy(&buffer[..n]);
            println!("   {} Servis: {}", "-->".yellow(), banner.trim().replace("\r\n", " ").cyan());

            if port == 21 && (banner.contains("220") || banner.contains("FTP")) {
                print!("   {} FTP Anonim Giriş Deneniyor... ", "!!!".red().bold());
                if test_ftp_anonymous(stream).await {
                    println!("{}", "ERİŞİM SAĞLANDI! (KRİTİK ZAFİYET)".red().bold().blink());
                } else {
                    println!("{}", "Başarısız (Güvenli)".green());
                }
            }
        }
        _ => println!("   {} Yanıt alınamadı.", "-->".black().bold()),
    }
}

async fn check_port(ip: IpAddr, port: u16, semaphore: Arc<Semaphore>) {
    // Semaphore ile eşzamanlı bağlantı sayısını kontrol ediyoruz
    let _permit = semaphore.acquire().await.unwrap();
    let addr = SocketAddr::new(ip, port);
    
    if let Ok(Ok(stream)) = timeout(Duration::from_millis(1000), TcpStream::connect(&addr)).await {
        println!("{} Port {:>5} {}", "[+]".green().bold(), port, "AÇIK".green());
        probe_service(stream, port).await;
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let target_ip: IpAddr = args.target.parse().expect("Geçersiz IP adresi!");
    
    // Port Aralığı Desteği (Örn: 1-100 veya 22,80)
    let mut port_list = Vec::new();
    for part in args.ports.split(',') {
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            if range.len() == 2 {
                let start: u16 = range[0].parse().unwrap_or(0);
                let end: u16 = range[1].parse().unwrap_or(0);
                for p in start..=end { port_list.push(p); }
            }
        } else if let Ok(p) = part.trim().parse::<u16>() {
            port_list.push(p);
        }
    }

    println!("{} {} üzerinde denetim başlatıldı (Eşzamanlılık: {}).", "[*]".blue().bold(), target_ip, args.concurrent);
    
    let semaphore = Arc::new(Semaphore::new(args.concurrent));
    
    stream::iter(port_list)
        .for_each_concurrent(None, |port| {
            let sem = Arc::clone(&semaphore);
            async move {
                check_port(target_ip, port, sem).await;
            }
        })
        .await;

    println!("\n{} Denetim başarıyla tamamlandı.", "[*]".blue().bold());
}