// TODO: User can define colors, shapes, sizes/padding, font, etc.
// Maybe this can be parsed from a theme file to simplify the process

// The theme object would be passed to widgets for use when laying out
// and rendering

/* example:


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
    padding: Padding {
        vertical: 10,
        horizontal: 20,
    },

    colors: Colors {
        primary: Color::ALMOST_WHITE,
        secondary: Color::DARK_GRAY,
        background: Color::AUBERGINE,
        text: Color::ALMOST_WHITE,
    }
};

////////////////////

pub struct Theme {
    pub padding: Padding,
    pub colors: Colors,
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