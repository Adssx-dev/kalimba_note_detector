use super::measures::{max, self};

pub fn max_normalize(data: &[f32]) -> Vec<f32> {
    if data.len() == 0 {
        Vec::new()
    } else {
        let max_inv = 1.0f32 / measures::max(data);
        let length = data.iter().map(|elem| elem.abs()).len();
        let mut result = Vec::with_capacity(length);
        for idx in 0..data.len() {
            result.push(data[idx] * max_inv);
        }
        result
    }
}
