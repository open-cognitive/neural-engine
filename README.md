# README.md

# neural-engine

Dış API YOK.

"Attention Is All You Need" teorisini ve Transformer mimarisini saf kodla çalıştıran; tensör matematiği, attention mekanizması ve ileri yayılım (forward pass) motoru.

İşletim sisteminin ALU'su (Aritmetik Mantık Birimi) gibi çalışan düşük seviyeli bir neural engine.

---

## Core Formula

$Attention(Q, K, V) = softmax(\frac{QK^T}{\sqrt{d_k}})V$

Transformer mimarisinin çekirdeği.

- Q (Query): Ne arıyorum?
- K (Key): Hangi bilgi bana uygun?
- V (Value): Taşınacak gerçek bilgi

Sistem:
1. Bilgiler arasındaki benzerliği hesaplar
2. Hangi bilginin daha önemli olduğunu belirler
3. En alakalı bilgiyi öne çıkarır

Detaylı matematik açıklaması:

- [docs/attention.md](docs/attention.md)

---

## Goals

- Saf Rust implementasyonu
- Düşük seviyeli tensör operasyonları
- Donanıma yakın attention sistemi
- Forward pass motoru
- Harici ML framework bağımlılığı olmadan çalışma

---

## Status

WIP (Work In Progress)