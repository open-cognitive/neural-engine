//! # Open-Cognitive: Neural Engine (Sistem 1) - Bus Worker

mod math;
use open_cognitive_protocol::{CMD_FORWARD_PASS};
use open_cognitive_protocol::ipc::MemoryBus;

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Neural Engine (BUS WORKER) Başlatılıyor ===");
    
    // 1. Aynı bellek otobüsüne bağlan
    let bus = MemoryBus::new("/tmp/cog.bus")?;
    println!("[SİSTEM] Bellek Otobüsü dinleniyor: /tmp/cog.bus");

    loop {
        // 2. Belleği oku (Zero-Copy)
        let signal = bus.read_signal();

        // 3. Emri değerlendir
        if signal.command_type == CMD_FORWARD_PASS {
            println!("\n[EMİR ALINDI] Forward Pass tetiklendi! Context: {}", signal.context_length);
            println!("[MATH] Attention mekanizması çalıştırılıyor...");
            
            // Burada daha önce yazdığımız math/attention kodlarını çalıştırabiliriz.
            // Şimdilik simüle ediyoruz:
            std::thread::sleep(std::time::Duration::from_millis(200));
            println!("[MATH] Hesaplama tamamlandı. Sonuçlar belleğe yansıtılmaya hazır.");
        }

        // CPU'yu %100 kullanmamak için kısa bir dinlenme (Polling Interval)
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}