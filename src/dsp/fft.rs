use rustfft::{FftPlanner, num_complex::Complex};

fn next_power_2(input_number : usize) -> usize {
    let mut number = input_number;
    let mut next_pow = 1;
    while number > 0 {
        number = number >> 1;
        next_pow = next_pow << 1;
    };
    next_pow
}


pub fn fft(data : &[f32]) -> Vec<f32> {
    let mut planner = FftPlanner::<f32>::new();
    let fft_length = next_power_2(data.len());

    let fft = planner.plan_fft_forward(fft_length);

    let mut buffer_complex = vec![Complex{re : 0.0f32, im : 0.0f32}; fft_length];

    for idx in 0..data.len() {
        buffer_complex[idx].re = data[idx];
    }
    
    fft.process(&mut buffer_complex);

    let buffer_real : Vec<f32> = buffer_complex.iter().take(fft_length / 2).map(|f| f.re.abs()).collect();

    buffer_real
}

pub fn fft_frequency_resolution(data_length : usize, original_fs : f32) -> f32 {
    original_fs / (next_power_2(data_length) as f32)
}