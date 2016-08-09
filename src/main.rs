extern crate md2;

use std::env;
use std::process;

use md2::md2::md2file;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: md2 <filename>");
        process::exit(64); // EX_USAGE, see sysexits(3) manpage.
    }

    match md2file(&args[1]) {
        Ok(cksum) => print!("{}\n", md2::md2::hex_string(&cksum)),
        Err(msg) => print!("{:?}\n", msg),
    };
}
