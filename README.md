# 🛡️Rust-Service-Port-Auditor

Rust ile geliştirilmiş, **Tokio** tabanlı, yüksek performanslı ve asenkron bir ağ denetim aracıdır. Bu proje, ağ üzerindeki açık portları tespit etmenin ötesine geçerek servis analizi ve zafiyet taraması gerçekleştirir.

## 🌟 Öne Çıkan Özellikler
- **Asenkron Mimari:** `tokio` ve `futures` ile eşzamanlı (concurrent) tarama kapasitesi.
- **Servis Analizi (Banner Grabbing):** Açık portların arkasındaki servislerin (SSH, HTTP vb.) versiyon bilgilerini yakalar.
- **Aktif Zafiyet Tespiti:** FTP protokolü üzerinde anonim giriş (Anonymous Login) zafiyetini (9. Madde) aktif olarak test eder.
- **Akıllı Kaynak Yönetimi:** `Semaphore` kullanarak sistem kaynaklarını optimize eder ve aşırı yüklenmeyi önler.
- **Esnek Kullanım:** Dinamik port aralığı (Range) desteği ve ayarlanabilir eşzamanlılık seviyesi.

## 🛠️ Kurulum ve Kullanım
```bash
# Projeyi derleyin
cargo build --release

# Örnek Kullanım (Hızlı Tarama)
cargo run -- --target 45.33.32.156 --ports 20-100,8080 --concurrent 100