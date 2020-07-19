use std::io::Read;

fn main() {
    println!("Hello, world!");
    let mut f =
        std::fs::File::open(r"C:\Users\hc-mi\Documents\GTA Vice City User Files\GTAVCsf5.b")
            .expect("Unable to open file");
    let mut buf = Vec::new();
    f.read_to_end(&mut buf).unwrap();
    let sg = vercettus::savegame::parse_savegame(&buf).expect("Parse savegame failed");
    println!("{}", serde_yaml::to_string(&sg).unwrap());
}
