use std::fs::File;
use std::io::{Write, Result, Error, ErrorKind};
use std::path::{Path, PathBuf};

pub fn make_cue(rom_name: String, output: Option<&str>) -> Result<()> {

    if !rom_name.ends_with(".bin") & !rom_name.ends_with(".BIN") {
        return Err(Error::from(ErrorKind::InvalidData))
    }

    // switch extensions from .bin to .cue
    let file_stem = Path::new(&rom_name).file_stem().unwrap();
    let mut cue_name = String::from(file_stem.to_str().unwrap());
    cue_name.push_str(".cue");
    
    let output_file = match output {
        Some(path) => {
            let abs_path = Path::new(path);
            abs_path.join(&cue_name)
        },
        None => PathBuf::from(&cue_name)
    };

    // TODO check if payload correct on PSX
    let mut payload = String::from("FILE  BINARY\nTRACK 01 MODE2/2352\nINDEX 01 00:00:00");
    payload.insert_str(5, &rom_name);
    let mut f = File::create(output_file)?;
    f.write(payload.as_bytes())?;

    Ok(())
}