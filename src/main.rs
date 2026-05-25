//! # Open-Cognitive: Neural Engine (Sistem 1) - Olasılıksal Çıkarım Motoru

mod math;

use math::weights::ModelWeights;
use open_cognitive_protocol::{CMD_FORWARD_PASS, CMD_IDLE, TOOL_SQUARE, TOOL_TEXT_PROCESS, TOOL_SYS_REPORT, TOOL_READ_FILE, TOOL_CLOCK};
use open_cognitive_protocol::ipc::MemoryBus;

fn argmax(logits: &[f32]) -> usize {
    let mut max_idx = 0;
    let mut max_val = logits[0];
    for (i, &val) in logits.iter().enumerate() {
        if val > max_val { max_val = val; max_idx = i; }
    }
    max_idx
}

fn compute_intent_logits(prompt: &str) -> [f32; 6] {
    let mut logits = [0.1, 0.1, 0.1, 0.1, 0.1, 0.1]; 
    let p = prompt.to_lowercase();
    
    if p.contains("kare") { logits[1] += 8.5; }
    if p.contains("çevir") || p.contains("metin") { logits[2] += 9.2; }
    if p.contains("rapor") || p.contains("sistem") { logits[3] += 8.9; }
    if p.contains("oku") || p.contains("dosya") { logits[4] += 9.5; }
    if p.contains("saat") || p.contains("zaman") { logits[5] += 9.7; }
    
    logits
}

fn extract_parameters(prompt: &str, intent_idx: usize) -> (i64, String) {
    let mut number_param: i64 = 0;
    let mut text_param = String::new();

    match intent_idx {
        1 => { for word in prompt.split_whitespace() { if let Ok(num) = word.parse::<i64>() { number_param = num; break; } } },
        2 => { text_param = prompt.replace("büyük harfe çevir", "").trim().to_string(); },
        3 => { text_param = "get_stats".to_string(); },
        4 => { for word in prompt.split_whitespace() { if word.starts_with("/") || word.starts_with("./") { text_param = word.to_string(); break; } } },
        _ => {}
    }
    (number_param, text_param)
}

fn main() -> std::io::Result<()> {
    println!("=== Open-Cognitive Neural Engine Başlatılıyor ===");
    
    let weights = ModelWeights::load("dummy_model.cogw").expect("Model bulunamadı!");
    let header = weights.header(); // Artık kullanılıyor, uyarı vermeyecek!
    println!("[SİSTEM] Model Mimarisi Yüklendi -> Dim: {}, Katmanlar: {}", header.model_dim, header.layer_count);
    
    let mut bus = MemoryBus::new("/tmp/cog.bus")?;

    loop {
        let mut signal = bus.read_signal();

        if signal.command_type == CMD_FORWARD_PASS {
            let prompt = signal.get_prompt();
            println!("\n[İLERİ YAYILIM] Girdi: \"{}\"", prompt);
            
            let logits = compute_intent_logits(&prompt);
            println!("[MATH] Logit Vektörü (Olasılıklar): {:.2?}", logits);
            
            let selected_intent = argmax(&logits);
            
            if logits[selected_intent] < 1.0 {
                println!("[NÖRAL AĞ] Güven eşiğine ulaşılamadı.");
                signal.set_tool_call(0, 0);
            } else {
                let (num_param, text_param) = extract_parameters(&prompt, selected_intent);
                
                match selected_intent {
                    1 => { signal.set_tool_call(TOOL_SQUARE, num_param); },
                    2 => { signal.set_tool_payload_string(TOOL_TEXT_PROCESS, &text_param); },
                    3 => { signal.set_tool_payload_string(TOOL_SYS_REPORT, &text_param); },
                    4 => { signal.set_tool_payload_string(TOOL_READ_FILE, &text_param); },
                    5 => { signal.set_tool_payload_string(TOOL_CLOCK, "time"); },
                    _ => {}
                }
            }
            signal.command_type = CMD_IDLE;
            bus.write_signal(&signal);
        }
        std::thread::sleep(std::time::Duration::from_millis(50)); 
    }
}