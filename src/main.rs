extern crate clap;

use clap::{App, Arg};
use smartwp::draw_callendar;

fn main() {
    let matches = App::new("SmartWallpaper")
        .version("0.1.0")
        .author("ZloyKot")
        .about("Standart description")
        .arg(
            Arg::with_name("image_path")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Path to wallpaper"),
        )
        .arg(
            Arg::with_name("start_x")
                .required(false)
                .takes_value(true)
                .default_value("0")
                .index(2)
                .help("Start position x coordinate (from left-up corner)"),
        )
        .arg(
            Arg::with_name("start_y")
                .required(false)
                .takes_value(true)
                .default_value("0")
                .index(3)
                .help("Start position y coordinate (from left-up corner)"),
        )
        .arg(
            Arg::with_name("font_size")
                .required(false)
                .takes_value(true)
                .default_value("32")
                .index(4)
                .help("Font size (default: 32px)"),
        )
        .arg(
            Arg::with_name("output_path")
                .required(false)
                .takes_value(true)
                .default_value("")
                .index(5)
                .help("Output file path (default: overwrite)"),
        )
        .get_matches();

    let path = matches.value_of("image_path").unwrap();
    let start_x: f32 = matches
        .value_of("start_x")
        .unwrap()
        .parse()
        .expect("Wrong integer format!");
    let start_y: f32 = matches
        .value_of("start_y")
        .unwrap()
        .parse()
        .expect("Wrong integer format!");
    let font_size: f32 = matches
        .value_of("font_size")
        .unwrap()
        .parse()
        .expect("Wrong float format!");
    let output = matches.value_of("output_path").unwrap();

    draw_callendar(path, start_x, start_y, font_size, output);
}
