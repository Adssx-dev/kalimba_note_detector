mod signal;


fn main() {
    let sig = signal::Signal::from_mp3("data/all notes.mp3");
    sig.dump_csv("data.csv") ;
}
