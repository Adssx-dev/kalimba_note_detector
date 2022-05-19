mod signal;
mod dsp;


fn main() {
    let sig = signal::Signal::from_mp3("data/harry potter.mp3");
    let (rms_l, rms_r) = sig.root_mean_square();
    let normalized_sig = sig.normalize();
    println!("{}, {}",rms_l, rms_r );
    sig.dump_csv("data.csv") ;
    normalized_sig.dump_csv("data3.csv") ;
}
