extern crate clap;

use std::env;

use clap::{App, Arg};
use smartwp::draw_calendar;

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
            Arg::with_name("font")
                .long("font")
                .short("f")
                .required(false)
                .takes_value(true)
                .default_value("32")
                .help("Font size (default: 32px)"),
        )
        .arg(
            Arg::with_name("output")
                .long("output")
                .short("o")
                .required(false)
                .takes_value(true)
                .default_value("")
                .help("Output file path (default: overwrite)"),
        )
        .arg(
            Arg::with_name("right")
                .short("r")
                .required(false)
                .takes_value(false)
                .conflicts_with("c")
                .help("Stick widget to the right border"),
        )
        .arg(
            Arg::with_name("bottom")
                .short("b")
                .required(false)
                .takes_value(false)
                .conflicts_with("c")
                .help("Stick widget to the bottom border"),
        )
        .arg(
            Arg::with_name("center")
                .short("c")
                .required(false)
                .takes_value(false)
                .help("Place widget at center"),
        )
        .arg(
            Arg::with_name("theme")
                .short("t")
                .long("theme")
                .required(false)
                .takes_value(true)
                .possible_values(&["gb-dark", "gb-light", "monochrome"])
                .default_value("gb-dark")
                .help("Available themes: gb-dark, gb-light, monochrome"),
        )
        .arg(
            Arg::with_name("calendar")
                .long("calendar")
                .required(false)
                .takes_value(false)
                .conflicts_with("net")
                .conflicts_with("disk")
                .help("Draw calendar widget"),
        )
        .arg(
            Arg::with_name("net")
                .long("net")
                .required(false)
                .takes_value(false)
                .conflicts_with("calendar")
                .conflicts_with("disk")
                .help("Draw network widget"),
        )
        .arg(
            Arg::with_name("disk")
                .long("disk")
                .required(false)
                .takes_value(true)
                .conflicts_with("net")
                .conflicts_with("calendar")
                .help("Draw disk widget (provide disk name or partition)"),
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
        .value_of("font")
        .unwrap()
        .parse()
        .expect("Wrong float format!");
    let output = matches.value_of("output").unwrap();
    let theme = matches.value_of("theme");
    let current_dir = env::current_dir().expect("Не удалось получить текущую рабочую директорию");
    println!("Текущая рабочая директория: {:?}", current_dir);

    if matches.is_present("calendar") {
        draw_calendar(
            path,
            start_x,
            start_y,
            font_size,
            matches.is_present("r"),
            matches.is_present("b"),
            matches.is_present("c"),
            theme,
            output,
        );
    }
}
