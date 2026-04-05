use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use colored::*;

pub async fn probe_service(mut stream: TcpStream, port: u16) {
    let mut buffer = [0; 1024];

    // Web portları için önceden istek gönder
    let web_ports = [80, 443, 8080, 8443];
    if web_ports.contains(&port) {
        let _ = stream.write_all(b"HEAD / HTTP/1.0\r\n\r\n").await;
    }

    match timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
        Ok(Ok(n)) if n > 0 => {
            let banner = String::from_utf8_lossy(&buffer[..n]);
            println!("    {} Servis Mesajı: {}", "-->".yellow(), banner.trim().replace("\r\n", " ").cyan());

            // FTP Zafiyet Kontrolü (vulns modülüne gidiyor)
            if port == 21 || banner.contains("FTP") {
                print!("    {} FTP Anonim Giriş Deneniyor... ", "!!!".red().bold());
                if crate::vulns::test_ftp_anonymous(stream).await {
                    println!("{}", "ERİŞİM SAĞLANDI! (KRİTİK ZAFİYET)".red().bold().blink());
                } else {
                    println!("{}", "Başarısız (Güvenli)".green());
                }
            }
        }
        _ => {
            // Yanıt gelmezse gri renkli bir bilgi mesajı yazdır
            println!("    {} Yanıt alınamadı (Zaman Aşımı).", "-->".truecolor(100, 100, 100).bold());
        }
    }
}