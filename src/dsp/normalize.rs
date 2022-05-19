use super::measures::{root_mean_square, self};

pub fn max_normalize(data: &[f32]) -> Vec<f32> {
    if data.len() == 0 {
        Vec::new()
    } else {
        let max_inv = 1.0f32 / measures::max(data);
        let length = data.len();
        let mut result = Vec::with_capacity(length);
        for idx in 0..data.len() {
            result.push(data[idx] * max_inv);
        }
        result
    }
}

pub fn rms_normalize(data: &[f32], target_gain : f32) -> Vec<f32> {
    if data.len() == 0 {
        Vec::new()
    } else {
        let rms = root_mean_square(data);
        let gain = target_gain / rms;
        data.iter().map(|sample| sample * gain).collect()
    }
}
