//! Öz-Dikkat (Self-Attention) Mekanizması

#![allow(dead_code)] 

use super::tensor::Tensor2D;

pub fn scaled_dot_product_attention(q: &Tensor2D, k_t: &Tensor2D, v: &Tensor2D) -> Tensor2D {
    let mut scores = Tensor2D::matmul(q, k_t);
    let scale_factor = (q.cols as f32).sqrt();
    
    for val in scores.data.iter_mut() {
        *val /= scale_factor;
    }

    for i in 0..scores.rows {
        let row_start = i * scores.cols;
        let row_end = row_start + scores.cols;
        let row_slice = &mut scores.data[row_start..row_end];
        
        let max_val = row_slice.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let mut sum_exp = 0.0;
        for val in row_slice.iter_mut() {
            *val = (*val - max_val).exp();
            sum_exp += *val;
        }
        for val in row_slice.iter_mut() {
            *val /= sum_exp;
        }
    }
    Tensor2D::matmul(&scores, v)
}