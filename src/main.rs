extern crate clap;

use clap::{Arg, App};
use std::process::Command;

fn main() {
        
    let matches = App::new("mypipe")
    .version("2.0")
    .author("Clement Delaunay <clementdelaunay3002@gmail.com")
    .about("Pipe System")
    .arg(Arg::with_name("input")
            .short("i")
            .long("in")
            .help("Set input argment")
            .takes_value(true)
            .required(true)
    )
    .arg(Arg::with_name("output")
            .short("o")
            .long("out")
            .help("Set output argment")
            .takes_value(true)
            .required(true)
    )
    .get_matches();

    let input_pipe = matches.value_of("input").unwrap();
    let output_pipe = matches.value_of("output").unwrap();

    let end_in = Command::new(input_pipe)
    .output()
    .expect("Error !");

    let get_std = String::from_utf8_lossy(&end_in.stdout).to_string();

    let end_out = Command::new(output_pipe)
    .arg(get_std)
    .output()
    .expect("Error !");

    println!("{}", String::from_utf8_lossy(&end_out.stdout).to_string());
    
}
