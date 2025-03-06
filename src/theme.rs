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
            "monochrome-dark" => {
                color_text = [168, 168, 168]; // #a8a8a8
                color_highlighted = [231, 231, 231]; // #e7e7e7
                color_headline = [154, 154, 154]; // #9a9a9a
            }
            "monochrome-light" => {
                color_text = [89, 89, 89]; // #595959
                color_highlighted = [25, 25, 25]; // #191919
                color_headline = [77, 77, 77]; // #4d4d4d
            }
            _ => panic!("No such theme!"),
        }

        Theme {
            color_text,
            color_highlighted,
            color_headline,
        }
    }
}
