<div align="center">
  <img src="./logo.jpg" alt="İstinye Üniversitesi Logo" width="250"/>
  <br/>
  <h3>İstinye Üniversitesi</h3>
  <b>Danışman / Eğitmen:</b> Keyvan Arasteh Abbasabad
</div>

<hr/>

# 🛡️ Rust-Service-Port-Auditor

<p align="left">
  <img src="https://img.shields.io/badge/Language-Rust-orange?style=flat-square&logo=rust" alt="Language: Rust"/>
  <a href="https://opensource.org/licenses/MIT"><img src="https://img.shields.io/badge/License-MIT-blue?style=flat-square" alt="License: MIT"/></a>
  <img src="https://img.shields.io/badge/Status-Active-success?style=flat-square" alt="Status: Active"/>
</p>

Rust ile geliştirilmiş, Tokio tabanlı, yüksek performanslı ve asenkron bir ağ denetim aracıdır. Bu proje, ağ üzerindeki açık portları tespit etmenin ötesine geçerek servis analizi ve zafiyet taraması gerçekleştirir.


## 📋 İçindekiler
- [🎬 Demo](#-demo)
- [Öne Çıkan Özellikler](#-öne-çıkan-özellikler)
- [Kurulum ve Kullanım](#-kurulum-ve-kullanım)

---

## 🎬 Demo

Aşağıdaki kayıtta aracın asenkron çalışma prensibi ve örnek bir zafiyet tarama çıktısı görülmektedir.

![Demo Scan Record](demo/project-demo.webp)

> [!TIP]
> Bu demoda yerel ağdaki (127.0.0.1) bir FTP sunucusu taranmış ve "Anonymous Login" zafiyeti tespit edilmiştir. Sistem gerçek zamanlı olarak kritiklik seviyesini raporlamaktadır.

---

## 🌟 Öne Çıkan Özellikler

* **Asenkron Mimari:** `tokio` ve `futures` ile eşzamanlı (concurrent) tarama kapasitesi.
* **Servis Analizi (Banner Grabbing):** Açık portların arkasındaki servislerin (SSH, HTTP vb.) versiyon bilgilerini yakalar.
* **Aktif Zafiyet Tespiti:** FTP protokolü üzerinde anonim giriş (Anonymous Login) zafiyetini (9. Madde) aktif olarak test eder.
* **Akıllı Kaynak Yönetimi:** `Semaphore` kullanarak sistem kaynaklarını optimize eder ve aşırı yüklenmeyi önler.
* **Esnek Kullanım:** Dinamik port aralığı (Range) desteği ve ayarlanabilir eşzamanlılık seviyesi.

## 🛠️ Kurulum ve Kullanım

```bash
# Projeyi derleyin
cargo build --release

# Örnek Kullanım (Hızlı Tarama)
cargo run -- --target 45.33.32.156 --ports 20-100,8080 --concurrent 100