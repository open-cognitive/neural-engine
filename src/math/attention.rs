//! Öz-Dikkat (Self-Attention) Mekanizması
//! Formül: Attention(Q, K, V) = softmax(Q * K^T / sqrt(d_k)) * V

use super::tensor::Tensor2D;

/// Dünyayı değiştiren o meşhur Attention fonksiyonu
pub fn scaled_dot_product_attention(q: &Tensor2D, k_t: &Tensor2D, v: &Tensor2D) -> Tensor2D {
    // 1. Q ve K'nın transpozunu çarp (Q * K^T)
    // Bu adım, kelimelerin (token) birbirleriyle olan anlamsal ilişkisini (skorunu) bulur.
    let mut scores = Tensor2D::matmul(q, k_t);

    // 2. Ölçeklendirme (Scale: / sqrt(d_k))
    let d_k = q.cols as f32;
    let scale_factor = d_k.sqrt();
    for val in scores.data.iter_mut() {
        *val /= scale_factor;
    }

    // 3. Softmax Uygulaması (Olasılıkları 0 ile 1 arasına çekme)
    // Basit bir softmax implementasyonu
    for i in 0..scores.rows {
        let row_start = i * scores.cols;
        let row_end = row_start + scores.cols;
        let row_slice = &mut scores.data[row_start..row_end];
        
        // Numerik stabilite için max değeri bul
        let max_val = row_slice.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        
        let mut sum_exp = 0.0;
        for val in row_slice.iter_mut() {
            *val = (*val - max_val).exp();
            sum_exp += *val;
        }
        
        // Normalize et
        for val in row_slice.iter_mut() {
            *val /= sum_exp;
        }
    }

    // 4. Bulunan dikkat olasılıklarını V (Value) matrisiyle çarp
    Tensor2D::matmul(&scores, v)
}