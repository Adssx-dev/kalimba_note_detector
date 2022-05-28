use std::io::Write;
use crate::dsp::measures::root_mean_square;
use crate::dsp::normalize::{rms_normalize};
use crate::fft::{fft, fft_frequency_resolution};

#[derive(Debug, Clone)]
pub enum BasisDefinition {
    None, 
    TimeDomain {fs : f32, start_time : f32},
    TimeVector (Vec<f32>),
    FrequencyDomain {fs_step : f32},
}

impl BasisDefinition {
    pub fn get_sample_basis(&self, sample_id : usize) -> Option<f32> {
        match self  {
            BasisDefinition::None => None,
            BasisDefinition::TimeDomain{start_time, fs} => Some(start_time + (sample_id as f32) / fs ),
            BasisDefinition::TimeVector(basis_vector) => {
                assert!(sample_id < basis_vector.len(), "Index out of time vector bounds");
                Some(basis_vector[sample_id])
            },
            BasisDefinition::FrequencyDomain{fs_step} => Some(sample_id as f32 * fs_step),
        }
    }
}

pub struct Signal {
    pub samples : Vec<f32>,

    pub basis_definition : BasisDefinition,
}

impl Signal {

    pub fn new() -> Signal {
        Signal {
            samples : Vec::new(),
            basis_definition : BasisDefinition::None,
        }
    }

    pub fn from_vector(samples : &Vec<f32>, basis_definition : BasisDefinition) -> Signal {
        Signal {
            samples : samples.clone(),
            basis_definition,
        }
    }

    pub fn from_mp3(file_path : &str) -> Signal {
        let mut result = Signal::new();
        let data = std::fs::read(file_path).expect("Could not open file");
        let (header, samples) = puremp3::read_mp3(&data[..]).expect("Invalid MP3");
        for (left, _) in samples {
            result.samples.push(left);
        }
        result.basis_definition = BasisDefinition::TimeDomain{fs: header.sample_rate.hz() as f32,start_time: 0.0};
        result
    }

    pub fn root_mean_square(&self) -> f32 {
        root_mean_square(&self.samples)
    }

    pub fn normalize(&self) -> Signal {
        Signal {
            samples : rms_normalize(&self.samples, 1.0),
            basis_definition : self.basis_definition.clone()
        }
    }

    pub fn dump_csv(&self, file_path : &str) {
        let mut file = std::fs::File::create(file_path).expect("create failed");
        file.write_all("x,y".as_bytes()).expect("write failed");
        for idx in 0..self.samples.len() {
            if let Some(basis) = self.basis_definition.get_sample_basis(idx) {
                file.write_all(format!("\n{},{}", basis, self.samples[idx]).as_bytes()).expect("write failed");
            }
            else {
                file.write_all(format!("\n{},{}", idx, self.samples[idx]).as_bytes()).expect("write failed");
            }
        }
    }

    pub fn fft(&self) -> Signal {
        Signal {
            samples : fft(&self.samples),
            basis_definition : BasisDefinition::FrequencyDomain {fs_step : 
                if let BasisDefinition::TimeDomain{fs, start_time : _} = self.basis_definition {
                    fft_frequency_resolution(self.samples.len(), fs)
                }
                else {
                    1.0f32
                }
            }
        }
    }
}