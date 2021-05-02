mod cue;

use clap::{App, Arg};

fn main() {
    let matches = App::new("cuemake")
        .version("0.1.0")
        .author("xadrianzetx")
        .about(".cue file generator for .bin or .BIN PSX ROMs")
        .args(&[
            Arg::with_name("binary")
                .short("b")
                .long("binary")
                .takes_value(true)
                .required(true)
                .help("Path to .bin or .BIN PSX ROM"),
            Arg::with_name("output")
                .short("o")
                .long("output")
                .takes_value(true)
                .help("Path to save generated .cue file. Saves into current dir if not specified")
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
