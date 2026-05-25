//! # Open-Cognitive: Neural Engine (Sistem 1) - Bus Worker

mod math;

use math::tensor::Tensor2D;
use math::attention::scaled_dot_product_attention;
use open_cognitive_protocol::{CMD_FORWARD_PASS, CMD_IDLE, TOOL_SQUARE};
use open_cognitive_protocol::ipc::MemoryBus;

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Neural Engine (BUS WORKER) Başlatılıyor ===");
    
    let mut bus = MemoryBus::new("/tmp/cog.bus")?;
    println!("[SİSTEM] Bellek Otobüsü dinleniyor: /tmp/cog.bus");

    loop {
        // Bellekten kopyalamadan sinyali oku
        let mut signal = bus.read_signal();

        if signal.command_type == CMD_FORWARD_PASS {
            let prompt = signal.get_prompt();
            println!("\n[EMİR ALINDI] Forward Pass tetiklendi!");
            println!("[GİRDİ] Gelen Metin: \"{}\"", prompt);
            println!("[MATH] Saf Matematiksel Attention mekanizması çalıştırılıyor...");
            
            // 1. Saf Matematik (Self-Attention Simülasyonu)
            let q = Tensor2D::zeros(2, 3);
            let k_t = Tensor2D::zeros(3, 2);
            let v = Tensor2D::zeros(2, 3);
            let _output = scaled_dot_product_attention(&q, &k_t, &v);

            // 2. MVP Niyet (Intent) Çıkarımı
            // Not: Gerçek ağırlıklar (.cogw) yüklenene kadar string parse ile simüle ediyoruz.
            let mut detected_number: i32 = 0;
            let mut intent_found = false;

            // Vektörün içindeki kelime token'larına "dikkat" (attention) ettiğimizi varsayıyoruz:
            if prompt.to_lowercase().contains("kare") {
                intent_found = true;
                
                // Prompt içindeki sayıyı (parametreyi) çıkarıyoruz (Örn: "6 sayısının karesi")
                for word in prompt.split_whitespace() {
                    if let Ok(num) = word.parse::<i32>() {
                        detected_number = num;
                        break;
                    }
                }
            }

            // 3. Sonucu Belleğe (Payload) Yaz
            if intent_found {
                println!("[NÖRAL AĞ] Niyet Saptandı: Karesini Alma (Tool ID: {})", TOOL_SQUARE);
                println!("[NÖRAL AĞ] Çıkarılan Parametre: {}", detected_number);
                
                // Zero-Copy ile veriyi yerleştir
                signal.set_tool_call(TOOL_SQUARE, detected_number);
            } else {
                println!("[NÖRAL AĞ] Niyet anlaşılamadı. Ajan belirsizlik (0) dönüyor.");
                signal.set_tool_call(0, 0);
            }

            println!("[MATH] Hesaplama tamamlandı.");
            
            // İşi bitirdiğini belirtmek için belleği temizle (ACK)
            signal.command_type = CMD_IDLE;
            bus.write_signal(&signal);
            println!("[SİSTEM] Otobüs temizlendi (ACK gönderildi). Yeni emir bekleniyor...");
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}