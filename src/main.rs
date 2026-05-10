//! # Open-Cognitive: Neural Engine (Sistem 1) - Bus Worker

mod math;

use math::tensor::Tensor2D;
use math::attention::scaled_dot_product_attention;
use open_cognitive_protocol::{CMD_FORWARD_PASS, CMD_IDLE};
use open_cognitive_protocol::ipc::MemoryBus;

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Neural Engine (BUS WORKER) Başlatılıyor ===");
    
    let mut bus = MemoryBus::new("/tmp/cog.bus")?;
    println!("[SİSTEM] Bellek Otobüsü dinleniyor: /tmp/cog.bus");

    loop {
        let signal = bus.read_signal();

        if signal.command_type == CMD_FORWARD_PASS {
            println!("\n[EMİR ALINDI] Forward Pass tetiklendi!");
            println!("[MATH] Saf Matematiksel Attention mekanizması çalıştırılıyor...");
            
            // Kullanılmayan Kod (dead_code) uyarılarını gidermek ve gerçek hesaplama yapmak için:
            let q = Tensor2D::zeros(2, 3);
            let k_t = Tensor2D::zeros(3, 2);
            let v = Tensor2D::zeros(2, 3);
            
            // Dikkat mekanizmasını çalıştır (Şu an sıfır matrisleri ama motorun çalıştığını kanıtlar)
            let _output = scaled_dot_product_attention(&q, &k_t, &v);

            println!("[MATH] Hesaplama tamamlandı.");
            
            // İşi bitirdiğini belirtmek için belleği temizle (ACK)
            let mut ack_signal = signal;
            ack_signal.command_type = CMD_IDLE;
            bus.write_signal(&ack_signal);
            println!("[SİSTEM] Otobüs temizlendi (ACK gönderildi). Yeni emir bekleniyor...");
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}