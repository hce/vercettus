use argparse::*;
use std::io::{Read, Write};
use std::process::exit;

fn main() {
    let mut savegame_to_load: Option<String> = None;
    let mut savegame_to_patch: Option<String> = None;
    let mut yaml_to_write: Option<String> = None;
    let mut yaml_to_read: Option<String> = None;
    let mut writer = Vec::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Vice City savegame fiddlin'");
        ap.refer(&mut savegame_to_load).add_option(
            &["-c", "--convert"],
            StoreOption,
            "Convert a savegame file to yaml",
        );
        ap.refer(&mut savegame_to_patch).add_option(
            &["-p", "--patch"],
            StoreOption,
            "Patch an existing savegame file",
        );

        ap.refer(&mut yaml_to_write).add_option(
            &["-o", "--yaml-out"],
            StoreOption,
            "YAML file to create",
        );
        ap.refer(&mut yaml_to_read).add_option(
            &["-i", "--yaml-in"],
            StoreOption,
            "YAML file to read",
        );
        ap.parse_args_or_exit();
        ap.print_help("vercettus", &mut writer).unwrap();
    }
    if savegame_to_load.is_some() && savegame_to_patch.is_some() {
        eprintln!("-c and -p are mutually exclusive");
        exit(2);
    }
    if yaml_to_read.is_some() && yaml_to_write.is_some() {
        eprintln!("-o and -i are mutually exclusive");
        exit(2);
    }
    if savegame_to_load.is_some() && yaml_to_write.is_none() {
        eprintln!("-c option requires -o");
        exit(2);
    }
    if savegame_to_patch.is_some() && yaml_to_read.is_none() {
        eprintln!("-p option requires -i");
        exit(2);
    }
    if yaml_to_write.is_some() && savegame_to_load.is_none() {
        eprintln!("-o option requires -c");
        exit(2);
    }
    if yaml_to_read.is_some() && savegame_to_patch.is_none() {
        eprintln!("-i option requires -p");
        exit(2);
    }
    if savegame_to_load.is_none() && savegame_to_patch.is_none() {
        eprintln!("{}", String::from_utf8(writer).unwrap());
        eprintln!("No operation specified.");
        exit(2);
    }

    if let Some(file_name) = savegame_to_load {
        let f = std::fs::File::open(&file_name);
        if f.is_err() {
            eprintln!("Unable to open {}: {:?}", file_name, f.err().unwrap());
            exit(1);
        }
        let mut f = f.unwrap();
        let mut buf = Vec::new();
        f.read_to_end(&mut buf).unwrap();
        let sg = vercettus::savegame::parse_savegame(&buf).expect("Parse savegame failed");
        let yaml = serde_yaml::to_string(&sg).unwrap();
        let out_fn = yaml_to_write.unwrap();
        let out_f = std::fs::File::create(&out_fn);
        if out_f.is_err() {
            eprintln!(
                "Unable to create output file {}: {:?}",
                out_fn,
                out_f.err().unwrap()
            );
            exit(1);
        }
        let mut out_f = out_f.unwrap();
        if let Err(e) = out_f.write_all(yaml.as_bytes()) {
            eprintln!("Unable to write to {}: {:?}", out_fn, e);
            exit(1);
        }
        eprintln!("{} written to {}.", file_name, out_fn);
        eprintln!(
            "You may now edit {} with the text editor of your choice",
            out_fn
        );
        eprintln!("and re-invoke this binary to patch your savegame.");
    }
}
