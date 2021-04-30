mod cue;

use clap::{App, Arg};

fn main() {
    let matches = App::new("cuemake")
        .args(&[
            Arg::with_name("binary")
                .short("b")
                .long("binary")
                .takes_value(true)
                .required(true),
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
        ])
        .get_matches();
    
    let bin = matches.value_of("binary").unwrap();
    let output = matches.value_of("output");
    let result = cue::make_cue(String::from(bin), output);

    match result {
        Ok(_) => println!("{}", "Created .cue file"),
        Err(_) => println!("{}", "Failed to create .cue file")
    };
}
