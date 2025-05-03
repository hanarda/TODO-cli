# Yapılacaklar CLI (Todo CLI)

Rust ile geliştirilmiş basit bir komut satırı yapılacaklar uygulaması.

## Özellikler
- Başlık, açıklama, öncelik, son tarih ve kategori ile görev ekleme.
- Görevleri listeleme, tamamlama, silme ve temizleme.
- Görevleri önceliğe veya tamamlama durumuna göre filtreleme.
- Kullanıcı dostu menüye sahip etkileşimli mod.

## Kurulum
1. Yayınlar (Releases) sayfasından ikili dosyayı (todo veya todo.exe) indirin.
2. Dosyayı PATH'inizde olan bir dizine yerleştirin.
3. Kullanım talimatları için todo --help komutunu çalıştırın.

## Kullanım
```bash
# Etkileşimli modu başlat
todo-cli interactive
```

### İnteraktif menüyü kullanmak istemezseniz --help komutu ile hangi komutları kullanacağınızı öğrenebilirsiniz
```bash
# Ekleme 
todo-cli add "baslik" "aciklama" --priority high --category shop --deadline "yıl-ay-gün saat:dakika"

# Listeleme
todo-cli list

# Tamamlama
todo-cli complete id

# Silme
todo-cli delete id

# Hepsini Silme
todo-cli clear

# Önceliğe Göre Filtreleme
todo-cli filter-priority priority(3 seçenek var => low,medium,high)

# Tamamlanmaya Göre Filtreleme
todo-cli filter-status complete(2 seçenek var => completed, incomplete)
```
