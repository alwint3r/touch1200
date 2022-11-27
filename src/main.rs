use std::{env, process::exit};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Insufficient arguments!");

        exit(-1);
    }

    let port_name = &args[1];

    if let Ok(mut port) = serialport::new(port_name, 1200).open() {
        port.write_data_terminal_ready(false).unwrap();
    } else {
        eprintln!("Serial port {} is not found!", port_name);
        exit(-1);
    }
}
