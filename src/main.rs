fn main() {
    let data = std::fs::read("data/all notes.mp3").expect("Could not open file");
    let (header, samples) = puremp3::read_mp3(&data[..]).expect("Invalid MP3");
    for (left, right) in samples {
        println!("{} ; {}", left, right);
    }
}
