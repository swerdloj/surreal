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