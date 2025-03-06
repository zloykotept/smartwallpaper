pub struct Theme {
    pub color_text: [u8; 3],
    pub color_highlighted: [u8; 3],
    pub color_headline: [u8; 3],
}

impl Theme {
    pub fn new(name: &str) -> Self {
        let color_text: [u8; 3];
        let color_headline: [u8; 3];
        let color_highlighted: [u8; 3];

        match name {
            "gb-dark" => {
                color_text = [251, 241, 199];
                color_highlighted = [254, 128, 25];
                color_headline = [235, 219, 178];
            }
            "gb-light" => {
                color_text = [146, 131, 116];
                color_highlighted = [254, 128, 25];
                color_headline = [124, 111, 100];
            }
            "monochrome" => {
                color_text = [52, 52, 52];
                color_highlighted = [254, 128, 25];
                color_headline = [141, 141, 141];
            }
            _ => panic!("ERROR WHILE CREATING THEME"),
        }

        Theme {
            color_text,
            color_highlighted,
            color_headline,
        }
    }
}
