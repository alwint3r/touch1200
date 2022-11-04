use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Insufficient arguments!");
    }

    let port_name = &args[1];

    let mut port = serialport::new(port_name, 1200)
        .open()
        .unwrap();
    port.write_data_terminal_ready(false).unwrap();
}
