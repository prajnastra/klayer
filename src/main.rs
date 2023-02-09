use clap::Parser;

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

    println!("path {}!", args.path);
    println!("volume {}!", args.volume);
}
