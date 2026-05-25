//! # Cognitive Weights (.cogw) Okuyucu
//! 
//! Model ağırlıklarını RAM'e kopyalamadan disk üzerinden Memory Map (mmap) 
//! yöntemiyle okuyan Zero-Copy modülü.

use bytemuck::{Pod, Zeroable};
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

/// 64-Byte .cogw Header yapısı
#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CogwHeader {
    pub magic: [u8; 8],      // "COGNITIV"
    pub version: u32,        // 1
    pub model_dim: u32,      // Modelin tensor boyutu (Örn: 128)
    pub layer_count: u32,    // Katman sayısı
    pub dtype: u8,           // 0: F32, 1: F16
    pub _padding1: [u8; 32], // 64 Bayta tamamlamak için dolgu (Bölüm 1)
    pub _padding2: [u8; 11], // 64 Bayta tamamlamak için dolgu (Bölüm 2)
}

pub struct ModelWeights {
    mmap: Mmap,
}

impl ModelWeights {
    pub fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        
        let header_bytes = &mmap[0..64];
        let header: &CogwHeader = bytemuck::from_bytes(header_bytes);
        
        if &header.magic != b"COGNITIV" {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Bozuk veya geçersiz .cogw dosyası!",
            ));
        }

        Ok(Self { mmap })
    }

    pub fn header(&self) -> &CogwHeader {
        let header_bytes = &self.mmap[0..64];
        bytemuck::from_bytes(header_bytes)
    }

    pub fn raw_data_size(&self) -> usize {
        self.mmap.len() - 64
    }
}