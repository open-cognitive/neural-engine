//! # Open-Cognitive: Neural Engine (Sistem 1) - Bus Worker

mod math;

use math::tensor::Tensor2D;
use math::attention::scaled_dot_product_attention;
use math::weights::ModelWeights;
use open_cognitive_protocol::{CMD_FORWARD_PASS, CMD_IDLE, TOOL_SQUARE};
use open_cognitive_protocol::ipc::MemoryBus;

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Neural Engine (BUS WORKER) Başlatılıyor ===");
    
    // --- 1. AĞIRLIKLARI (WEIGHTS) ZERO-COPY İLE YÜKLE ---
    println!("[SİSTEM] Ağırlıklar (Weights) belleğe eşleniyor...");
    let weights = ModelWeights::load("dummy_model.cogw").expect(
        "Ağırlık dosyası bulunamadı! Lütfen önce 'cargo run --bin generate_weights' çalıştırın."
    );
    let header = weights.header();
    println!("[SİSTEM] Model Başarıyla Yüklendi!");
    println!("         -> Boyut (Dim): {}", header.model_dim);
    println!("         -> Katman (Layers): {}", header.layer_count);
    println!("         -> Toplam Parametre Boyutu: {} Bayt", weights.raw_data_size());
    
    // --- 2. IPC OTOBÜSÜNÜ BAĞLA ---
    let mut bus = MemoryBus::new("/tmp/cog.bus")?;
    println!("[SİSTEM] Bellek Otobüsü dinleniyor: /tmp/cog.bus");

    loop {
        let mut signal = bus.read_signal();

        if signal.command_type == CMD_FORWARD_PASS {
            let prompt = signal.get_prompt();
            println!("\n[EMİR ALINDI] Forward Pass tetiklendi!");
            println!("[GİRDİ] Metin: \"{}\"", prompt);
            println!("[MATH] Ağırlıklar (Weights) üzerinden Attention hesaplanıyor...");
            
            // 3. Matris Çarpımı 
            // Gelecek sürümde (v0.3.0) q, k, v matrisleri 'weights.mmap' içinden çekilecek!
            let q = Tensor2D::zeros(header.model_dim as usize, 3);
            let k_t = Tensor2D::zeros(3, header.model_dim as usize);
            let v = Tensor2D::zeros(header.model_dim as usize, 3);
            let _output = scaled_dot_product_attention(&q, &k_t, &v);

            // 4. MVP Niyet Çıkarımı (İleride Softmax logit'lerine göre yapılacak, şimdilik hibrit)
            let mut detected_number: i64 = 0; // i32 idi, i64 yapıldı
            let mut intent_found = false;

            if prompt.to_lowercase().contains("kare") {
                intent_found = true;
                for word in prompt.split_whitespace() {
                    if let Ok(num) = word.parse::<i64>() { // parse::<i64> yapıldı
                        detected_number = num;
                        break;
                    }
                }
            }

            // 5. Sonucu Belleğe Yaz
            if intent_found {
                println!("[NÖRAL AĞ] Matris Çıktısı -> Niyet: Kare Alma (Tool ID: {})", TOOL_SQUARE);
                println!("[NÖRAL AĞ] Çıkarılan Parametre: {}", detected_number);
                signal.set_tool_call(TOOL_SQUARE, detected_number);
            } else {
                println!("[NÖRAL AĞ] Matris Çıktısı -> Niyet anlaşılamadı (0).");
                signal.set_tool_call(0, 0);
            }

            signal.command_type = CMD_IDLE;
            bus.write_signal(&signal);
            println!("[SİSTEM] Otobüs temizlendi (ACK gönderildi).");
        }

        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}