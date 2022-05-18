
pub fn root_mean_square(data: &[f32]) -> f32 {
    let mut sum = 0f32;

    for elem in data {
        sum += elem * elem;
    }

    sum / (data.len() as f32)
}


pub fn mean(data: &[f32]) -> f32 {
    let mut sum = 0f32;

    for elem in data {
        sum += elem * elem;
    }

    sum / (data.len() as f32)
}

pub fn min(data: &[f32]) -> f32 {
    data.into_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone()
}

pub fn max(data: &[f32]) -> f32 {
    data.into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone()
}
