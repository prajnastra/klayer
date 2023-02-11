use clap::Parser;
use rodio::{Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,

    #[arg(short, long, default_value_t = 0.6)]
    volume: f32,
}

fn main() {
    let args = Args::parse();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    // add a source
    let file = BufReader::new(File::open(args.path).unwrap());
    let source = Decoder::new(file).unwrap();

    sink.append(source);
    sink.set_volume(args.volume);
    sink.sleep_until_end();
}
