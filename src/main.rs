//! Open-Cognitive: Neural Engine (Sistem 1)

// Uyarıları bastırmak için değil, protokolü gelecekte kullanacağımız için import ettik.
#[allow(unused_imports)]
use open_cognitive_protocol::{CognitiveSignal, TensorDescriptor, CMD_FORWARD_PASS};

// Matematik modüllerimizi dahil ediyoruz
mod math;
use math::tensor::Tensor2D;
use math::attention::scaled_dot_product_attention;

fn main() {
    println!("=== Open-Cognitive Neural Engine Başlatılıyor ===");
    println!("Saf Matematiksel Attention Çekirdeği Test Ediliyor...\n");

    // Basit bir test: 2 kelimelik (token) bir cümle, her kelimenin 3 boyutu (d_k) var.
    // Query (Arayan), Key (Anahtar), Value (Değer) matrisleri oluşturuluyor.
    
    let mut q = Tensor2D::zeros(2, 3);
    q.data = vec![1.0, 0.0, 1.0,   // "Ben" kelimesinin vektörü
                  0.0, 1.0, 0.0];  // "Kimim" kelimesinin vektörü

    // K'nın Transpozu (K^T). Boyutu: 3x2
    let mut k_t = Tensor2D::zeros(3, 2);
    k_t.data = vec![1.0, 0.0, 
                    0.0, 1.0, 
                    1.0, 0.0];

    // Value (Değer). Boyutu: 2x3
    let mut v = Tensor2D::zeros(2, 3);
    v.data = vec![1.0, 2.0, 3.0, 
                  4.0, 5.0, 6.0];

    // Dikkat formülünü çalıştır
    let output = scaled_dot_product_attention(&q, &k_t, &v);

    println!("Girdi (Query) Boyutları: {}x{}", q.rows, q.cols);
    println!("Çıktı (Attention Output) Boyutları: {}x{}", output.rows, output.cols);
    println!("Hesaplanan Context Vektörü Sonucu:");
    println!("{:?}", output.data);
    println!("\nBaşarılı! LLM'lerin sihirli formülü saf Rust ile çalıştırıldı.");
}