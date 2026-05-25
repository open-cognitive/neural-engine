//! # Open-Cognitive: Neural Engine (Sistem 1) - Olasılıksal Çıkarım Motoru

mod math;

use math::weights::ModelWeights;
use open_cognitive_protocol::{CMD_FORWARD_PASS, CMD_IDLE, TOOL_SQUARE, TOOL_TEXT_PROCESS, TOOL_SYS_REPORT, TOOL_READ_FILE};
use open_cognitive_protocol::ipc::MemoryBus;

/// En yüksek olasılığı (Logit) bulur. (Yapay zekanın klasik Argmax fonksiyonu)
fn argmax(logits: &[f32]) -> usize {
    let mut max_idx = 0;
    let mut max_val = logits[0];
    for (i, &val) in logits.iter().enumerate() {
        if val > max_val {
            max_val = val;
            max_idx = i;
        }
    }
    max_idx
}

/// Gelen metne göre niyet skorlarını hesaplar (İleride mmap matrislerinden gelecek)
fn compute_intent_logits(prompt: &str) -> [f32; 5] {
    // 0: Idle, 1: Square, 2: Text, 3: Report, 4: File
    let mut logits = [0.1, 0.1, 0.1, 0.1, 0.1]; // Temel (Base) olasılıklar
    
    let p = prompt.to_lowercase();
    
    // Simüle edilmiş ağırlık aktivasyonları
    if p.contains("kare") { logits[1] += 8.5; }
    if p.contains("çevir") || p.contains("metin") { logits[2] += 9.2; }
    if p.contains("rapor") || p.contains("sistem") { logits[3] += 8.9; }
    if p.contains("oku") || p.contains("dosya") { logits[4] += 9.5; }
    
    logits
}

fn extract_parameters(prompt: &str, intent_idx: usize) -> (i64, String) {
    let mut number_param: i64 = 0;
    let mut text_param = String::new();

    match intent_idx {
        1 => { // Kare
            for word in prompt.split_whitespace() {
                if let Ok(num) = word.parse::<i64>() { number_param = num; break; }
            }
        },
        2 => { // Metin İşleme
            text_param = prompt.replace("büyük harfe çevir", "").trim().to_string();
        },
        3 => { // Rapor
            text_param = "get_stats".to_string();
        },
        4 => { // Dosya
            for word in prompt.split_whitespace() {
                if word.starts_with("/") || word.starts_with("./") {
                    text_param = word.to_string();
                    break;
                }
            }
        },
        _ => {}
    }
    (number_param, text_param)
}

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Neural Engine Başlatılıyor ===");
    
    let weights = ModelWeights::load("dummy_model.cogw").expect("Model bulunamadı!");
    let header = weights.header();
    println!("[SİSTEM] Model Mimarisi Yüklendi -> Dim: {}", header.model_dim);
    
    let mut bus = MemoryBus::new("/tmp/cog.bus")?;

    loop {
        let mut signal = bus.read_signal();

        if signal.command_type == CMD_FORWARD_PASS {
            let prompt = signal.get_prompt();
            println!("\n[İLERİ YAYILIM - FORWARD PASS] Girdi: \"{}\"", prompt);
            
            // 1. Logit Hesaplama
            let logits = compute_intent_logits(&prompt);
            println!("[MATH] Logit Vektörü (Olasılıklar): {:.2?}", logits);
            
            // 2. Argmax (En yüksek olasılığı seçme)
            let selected_intent = argmax(&logits);
            
            // 3. Olasılık eşiği (Güvenilirlik Testi)
            if logits[selected_intent] < 1.0 {
                println!("[NÖRAL AĞ] Yeterli güven eşiğine ulaşılamadı. Niyet anlaşılamadı.");
                signal.set_tool_call(0, 0);
            } else {
                // 4. Parametre Çıkarımı ve Yönlendirme
                let (num_param, text_param) = extract_parameters(&prompt, selected_intent);
                
                match selected_intent {
                    1 => {
                        println!("[NÖRAL AĞ] Karar: Sayısal İşlem (Güven: {:.2})", logits[1]);
                        signal.set_tool_call(TOOL_SQUARE, num_param);
                    },
                    2 => {
                        println!("[NÖRAL AĞ] Karar: Metin İşleme (Güven: {:.2})", logits[2]);
                        signal.set_tool_payload_string(TOOL_TEXT_PROCESS, &text_param);
                    },
                    3 => {
                        println!("[NÖRAL AĞ] Karar: Sistem Raporu (Güven: {:.2})", logits[3]);
                        signal.set_tool_payload_string(TOOL_SYS_REPORT, &text_param);
                    },
                    4 => {
                        println!("[NÖRAL AĞ] Karar: Dosya Okuma (Güven: {:.2})", logits[4]);
                        signal.set_tool_payload_string(TOOL_READ_FILE, &text_param);
                    },
                    _ => {}
                }
            }

            signal.command_type = CMD_IDLE;
            bus.write_signal(&signal);
        }
        std::thread::sleep(std::time::Duration::from_millis(50)); // Dinleme hızını artırdık
    }
}