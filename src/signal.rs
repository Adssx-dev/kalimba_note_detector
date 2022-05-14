use std::io::Write;

pub struct Signal {
    samples : Vec<Vec<f32>>,

    sample_rate : u32,
}

impl Signal {

    pub fn new() -> Signal {
        let mut signal = Signal {
            samples : Vec::new(),
            sample_rate : 0
        };

        signal.samples.push(Vec::new()); // Left
        signal.samples.push(Vec::new()); // Right

        signal
    }

    pub fn from_mp3(file_path : &str) -> Signal {
        let mut result = Signal::new();
        let data = std::fs::read(file_path).expect("Could not open file");
        let (header, samples) = puremp3::read_mp3(&data[..]).expect("Invalid MP3");
        for (left, right) in samples {
            result.samples[0].push(left);
            result.samples[1].push(right);
        }
        result.sample_rate = header.sample_rate.hz();
        result
    }

    pub fn dump_csv(&self, file_path : &str) {
        let mut file = std::fs::File::create(file_path).expect("create failed");
        file.write_all("left,right".as_bytes()).expect("write failed");
        for idx in 0..self.samples[0].len() {
            file.write_all(format!("\n{},{}", self.samples[0][idx], self.samples[1][idx]).as_bytes()).expect("write failed");
        }
    }
}