//! # Cognitive Weights (.cogw) Okuyucu

use bytemuck::{Pod, Zeroable};
use memmap2::Mmap;
use std::fs::File;
use std::path::Path;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct CogwHeader {
    pub magic: [u8; 8],
    pub version: u32,
    pub model_dim: u32,
    pub layer_count: u32,
    pub dtype: u8,
    pub _padding1: [u8; 32],
    pub _padding2: [u8; 11],
}

pub struct ModelWeights {
    pub mmap: Mmap, // Erişimi public yaptık (Uyarıyı çözecek)
}

impl ModelWeights {
    pub fn load<P: AsRef<Path>>(path: P) -> std::io::Result<Self> {
        let file = File::open(path)?;
        let mmap = unsafe { Mmap::map(&file)? };
        
        let header_bytes = &mmap[0..64];
        let header: &CogwHeader = bytemuck::from_bytes(header_bytes);
        
        if &header.magic != b"COGNITIV" {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData,"Bozuk .cogw dosyası!"));
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