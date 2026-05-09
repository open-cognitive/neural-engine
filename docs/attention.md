# Attention Formula Explained

Transformer mimarisinin temelinde şu denklem bulunur:

$$
Attention(Q, K, V) =
softmax\left(
\frac{QK^T}{\sqrt{d_k}}
\right)V
$$

Bu formül modern yapay zekâ sistemlerinin temel yapı taşlarından biridir.

---

# Intuition

Attention mekanizmasının amacı:

> "Verilen bilgi içinde en alakalı parçaları bulmak."

İnsan beyninin dikkat mekanizmasına benzer çalışır.

Örneğin:
- Kalabalık bir odada kendi adını duymak
- Bir paragrafta önemli kelimeyi fark etmek
- Bir cümlede hangi kelimenin diğerine bağlı olduğunu anlamak

Transformer bunu matematiksel olarak yapar.

---

# Q, K, V Nedir?

## Query (Q)

Modelin aradığı şey.

Soru gibi düşünülebilir:

> "Ben şu anda hangi bilgiye ihtiyaç duyuyorum?"

---

## Key (K)

Bilgilerin etiketleri veya temsilidir.

Query ile karşılaştırılır.

> "Bu bilgi benim aradığımla ne kadar ilişkili?"

---

## Value (V)

Asıl taşınan bilgi.

Attention sonucu hangi bilgi önemliyse onun Value kısmı öne çıkarılır.

---

# Adım Adım Çalışma

## 1. Benzerlik Hesabı

$$
QK^T
$$

Query ile Key vektörleri karşılaştırılır.

Bu işlem:
- benzerlik,
- ilişki,
- alaka düzeyi

hesaplar.

Yüksek skor = daha ilgili bilgi.

---

## 2. Ölçekleme

$$
\frac{QK^T}{\sqrt{d_k}}
$$

Büyük sayılar softmax'i kararsız hale getirebilir.

Bu yüzden sonuç normalize edilir.

$d_k$:
Key vektörünün boyutudur.

---

## 3. Softmax

$$
softmax(...)
$$

Skorları olasılık dağılımına çevirir.

Örneğin:

```text
[2.1, 8.7, 0.3]
↓
[0.01, 0.98, 0.01]
```

Model artık hangi bilginin önemli olduğunu bilir.

---

## 4. Value ile Çarpım

$$
softmax(...)V
$$

Önem skorları gerçek bilgiye uygulanır.

Böylece model:

* önemsiz bilgileri bastırır,
* önemli bilgileri güçlendirir.

---

# Basit Bir Örnek

Şu cümleyi düşün:

```text
"Kedi masanın üstünde uyuyor."
```

Model "uyuyor" kelimesini işlerken:

* "kedi" kelimesine yüksek attention verebilir
* "masanın" kelimesine orta seviye attention verebilir

Çünkü:

* uyuyan şey = kedi
* konum bilgisi = masa

Attention mekanizması bu ilişkileri matematiksel olarak öğrenir.

---

# Bu Neden Güçlü?

Attention mekanizması:

* uzun bağımlılıkları öğrenebilir,
* paralel çalışabilir,
* dil ilişkilerini anlayabilir,
* büyük veri üzerinde ölçeklenebilir.

Modern transformer modellerinin temel sebebi budur.

---

# neural-engine İçindeki Amaç

Bu proje:

* attention mekanizmasını,
* tensör matematiğini,
* forward pass sistemini

harici ML framework kullanmadan, saf Rust ile implement etmeyi hedefler.

Amaç:

* algoritmayı gerçekten anlamak,
* donanıma yakın çalışmak,
* neural computation çekirdeğini sıfırdan kurmaktır.

