mod signal;
mod signal_iterator;
mod dsp;
use crate::dsp::fft;
use signal_iterator::SignalIterator;

fn main() {
    let sig = signal::Signal::from_mp3("data/A.mp3");
    let normalized_sig = sig.normalize();
    sig.dump_csv("data.csv") ;
    normalized_sig.dump_csv("data3.csv") ;

    let sig_fft = sig.fft();
    sig_fft.dump_csv("fft.csv");

    let it = SignalIterator::new(&sig, 4096);
    for val in it {
        let a  = val.fft();
    }
}
