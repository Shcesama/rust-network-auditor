mod scanner;
mod vulns;

use tokio::sync::Semaphore;
use std::net::IpAddr;
use std::sync::Arc;
use colored::*;
use futures::stream::{self, StreamExt};
use clap::Parser;

#[derive(Parser)]
#[command(name = "RustServiceAuditor", author = "Senin Adın", version = "1.1", about = "Profesyonel Ağ ve Servis Denetçisi")]
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
            eprintln!("{} Geçersiz IP adresi girdiniz!", "[!]".red());
            return;
        }
    };
    
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
                check_port_wrapper(target_ip, port, sem).await;
            }
        })
        .await;

    println!("\n{} Denetim başarıyla tamamlandı.", "[*]".blue().bold());
}

async fn check_port_wrapper(ip: IpAddr, port: u16, semaphore: Arc<Semaphore>) {
    let _permit = match semaphore.acquire().await {
        Ok(p) => p,
        Err(_) => return,
    };
    
    let addr = std::net::SocketAddr::new(ip, port);
    
    if let Ok(Ok(stream)) = tokio::time::timeout(tokio::time::Duration::from_millis(1000), tokio::net::TcpStream::connect(&addr)).await {
        println!("{} Port {:>5} {}", "[+]".green().bold(), port, "AÇIK".green());
        scanner::probe_service(stream, port).await;
    }
}