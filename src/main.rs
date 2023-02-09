use clap::Parser;
use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long, default_value_t = 1.0)]
    volume: f32,
}

fn main() {
    let args = Args::parse();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = BufReader::new(File::open(args.path).unwrap());
    let source = Decoder::new(file).unwrap();

    stream_handle.play_raw(source.convert_samples()).unwrap();

    sleep(Duration::from_secs(2));
}
