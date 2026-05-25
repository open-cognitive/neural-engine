//! # Open-Cognitive: Neural Engine (Sistem 1)

mod math;

use math::tensor::Tensor2D;
use math::attention::scaled_dot_product_attention;
use math::weights::ModelWeights;
use open_cognitive_protocol::{CMD_FORWARD_PASS, CMD_IDLE, TOOL_SQUARE, TOOL_TEXT_PROCESS};
use open_cognitive_protocol::ipc::MemoryBus;

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Neural Engine Başlatılıyor ===");
    
    let weights = ModelWeights::load("dummy_model.cogw").expect("Model bulunamadı!");
    let mut bus = MemoryBus::new("/tmp/cog.bus")?;

    loop {
        let mut signal = bus.read_signal();

        if signal.command_type == CMD_FORWARD_PASS {
            let prompt = signal.get_prompt();
            println!("\n[EMİR ALINDI] GİRDİ: \"{}\"", prompt);
            
            let q = Tensor2D::zeros(128, 3);
            let k_t = Tensor2D::zeros(3, 128);
            let v = Tensor2D::zeros(128, 3);
            let _output = scaled_dot_product_attention(&q, &k_t, &v);

            let mut intent_found = false;

            if prompt.to_lowercase().contains("kare") {
                intent_found = true;
                let mut detected_number: i64 = 0;
                for word in prompt.split_whitespace() {
                    if let Ok(num) = word.parse::<i64>() {
                        detected_number = num;
                        break;
                    }
                }
                println!("[NÖRAL AĞ] Niyet: Kare Alma | Girdi: {}", detected_number);
                signal.set_tool_call(TOOL_SQUARE, detected_number);
            } else if prompt.to_lowercase().contains("büyük harfe çevir") {
                intent_found = true;
                let clean_text = prompt.replace("büyük harfe çevir", "").replace("\"", "").trim().to_string();
                println!("[NÖRAL AĞ] Niyet: Metin İşleme | Girdi: {}", clean_text);
                signal.set_tool_payload_string(TOOL_TEXT_PROCESS, &clean_text);
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