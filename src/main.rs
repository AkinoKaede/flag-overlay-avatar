use clap::{load_yaml, App};
use flag_overlay_avatar::overlay;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = App::from(yaml).get_matches();

    let avatar = matches.value_of("avatar").unwrap();
    let flag = matches.value_of("flag").unwrap();
    let output = matches.value_of("output").unwrap();

    let avatar_image = image::open(avatar).unwrap().to_rgba8();
    let flag_image = image::open(flag).unwrap().to_rgba8();
    let output_image = overlay(avatar_image, flag_image);

    match output_image.save(output) {
        Ok(_) => println!("Saved to {}", output),
        Err(e) => println!("Error: {}", e),
    }
}
