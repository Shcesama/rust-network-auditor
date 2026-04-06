mod core;

use chrono::Local;
use clap::Parser;
use colored::*;
use futures::stream::{self, StreamExt};
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::net::IpAddr;
use std::sync::{Arc, Mutex}; // Mutex ekledik
use tokio::sync::Semaphore;

#[derive(Serialize, Clone)]

struct ScanResult {
    port: u16,
    status: String,
}

#[derive(Serialize)]
struct FinalReport {
    target: String,
    scan_time: String,
    results: Vec<ScanResult>,
}

#[derive(Parser)]
#[command(
    name = "RustServiceAuditor",
    author = "Batuhan Seydi Çelik",
    version = "1.1",
    about = "Profesyonel Ağ ve Servis Denetçisi"
)]
struct Args {
    #[arg(short, long)]
    target: String,
    #[arg(short, long, default_value = "21,22,80,443,445,3306")]
    ports: String,
    #[arg(short, long, default_value = "100")]
    concurrent: usize,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let target_ip: IpAddr = match args.target.parse() {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("{} Geçersiz IP adresi!", "[!]".red());
            return;
        }
    };

    let port_list = parse_ports(&args.ports);

    // SONUÇLARI TOPLAYACAK GÜVENLİ LİSTE
    let scan_results = Arc::new(Mutex::new(Vec::new()));
    let semaphore = Arc::new(Semaphore::new(args.concurrent));

    println!(
        "{} {} üzerinde denetim başlatıldı...",
        "[*]".blue().bold(),
        target_ip
    );

    stream::iter(port_list)
        .for_each_concurrent(None, |port| {
            let sem = Arc::clone(&semaphore);
            let res_clone = Arc::clone(&scan_results); // Listeyi kopyala
            async move {
                check_port_wrapper(target_ip, port, sem, res_clone).await;
            }
        })
        .await;

    // --- RAPORLAMA KISMI BURADA (Döngü bittikten sonra bir kez) ---
    let final_results = scan_results.lock().unwrap();
    let report = FinalReport {
        target: target_ip.to_string(),
        scan_time: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(),
        results: final_results.clone(),
    };

    let json_data = match serde_json::to_string_pretty(&report) {
        Ok(veri) => veri,
        Err(hata) => {
            eprintln!("JSON dönüştürme hatası: {}", hata);
            return;
        }
    };

    let mut file = match File::create("scan_report.json") {
        Ok(dosya) => dosya,
        Err(hata) => {
            eprintln!("Dosya oluşturma hatası: {}", hata);
            return;
        }
    };

    match file.write_all(json_data.as_bytes()) {
        Ok(_) => println!(
            "\n{} Denetim bitti. Rapor: {}",
            "[*]".blue().bold(),
            "scan_report.json".cyan()
        ),
        Err(hata) => eprintln!("Dosyaya yazma hatası: {}", hata),
    }
}

async fn check_port_wrapper(
    ip: IpAddr,
    port: u16,
    semaphore: Arc<Semaphore>,
    results: Arc<Mutex<Vec<ScanResult>>>,
) {
    let _permit = match semaphore.acquire().await {
        Ok(p) => p,
        Err(_) => return,
    };

    let addr = std::net::SocketAddr::new(ip, port);
    if let Ok(Ok(stream)) = tokio::time::timeout(
        tokio::time::Duration::from_millis(1000),
        tokio::net::TcpStream::connect(&addr),
    )
    .await
    {
        println!(
            "{} Port {:>5} {}",
            "[+]".green().bold(),
            port,
            "AÇIK".green()
        );

        // Sonucu listeye ekle
        {
            let mut res = results.lock().unwrap();
            res.push(ScanResult {
                port,
                status: "Açık".to_string(),
            });
        }

        core::scanner::probe_service(stream, port).await;
    }
}
pub fn parse_ports(ports_input: &str) -> Vec<u16> {
    let mut port_list = Vec::new();
    for part in ports_input.split(',') {
        if part.contains('-') {
            let range: Vec<&str> = part.split('-').collect();
            if range.len() == 2 {
                let start: u16 = range[0].parse().unwrap_or(0);
                let end: u16 = range[1].parse().unwrap_or(0);
                if start > 0 && end >= start {
                    for p in start..=end {
                        port_list.push(p);
                    }
                }
            }
        } else if let Ok(p) = part.trim().parse::<u16>() {
            port_list.push(p);
        }
    }
    port_list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tekil_degerler() {
        let sonuc = parse_ports("80,443");
        assert_eq!(sonuc, vec![80, 443]);
    }

    #[test]
    fn test_aralik_degerleri() {
        let sonuc = parse_ports("20-22");
        assert_eq!(sonuc, vec![20, 21, 22]);
    }

    #[test]
    fn test_karisik_degerler() {
        let sonuc = parse_ports("80,90-92,443");
        assert_eq!(sonuc, vec![80, 90, 91, 92, 443]);
    }
}
