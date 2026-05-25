//! # Open-Cognitive: Neural Engine (Sistem 1)

mod math;

use math::tensor::Tensor2D;
use math::attention::scaled_dot_product_attention;
use math::weights::ModelWeights;
use open_cognitive_protocol::{CMD_FORWARD_PASS, CMD_IDLE, TOOL_SQUARE, TOOL_TEXT_PROCESS, TOOL_SYS_REPORT, TOOL_READ_FILE};
use open_cognitive_protocol::ipc::MemoryBus;

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Neural Engine Başlatılıyor ===");
    
    let weights = ModelWeights::load("dummy_model.cogw").expect("Model bulunamadı!");
    let header = weights.header();
    let mut bus = MemoryBus::new("/tmp/cog.bus")?;

    loop {
        let mut signal = bus.read_signal();

        if signal.command_type == CMD_FORWARD_PASS {
            let prompt = signal.get_prompt();
            println!("\n[EMİR ALINDI] GİRDİ: \"{}\"", prompt);
            
            let q = Tensor2D::zeros(header.model_dim as usize, 3);
            let k_t = Tensor2D::zeros(3, header.model_dim as usize);
            let v = Tensor2D::zeros(header.model_dim as usize, 3);
            let output = scaled_dot_product_attention(&q, &k_t, &v);
            println!("[MATH] Öz-Dikkat (Self-Attention) Matrisi Hesaplandı! Çıktı Boyutu: {}x{}", output.rows, output.cols);

            let mut intent_found = false;

            if prompt.to_lowercase().contains("kare") {
                intent_found = true;
                let mut detected_number: i64 = 0;
                for word in prompt.split_whitespace() {
                    if let Ok(num) = word.parse::<i64>() { detected_number = num; break; }
                }
                println!("[NÖRAL AĞ] Niyet: Kare Alma | Girdi: {}", detected_number);
                signal.set_tool_call(TOOL_SQUARE, detected_number);
                
            } else if prompt.to_lowercase().contains("oku") {
                // Ajanın cümleden dosya yolunu (/etc/hostname) çıkarması
                intent_found = true;
                let mut path = "/etc/hostname".to_string(); // Varsayılan
                for word in prompt.split_whitespace() {
                    if word.starts_with("/") || word.starts_with("./") {
                        path = word.to_string();
                        break;
                    }
                }
                println!("[NÖRAL AĞ] Niyet: Dosya Okuma | Hedef Yol: {}", path);
                signal.set_tool_payload_string(TOOL_READ_FILE, &path);
                
            } else if prompt.to_lowercase().contains("çevir") {
                intent_found = true;
                let clean_text = prompt.replace("büyük harfe çevir", "").trim().to_string();
                signal.set_tool_payload_string(TOOL_TEXT_PROCESS, &clean_text);
                
            } else if prompt.to_lowercase().contains("rapor") {
                intent_found = true;
                signal.set_tool_payload_string(TOOL_SYS_REPORT, "get_stats");
            }

            if !intent_found {
                println!("[NÖRAL AĞ] Niyet anlaşılamadı (0).");
                signal.set_tool_call(0, 0);
            }

            signal.command_type = CMD_IDLE;
            bus.write_signal(&signal);
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}