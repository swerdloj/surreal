/* TODO: Custom file format for defining themes

example:


theme ExampleTheme:

fonts {
    Default = "path/to/font.ttf"
    Button = "some/specific/font.ttf"
}

styles {
    WidgetPadding = (10, 10, 5, 5) //left, right, top, bottom in pixels
    ViewPadding = 10 // equal to (10, 10, 10, 10)
}

colors {
    Primary = 0xff1234
    Secondary = (123, 1, 23)
    Accent = LightBlue
}

shapes {
    Button = Rounded(10) // rounded corner amount
    ClickAnimation = ColorFade {
        Style = Darken
        Duration = 200ms
    }
}

...

theme AnotherOneInSameFile:
...


*/

use crate::Color;

// TEMP: Import this from a file
// TODO: Create theme hot-reloading feature
pub const DEFAULT_THEME: Theme = Theme {
    view_padding: Padding {
        vertical: 10,
        horizontal: 20,
    },

    default_alignment: crate::Alignment::Center,

    widget_padding: Padding {
        vertical: 10,
        horizontal: 20,
    },

    colors: Colors {
        primary: Color::ALMOST_WHITE,
        secondary: Color::DARK_GRAY,
        background: Color::AUBERGINE,
        text: Color::ALMOST_WHITE,
    },

    text: Text {
        scale: 40.0
    },

    widget_styles: Widgets {
        buttons: Buttons {
            roundness: 50.0,
            circle_button_radius: 50,
        },
    },
};

////////////////////

pub struct Theme {
    pub view_padding: Padding,
    pub default_alignment: crate::Alignment,
    pub widget_padding: Padding,
    pub colors: Colors,
    pub text: Text,

    pub widget_styles: Widgets,
}

////////////////////

#[derive(Copy, Clone)]
pub enum PrimitiveStyle {
    Circle,
    Rectangle,
    RoundedRectangle {
        roundness: f32,
    },
}

pub struct Text {
    pub scale: f32,
}

pub struct Buttons {
    pub roundness: f32,
    pub circle_button_radius: u32,
}

pub struct Widgets {
    pub buttons: Buttons,
}

pub struct Padding {
    pub vertical: u32,
    pub horizontal: u32,
}

pub struct Colors {
    pub primary: Color,
    pub secondary: Color,
    // accent: Color,
    pub background: Color,
    pub text: Color,
}