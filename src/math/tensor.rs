//! Temel Tensor (Matris) operasyonları.
//! Bellek hizalaması için veriler düz bir Vec<f32> içinde tutulur.

#[derive(Debug, Clone)]
pub struct Tensor2D {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f32>,
}

impl Tensor2D {
    /// İçi sıfırlarla dolu yeni bir matris oluşturur
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![0.0; rows * cols],
        }
    }

    /// Matris Çarpımı (Dot Product). O(n^3) karmaşıklık.
    /// Gelecek optimizasyon: SIMD veya BLAS kullanılacak.
    pub fn matmul(a: &Tensor2D, b: &Tensor2D) -> Tensor2D {
        assert_eq!(a.cols, b.rows, "Matris boyutları çarpım için uyumsuz!");
        
        let mut result = Tensor2D::zeros(a.rows, b.cols);
        
        for i in 0..a.rows {
            for j in 0..b.cols {
                let mut sum = 0.0;
                for k in 0..a.cols {
                    // Düz dizideki 2D indeks hesabı: (satır * toplam_kolon) + kolon
                    sum += a.data[i * a.cols + k] * b.data[k * b.cols + j];
                }
                result.data[i * result.cols + j] = sum;
            }
        }
        result
    }
}