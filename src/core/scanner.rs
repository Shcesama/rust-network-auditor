use colored::*;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{Duration, timeout};
/// Açık olan bir bağlantı noktasına veri göndererek arkasında çalışan yazılımın bilgisini (banner) almaya çalışır.
///
/// # Argümanlar
/// * `stream` - Karşı sistemle kurulan aktif TCP bağlantısı.
/// * `port` - Kontrol edilen bağlantı noktası numarası.
pub async fn probe_service(mut stream: TcpStream, port: u16) {
    let mut buffer = [0; 1024];

    // Web portları için önceden istek gönder
    let web_ports = [80, 443, 8080, 8443];
    if web_ports.contains(&port) {
        if let Err(e) = stream.write_all(b"HEAD / HTTP/1.0\r\n\r\n").await {
            eprintln!(
                "{} Port {}: Servis bilgisi gönderilemedi: {}",
                "[!]".yellow(),
                port,
                e
            );
            return;
        }
    }

    match timeout(Duration::from_secs(2), stream.read(&mut buffer)).await {
        Ok(Ok(n)) if n > 0 => {
            let banner = String::from_utf8_lossy(&buffer[..n]);
            println!(
                "    {} Servis Mesajı: {}",
                "-->".yellow(),
                banner.trim().replace("\r\n", " ").cyan()
            );

            // FTP Zafiyet Kontrolü (vulns modülüne gidiyor)
            if port == 21 || banner.contains("FTP") {
                print!("    {} FTP Anonim Giriş Deneniyor... ", "!!!".red().bold());
                if crate::core::vulns::test_ftp_anonymous(stream).await {
                    println!(
                        "{}",
                        "ERİŞİM SAĞLANDI! (KRİTİK ZAFİYET)".red().bold().blink()
                    );
                } else {
                    println!("{}", "Başarısız (Güvenli)".green());
                }
            }
        }
        _ => {
            // Yanıt gelmezse gri renkli bir bilgi mesajı yazdır
            println!(
                "    {} Yanıt alınamadı (Zaman Aşımı).",
                "-->".truecolor(100, 100, 100).bold()
            );
        }
    }
}
