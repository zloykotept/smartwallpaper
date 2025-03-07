use std::path::Path;

use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use chrono::{Datelike, Local};
use fs2::{available_space, total_space};
use image::{open, Rgb};
use imageproc::drawing::draw_text_mut;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use theme::Theme;
use time::util::days_in_month;
use time::OffsetDateTime;

pub mod theme;

static FONT_BYTES_REGULAR: &[u8] = include_bytes!("D:\\JetBrainsMono-Regular.ttf");
static FONT_BYTES_BOLD: &[u8] = include_bytes!("D:\\JetBrainsMono-Bold.ttf");
//static FONT_BYTES_SEMIBOLD: &[u8] = include_bytes!("D:\\JetBrainsMono-Semibold.ttf");
//static FONT_BYTES_ITALIC: &[u8] = include_bytes!("D:\\JetBrainsMono-Italic.ttf");

pub struct Config<'a> {
    pub file_in: &'a str,
    pub start_x: f32,
    pub start_y: f32,
    pub font_size: f32,
    pub right: bool,
    pub bottom: bool,
    pub center: bool,
    pub theme: Theme,
    pub file_out: &'a str,
}

pub fn draw_calendar(mut conf: Config) {
    //prepare values
    let now = Local::now();

    let first_day_of_month = now.with_day(1).unwrap().weekday().num_days_from_monday() + 1;

    let days_in_month = days_in_month(OffsetDateTime::now_local().unwrap().month(), now.year());
    let current_day = now.day();
    let current_year = now.year();
    let current_month = now.format("%B").to_string();

    //open image and font
    let mut img = open(conf.file_in).expect("Not an image").into_rgb8();
    let font_regular = FontRef::try_from_slice(FONT_BYTES_REGULAR as &[u8]).expect("Invalid font");
    let font_bold = FontRef::try_from_slice(FONT_BYTES_BOLD as &[u8]).expect("Invalid font");

    //normalize font
    let (img_width, img_height) = img.dimensions();
    let scale_factor = (img_width * img_height) / (1920 * 1080);

    conf.font_size *= (scale_factor as f32).sqrt();
    let scale = PxScale::from(conf.font_size);

    let mut offset_x: f32;
    let mut offset_y: f32;

    const WEEK_DAYS: &str = "Mon Tue Wed Thu Fri Sat Sun";
    let week_width = text_width(&font_bold, scale, WEEK_DAYS);
    let title = &format!("{} {}", current_month, current_year);
    let title_width = text_width(&font_regular, scale, title);

    //change position of widget using flags -r -b -c
    let bold_height = text_height(&font_bold, scale);
    let regular_height = text_height(&font_regular, scale);

    let numbers_lines: f32; //number of lines we can know in advance
    if first_day_of_month == 1 && days_in_month == 28 {
        numbers_lines = 4.0;
    } else if (first_day_of_month == 1 && days_in_month > 28)
        || (first_day_of_month != 1 && days_in_month == 28)
    {
        numbers_lines = 5.0;
    } else {
        numbers_lines = 6.0;
    }
    let widget_height =
        bold_height + (regular_height * (numbers_lines + 1.0)) + 10.0 * (numbers_lines + 1.0);

    if conf.center {
        conf.start_x = get_coord_centered(img_width as f32, week_width, 0.0);
        conf.start_y = get_coord_centered(img_height as f32, widget_height, 0.0);
    }
    if conf.right {
        conf.start_x = img_width as f32 - week_width - conf.start_x;
    }
    if conf.bottom {
        conf.start_y = img_height as f32 - widget_height - conf.start_y;
    }

    //calculate center related to days of week string
    offset_x = get_coord_centered(week_width, title_width, conf.start_x);

    //draw title
    draw_text_mut(
        &mut img,
        Rgb(conf.theme.color_highlighted),
        offset_x as i32,
        conf.start_y as i32,
        scale,
        &font_regular,
        title,
    );

    offset_y = conf.start_y + conf.font_size + 10.0;
    //draw week days
    draw_text_mut(
        &mut img,
        Rgb(conf.theme.color_headline),
        conf.start_x as i32,
        offset_y as i32,
        scale,
        &font_bold,
        WEEK_DAYS,
    );

    offset_y = offset_y + conf.font_size + 10.0;
    //draw lines of numbers
    let cell_width = week_width / 7.0;
    let mut days_iter = 1..=days_in_month;
    //first line is specific
    offset_x = conf.start_x;
    if first_day_of_month != 1 {
        for i in 1..=7 {
            if i < first_day_of_month {
                offset_x += cell_width;
            } else {
                let day = days_iter.next().unwrap();
                let color;

                if day as u32 == current_day {
                    color = conf.theme.color_highlighted;
                } else {
                    color = conf.theme.color_text;
                }
                let num_width = text_width(&font_regular, scale, &day.to_string());
                offset_x = get_coord_centered(cell_width, num_width, offset_x);

                draw_text_mut(
                    &mut img,
                    Rgb(color),
                    offset_x as i32,
                    offset_y as i32,
                    scale,
                    &font_bold,
                    &day.to_string(),
                );

                offset_x = conf.start_x + cell_width * i as f32;
            }
        }
    }

    //draw other days
    offset_y = offset_y + conf.font_size + 10.0;
    offset_x = conf.start_x;
    loop {
        for i in 1..=7 {
            let Some(day) = days_iter.next() else {
                break;
            };
            let num_width = text_width(&font_regular, scale, &day.to_string());
            let color;

            if day as u32 == current_day {
                color = conf.theme.color_highlighted;
            } else {
                color = conf.theme.color_text;
            }
            offset_x = get_coord_centered(cell_width, num_width, offset_x);
            draw_text_mut(
                &mut img,
                Rgb(color),
                offset_x as i32,
                offset_y as i32,
                scale,
                &font_bold,
                &day.to_string(),
            );
            offset_x = conf.start_x + cell_width * i as f32;
        }
        if days_iter.is_empty() {
            break;
        }
        offset_y += conf.font_size + 10.0;
        offset_x = conf.start_x;
    }

    if conf.file_out.is_empty() {
        img.save(conf.file_in).expect("Error while saving file");
    } else {
        let path = Path::new(conf.file_in).parent().unwrap().display();
        img.save(format!("{}\\{}", path, conf.file_out))
            .expect("Error while saving file");
    }
}

pub fn draw_inet(mut conf: Config, iface_name: String) {
    //open image and font
    let mut img = open(conf.file_in).expect("Not an image").into_rgb8();
    let font_regular = FontRef::try_from_slice(FONT_BYTES_REGULAR as &[u8]).expect("Invalid font");
    let font_bold = FontRef::try_from_slice(FONT_BYTES_BOLD as &[u8]).expect("Invalid font");

    //normalize font
    let (img_width, img_height) = img.dimensions();
    let scale_factor = (img_width * img_height) / (1920 * 1080);

    conf.font_size *= (scale_factor as f32).sqrt();
    let scale = PxScale::from(conf.font_size);

    //info for widget
    let hostname_binding = hostname::get().unwrap_or("".into());
    let hostname = hostname_binding.to_str().unwrap();
    //let hostname = "CHLEN";
    //let local_ip = local_ip().unwrap().to_string();

    let interface_binding = NetworkInterface::show()
        .unwrap()
        .into_iter()
        .find(|iface| iface.name == iface_name)
        .unwrap();
    let interface_name = interface_binding.name.clone();
    let local_ip = interface_binding.addr[1].ip().to_string();

    let global_ip;
    match ureq::get("https://api64.ipify.org").call() {
        Ok(mut res) => global_ip = res.body_mut().read_to_string().unwrap(),
        Err(_) => global_ip = "".to_string(),
    }

    let hostname_text = format!("Hostname: {}", hostname);
    let local_ip_text = format!("Local IP: {}", local_ip);
    let interface_name_text = format!("Interface: {}", interface_name);
    let global_ip_text = format!("Global IP: {}", global_ip);
    let texts = [
        interface_name_text,
        hostname_text,
        local_ip_text,
        global_ip_text,
    ];

    //positioning
    let widget_width = texts
        .iter()
        .map(|line| text_width(&font_regular, scale, line) as u32)
        .max()
        .unwrap() as f32;
    let regular_height = text_height(&font_regular, scale);
    let widget_height = regular_height * 4.0 + 10.0 * 4.0;

    if conf.center {
        conf.start_x = get_coord_centered(img_width as f32, widget_width, 0.0);
        conf.start_y = get_coord_centered(img_height as f32, widget_height, 0.0);
    }
    if conf.bottom {
        conf.start_y = img_height as f32 - widget_height - conf.start_y;
    }
    if conf.right {
        conf.start_x = img_width as f32 - widget_width - conf.start_x;
    }

    //draw widget
    for text in texts {
        draw_text_mut(
            &mut img,
            Rgb(conf.theme.color_text),
            conf.start_x as i32,
            conf.start_y as i32,
            scale,
            &font_bold,
            &text,
        );
        conf.start_y += conf.font_size + 10.0;
    }

    //save file
    if conf.file_out.is_empty() {
        img.save(conf.file_in).expect("Error while saving file");
    } else {
        let path = Path::new(conf.file_in).parent().unwrap().display();
        img.save(format!("{}\\{}", path, conf.file_out))
            .expect("Error while saving file");
    }
}

pub fn draw_disk(mut conf: Config, partition: String) {
    //open image and font
    let mut img = open(conf.file_in).expect("Not an image").into_rgb8();
    let font_regular = FontRef::try_from_slice(FONT_BYTES_REGULAR as &[u8]).expect("Invalid font");

    //normalize font
    let (img_width, img_height) = img.dimensions();
    let scale_factor = (img_width * img_height) / (1920 * 1080);

    conf.font_size *= (scale_factor as f32).sqrt();
    let scale = PxScale::from(conf.font_size);

    //info for widget
    let total = total_space(&partition).expect("Failed to get metadata from partition") as f64
        / 1_073_741_824.0;
    let free = available_space(&partition).expect("Failed to get metadata from partition") as f64
        / 1_073_741_824.0;
    let widget_text = format!("In {} {:.1}Gb free from {:.1}Gb", partition, free, total);

    // positioning
    let widget_width = text_width(&font_regular, scale, &widget_text);
    let widget_height = text_height(&font_regular, scale);

    if conf.center {
        conf.start_x = get_coord_centered(img_width as f32, widget_width, 0.0);
        conf.start_y = get_coord_centered(img_height as f32, widget_height, 0.0);
    }
    if conf.bottom {
        conf.start_y = img_height as f32 - widget_height - conf.start_y;
    }
    if conf.right {
        conf.start_x = img_width as f32 - widget_width - conf.start_x;
    }

    //draw widget
    draw_text_mut(
        &mut img,
        Rgb(conf.theme.color_text),
        conf.start_x as i32,
        conf.start_y as i32,
        scale,
        &font_regular,
        &widget_text,
    );

    if conf.file_out.is_empty() {
        img.save(conf.file_in).expect("Error while saving file");
    } else {
        let path = Path::new(conf.file_in).parent().unwrap().display();
        img.save(format!("{}\\{}", path, conf.file_out))
            .expect("Error while saving file");
    }
}

fn text_width(font: &FontRef, scale: PxScale, text: &str) -> f32 {
    let scaled_font = font.as_scaled(scale);
    text.chars()
        .map(|c| {
            let glyph = scaled_font.scaled_glyph(c);
            scaled_font.h_advance(glyph.id)
        })
        .sum()
}

fn text_height(font: &FontRef, scale: PxScale) -> f32 {
    let scaled_font = font.as_scaled(scale);
    scaled_font.height()
}

fn get_coord_centered(w_master: f32, w_slave: f32, master_start_x: f32) -> f32 {
    (w_master - w_slave) / 2.0 + master_start_x
}
