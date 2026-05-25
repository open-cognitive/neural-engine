//! Sahte bir .cogw dosyası üretici (Test amaçlı)

use bytemuck::{Pod, Zeroable};
use std::fs::File;
use std::io::Write;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct CogwHeader {
    magic: [u8; 8],
    version: u32,
    model_dim: u32,
    layer_count: u32,
    dtype: u8,
    _padding: [u8; 43],
}

fn main() -> std::io::Result<()> {
    println!("⚙️ Open-Cognitive: .cogw Ağırlık Dosyası Üretiliyor...");

    let header = CogwHeader {
        magic: *b"COGNITIV",
        version: 1,
        model_dim: 128,
        layer_count: 4,
        dtype: 0, // F32
        _padding: [0; 43],
    };

    let mut file = File::create("dummy_model.cogw")?;
    
    // Header'ı yaz
    file.write_all(bytemuck::bytes_of(&header))?;
    
    // Sahte matris verisi yaz (Örn: 1024 tane f32 sayısı)
    let dummy_weights: Vec<f32> = vec![0.5; 1024];
    file.write_all(bytemuck::cast_slice(&dummy_weights))?;

    println!("✅ Başarılı: 'dummy_model.cogw' diske yazıldı!");
    Ok(())
}