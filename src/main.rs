use clap::Parser;
use std::{process::exit, thread::sleep, time::Duration};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    port: String,

    #[arg(short, long)]
    dtr: bool,
}

fn main() {
    let args = Args::parse();

    if let Ok(mut port) = serialport::new(args.port.clone(), 1200).open() {
        port.write_data_terminal_ready(args.dtr).unwrap();

        sleep(Duration::from_secs(3));
    } else {
        eprintln!("Serial port {} is not found!", args.port);
        exit(-1);
    }
}
