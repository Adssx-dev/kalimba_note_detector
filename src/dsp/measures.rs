
pub fn root_mean_square(data: &[f32]) -> f32 {
    f32::sqrt(data.iter()
        .fold(0.0,|partial_res, elem| partial_res + elem * elem)
        / data.len() as f32
    )
}


pub fn mean(data: &[f32]) -> f32 {
    f32::sqrt(data.iter()
        .fold(0.0,|partial_res, elem| partial_res + elem)
        / data.len() as f32
    )
}

pub fn min(data: &[f32]) -> f32 {
    data.into_iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone()
}

pub fn max(data: &[f32]) -> f32 {
    data.into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap().clone()
}
