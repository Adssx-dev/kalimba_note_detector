use std::io::Write;
use crate::dsp::measures::root_mean_square;
use crate::dsp::normalize::{rms_normalize};

pub struct Signal {
    left_samples : Vec<f32>,
    right_samples : Vec<f32>,

    sample_rate : u32,
}

impl Signal {

    pub fn new() -> Signal {
        Signal {
            left_samples : Vec::new(),
            right_samples : Vec::new(),
            sample_rate : 0
        }
    }

    pub fn from_mp3(file_path : &str) -> Signal {
        let mut result = Signal::new();
        let data = std::fs::read(file_path).expect("Could not open file");
        let (header, samples) = puremp3::read_mp3(&data[..]).expect("Invalid MP3");
        for (left, right) in samples {
            result.left_samples.push(left);
            result.right_samples.push(right);
        }
        result.sample_rate = header.sample_rate.hz();
        result
    }

    pub fn root_mean_square(&self) -> (f32, f32) {
        
        let left_rms = root_mean_square(&self.left_samples);
        let right_rms = root_mean_square(&self.right_samples);
        (left_rms, right_rms)
    }

    pub fn normalize(&self) -> Signal {
        let mut s = Signal {
            left_samples : Vec::new(),
            right_samples : Vec::new(),
            sample_rate : self.sample_rate
        };
        s.left_samples = rms_normalize(&self.left_samples, 1.0);
        s.right_samples = rms_normalize(&self.right_samples, 1.0);
        s
    }

    pub fn dump_csv(&self, file_path : &str) {
        let mut file = std::fs::File::create(file_path).expect("create failed");
        file.write_all("left,right".as_bytes()).expect("write failed");
        for idx in 0..self.left_samples.len() {
            file.write_all(format!("\n{},{}", self.left_samples[idx], self.right_samples[idx]).as_bytes()).expect("write failed");
        }
    }
}