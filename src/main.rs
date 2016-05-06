extern crate getopts;

#[macro_use]
extern crate lazy_static;

mod template;
mod segment;
mod functions;

// Std
use std::env;
use std::fs::File;

// Getopts
use getopts::Options;

// Modules
use functions::funcs;

// Print usage
fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optmulti("i", "input", "input files that will be parsed", "FILE1 FILE2 ...");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let filenames = matches.opt_strs("i");
    if filenames.len() == 0 {
        println!("Must specify files to parse");
        return;
    }
    println!("Parsing: {:?}", &filenames);

    for filename in filenames {
        if let Ok(t) = template::parse(&filename) {
            let output = t.render();
            println!("{}", output);
        }
    }
}
