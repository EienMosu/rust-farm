# Rust Farm

Rust programlama dilini öğrenmek amacıyla oluşturdum. Kitap, eğitim videosu, makale ve benzeri kaynaklardan öğrendiklerimi derleyip toparladığım bir çalışma alanı olarak kullanmayı planlıyorum. Hatta şirket bünyesinde açılması planlanan Rust eğitimleri için gerekli materyalleri de burada toparlayabilirim. Örnekleri Ubuntu üstünde ve [JetBrains IntelliJ IDE'sini](https://www.jetbrains.com/idea/) kullanarak geliştirmekteyim. Ben Community sürümünü tercih ettim. Ayrıca IDE'yi kurduktan sonra Rust plug-in'inini yükleyince tadından yenmiyor. Rust programlama dili için gerekli ortamın bilgisayara yüklenmesi de oldukça kolay. Bunun için [RustUp.rs](https://rustup.rs/) adresine gidip kendi platformunuz için gerekli talimatları takip etmeniz yeterli. 

Projeleri oluştururken sıklıkla kullanılan bazı komut satırı ifadeleri de var. cargo, rust'ın önemli bir terminal amacı. Proje oluşturmak, test koşmak, sürüm çıkarmak, dokümantasyon üretmek, paketleri yönetmek gibi pek çok işi yapıyor. Kullanımı ile ilgili [şu adreste](https://doc.rust-lang.org/cargo/commands/index.html) detaylı bilgiler var. Çalışmalar sırasında detaylı kullanımlarını da göreceğiz.

```bash
# hello_world isimli yeni rust projesi oluşturur
cargo new hello_world

# locig isimli yeni bir rust kütüphanesi oluşturur
cargo new logic --lib

# projeyi çalıştırır
cargo run

# projedeki testleri koşmamızı sağlar
cargo test

# akıllı yardım dokümantasyonunu hazırlar
cargo doc
```
