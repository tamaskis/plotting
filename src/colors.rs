use plotly::color::Rgba;

/// Named colors.
///
/// # Note
///
/// This enum is a direct re-implementation of the [`plotly::common::color::NamedColor`] enum from
/// the [`plotly`] crate (Ref. \[1\]). As such, we have included the license of the [`plotly`] crate
/// in the
/// [`src/plotly_licenses`](https://github.com/tamaskis/plotting/tree/main/src/plotly_licenses/LICENSE)
/// folder.
///
/// # References
///
/// * \[1\] <https://docs.rs/plotly/latest/plotly/common/color/enum.NamedColor.html>
/// * \[2\] <https://www.w3schools.com/cssref/css_colors.asp>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum NamedColor {
    AliceBlue = 0xF0F8FF,
    AntiqueWhite = 0xFAEBD7,
    Aqua,
    Aquamarine = 0x7FFFD4,
    Azure = 0xF0FFFF,
    Beige = 0xF5F5DC,
    Bisque = 0xFFE4C4,
    Black = 0x000000,
    BlanchedAlmond = 0xFFEBCD,
    Blue = 0x0000FF,
    BlueViolet = 0x8A2BE2,
    Brown = 0xA52A2A,
    BurlyWood = 0xDEB887,
    CadetBlue = 0x5F9EA0,
    Chartreuse = 0x7FFF00,
    Chocolate = 0xD2691E,
    Coral = 0xFF7F50,
    CornflowerBlue = 0x6495ED,
    CornSilk = 0xFFF8DC,
    Crimson = 0xDC143C,
    Cyan = 0x00FFFF,
    DarkBlue = 0x00008B,
    DarkCyan = 0x008B8B,
    DarkGoldenrod = 0xB8860B,
    DarkGray = 0xA9A9A9,
    DarkGreen = 0x006400,
    DarkGrey,
    DarkKhaki = 0xBDB76B,
    DarkMagenta = 0x8B008B,
    DarkOliveGreen = 0x556B2F,
    DarkOrange = 0xFF8C00,
    DarkOrchid = 0x9932CC,
    DarkRed = 0x8B0000,
    DarkSalmon = 0xE9967A,
    DarkSeaGreen = 0x8FBC8F,
    DarkSlateBlue = 0x483D8B,
    DarkSlateGray = 0x2F4F4F,
    DarkSlateGrey,
    DarkTurquoise = 0x00CED1,
    DarkViolet = 0x9400D3,
    DeepPink = 0xFF1493,
    DeepSkyBlue = 0x00BFFF,
    DimGray = 0x696969,
    DimGrey,
    DodgerBlue = 0x1E90FF,
    FireBrick = 0xB22222,
    FloralWhite = 0xFFFAF0,
    ForestGreen = 0x228B22,
    Fuchsia,
    Gainsboro = 0xDCDCDC,
    GhostWhite = 0xF8F8FF,
    Gold = 0xFFD700,
    Goldenrod = 0xDAA520,
    Gray = 0x808080,
    Green = 0x008000,
    GreenYellow = 0xADFF2F,
    Grey,
    Honeydew = 0xF0FFF0,
    HotPink = 0xFF69B4,
    IndianRed = 0xCD5C5C,
    Indigo = 0x4B0082,
    Ivory = 0xFFFFF0,
    Khaki = 0xF0E68C,
    Lavender = 0xE6E6FA,
    LavenderBlush = 0xFFF0F5,
    LawnGreen = 0x7CFC00,
    LemonChiffon = 0xFFFACD,
    LightBlue = 0xADD8E6,
    LightCoral = 0xF08080,
    LightCyan = 0xE0FFFF,
    LightGoldenrodYellow = 0xFAFAD2,
    LightGray = 0xD3D3D3,
    LightGreen = 0x90EE90,
    LightGrey,
    LightPink = 0xFFB6C1,
    LightSalmon = 0xFFA07A,
    LightSeaGreen = 0x20B2AA,
    LightSkyBlue = 0x87CEFA,
    LightSlateGray = 0x778899,
    LightSlateGrey,
    LightSteelBlue = 0xB0C4DE,
    LightYellow = 0xFFFFE0,
    Lime = 0x00FF00,
    LimeGreen = 0x32CD32,
    Linen = 0xFAF0E6,
    Magenta = 0xFF00FF,
    Maroon = 0x800000,
    MediumAquamarine = 0x66CDAA,
    MediumBlue = 0x0000CD,
    MediumOrchid = 0xBA55D3,
    MediumPurple = 0x9370DB,
    MediumSeaGreen = 0x3CB371,
    MediumSlateBlue = 0x7B68EE,
    MediumSpringGreen = 0x00FA9A,
    MediumTurquoise = 0x48D1CC,
    MediumVioletRed = 0xC71585,
    MidnightBlue = 0x191970,
    MintCream = 0xF5FFFA,
    MistyRose = 0xFFE4E1,
    Moccasin = 0xFFE4B5,
    NavajoWhite = 0xFFDEAD,
    Navy = 0x000080,
    OldLace = 0xFDF5E6,
    Olive = 0x808000,
    OliveDrab = 0x6B8E23,
    Orange = 0xFFA500,
    OrangeRed = 0xFF4500,
    Orchid = 0xDA70D6,
    PaleGoldenrod = 0xEEE8AA,
    PaleGreen = 0x98FB98,
    PaleTurquoise = 0xAFEEEE,
    PaleVioletRed = 0xDB7093,
    PapayaWhip = 0xFFEFD5,
    PeachPuff = 0xFFDAB9,
    Peru = 0xCD853F,
    Pink = 0xFFC0CB,
    Plum = 0xDDA0DD,
    PowderBlue = 0xB0E0E6,
    Purple = 0x800080,
    RebeccaPurple = 0x663399,
    Red = 0xFF0000,
    RosyBrown = 0xBC8F8F,
    RoyalBlue = 0x4169E1,
    SaddleBrown = 0x8B4513,
    Salmon = 0xFA8072,
    SandyBrown = 0xF4A460,
    SeaGreen = 0x2E8B57,
    Seashell = 0xFFF5EE,
    Sienna = 0xA0522D,
    Silver = 0xC0C0C0,
    SkyBlue = 0x87CEEB,
    SlateBlue = 0x6A5ACD,
    SlateGray = 0x708090,
    SlateGrey,
    Snow = 0xFFFAFA,
    SpringGreen = 0x00FF7F,
    SteelBlue = 0x4682B4,
    Tan = 0xD2B48C,
    Teal = 0x008080,
    Thistle = 0xD8BFD8,
    Tomato = 0xFF6347,
    Turquoise = 0x40E0D0,
    Violet = 0xEE82EE,
    Wheat = 0xF5DEB3,
    White = 0xFFFFFF,
    WhiteSmoke = 0xF5F5F5,
    Yellow = 0xFFFF00,
    YellowGreen = 0x9ACD32,
}

/// Color.
///
/// # Constructors from color representations
///
/// | From | Constructor |
/// | ---- | ----------- |
/// | RGB | [`Color::rgb`] |
/// | RGBA | [`Color::rgba`] |
/// | Named Color | [`Color::named`] |
/// | Hexadecimal Literal | [`Color::hex_literal`] |
/// | Hexadecimal String | [`Color::hex_str`] |
///
/// # Default constructor
///
/// [`Color::default`] constructs a color with RGB values of `0, 0, 0` (black) and an alpha value of
/// `1.0` (opaque).
///
/// # Other methods
///
/// | Method | Description |
/// | ------ | ----------- |
/// | [`Color::alpha`] | Set the opacity/transparency of the color. |
/// | [`Color::to_plotly_rgba`] | Convert the color to an [`Rgba`] from the [`plotly`] crate. |
#[derive(Debug, PartialEq)]
pub struct Color {
    /// Red component.
    r: u8,

    /// Green component.
    g: u8,

    /// Blue component.
    b: u8,

    /// Opacity
    ///
    /// * `0.0` corresponds to completely transparent (fully see-through).
    /// * `1.0` corresponds to completely opaque (fully solid).
    a: f64,
}

impl Color {
    /// Construct a color from RGB (red, green, blue) values.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component.
    /// * `g` - Green component.
    /// * `b` - Blue component.
    ///
    /// # Returns
    ///
    /// Color.
    pub fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b, a: 1.0 }
    }

    /// Construct a color from RGBA (red, green, blue, alpha) values.
    ///
    /// # Arguments
    ///
    /// * `r` - Red component.
    /// * `g` - Green component.
    /// * `b` - Blue component.
    /// * `a` - Alpha (opacity) between 0 (transparent) and 1 (opaque). Inputs outside the range
    ///   `[0, 1]` are clamped.
    ///
    /// # Returns
    ///
    /// Color.
    pub fn rgba(r: u8, g: u8, b: u8, a: f64) -> Color {
        Color::rgb(r, g, b).alpha(a)
    }

    /// Construct a color from a named color.
    ///
    /// # Arguments
    ///
    /// * `color` - Named color.
    ///
    /// # Returns.
    ///
    /// Color.
    pub fn named(color: NamedColor) -> Color {
        // Perform mappings for aliased colors.
        let color = match color {
            NamedColor::Aqua => NamedColor::Cyan,
            NamedColor::DarkGrey => NamedColor::DarkGray,
            NamedColor::DarkSlateGrey => NamedColor::DarkSlateGray,
            NamedColor::DimGrey => NamedColor::DimGray,
            NamedColor::Fuchsia => NamedColor::Magenta,
            NamedColor::Grey => NamedColor::Gray,
            NamedColor::LightGrey => NamedColor::LightGray,
            NamedColor::LightSlateGrey => NamedColor::LightSlateGray,
            NamedColor::SlateGrey => NamedColor::SlateGray,
            _ => color, // Return the original if no mapping is needed
        };

        // Cast the color to a u32.
        let hex = color as u32;

        // Extract the red, green, and blue components.
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;

        // Construct the color from the RGB components.
        Color::rgb(r, g, b)
    }

    /// Construct a color from a hexadecimal literal.
    ///
    /// # Arguments
    ///
    /// * `color` - Hexadecimal literal representing a color.
    ///
    /// # Returns
    ///
    /// Color.
    ///
    /// # Note
    ///
    /// If `color` is outside the valid RGB range (e.g. `color > 0xFFFFFF`), then the default color
    /// is returned.
    pub fn hex_literal(color: u32) -> Color {
        // Return the default color if out of range.
        if color > 0xFFFFFF {
            return Color::default();
        }

        // Extract the red, green, and blue components.
        let r = ((color >> 16) & 0xFF) as u8;
        let g = ((color >> 8) & 0xFF) as u8;
        let b = (color & 0xFF) as u8;

        // Construct the color from the RGB components.
        Color::rgb(r, g, b)
    }

    /// Construct a color from a hexadecimal string.
    ///
    /// # Arguments
    ///
    /// * `color` - Hexadecimal string representing a color.
    ///
    /// # Returns
    ///
    /// Color.
    ///
    /// # Note
    ///
    /// If `color` is outside the valid RGB range (e.g. `color > "#FFFFFF"), then the default color
    /// is returned.
    pub fn hex_str(color: &str) -> Color {
        // Remove optional '#' at the beginning
        let color = color.trim_start_matches('#');

        // Ensure the hex string is valid
        if color.len() != 6 {
            return Color::default();
        }

        // Convert the hex string to a hex literal (u32).
        let hex_literal = u32::from_str_radix(color, 16).unwrap_or(0x000000);

        // Construct the color from the hex literal.
        Color::hex_literal(hex_literal)
    }

    /// Set the opacity/transparency of the color.
    ///
    /// # Arguments
    ///
    /// * `a` - Alpha (opacity) between 0 (transparent) and 1 (opaque). Inputs outside the range
    ///   `[0, 1]` are clamped.
    ///
    /// # Returns
    ///
    /// Color with the opacity set.
    pub fn alpha(mut self, a: f64) -> Color {
        self.a = a.clamp(0.0, 1.0);
        self
    }

    /// Convert the color to an [`Rgba`] from the [`plotly`] crate.
    ///
    /// # Returns
    ///
    /// This color as an [`Rgba`] from the [`plotly`] crate.
    pub(crate) fn to_plotly_rgba(&self) -> Rgba {
        Rgba::new(self.r, self.g, self.b, self.a)
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::rgb(0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_partial_eq() {
        // Test equality.
        let color_1 = Color::rgb(20, 30, 40);
        let color_2 = Color::rgb(20, 30, 40);
        assert_eq!(color_1, color_2);

        // Test inequality in r.
        let color_1 = Color::rgb(20, 30, 40);
        let color_2 = Color::rgb(25, 30, 40);
        assert_ne!(color_1, color_2);

        // Test inequality in g.
        let color_1 = Color::rgb(20, 30, 40);
        let color_2 = Color::rgb(20, 35, 40);
        assert_ne!(color_1, color_2);

        // Test inequality in b.
        let color_1 = Color::rgb(20, 30, 40);
        let color_2 = Color::rgb(20, 30, 45);
        assert_ne!(color_1, color_2);

        // Test inequality in a.
        let color_1 = Color::rgb(20, 30, 40).alpha(0.1);
        let color_2 = Color::rgb(20, 30, 40).alpha(0.2);
        assert_ne!(color_1, color_2);
    }

    #[test]
    fn test_rgb() {
        let color = Color::rgb(20, 30, 40);
        assert_eq!(color.r, 20);
        assert_eq!(color.g, 30);
        assert_eq!(color.b, 40);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_rgba() {
        // Simple check.
        let color = Color::rgba(20, 30, 40, 0.5);
        assert_eq!(color.r, 20);
        assert_eq!(color.g, 30);
        assert_eq!(color.b, 40);
        assert_eq!(color.a, 0.5);

        // Check clamping at lower bound.
        let color = Color::rgba(20, 30, 40, -0.1);
        assert_eq!(color.a, 0.0);

        // Check clamping at upper bound.
        let color = Color::rgba(20, 30, 40, 1.1);
        assert_eq!(color.a, 1.0);
    }

    #[test]
    fn test_named() {
        // Simple checks.
        assert_eq!(Color::named(NamedColor::Black), Color::rgb(0, 0, 0));
        assert_eq!(Color::named(NamedColor::White), Color::rgb(255, 255, 255));
        assert_eq!(Color::named(NamedColor::Red), Color::rgb(255, 0, 0));
        assert_eq!(Color::named(NamedColor::Lime), Color::rgb(0, 255, 0));
        assert_eq!(Color::named(NamedColor::Blue), Color::rgb(0, 0, 255));
        assert_eq!(Color::named(NamedColor::Yellow), Color::rgb(255, 255, 0));
        assert_eq!(Color::named(NamedColor::Magenta), Color::rgb(255, 0, 255));
        assert_eq!(Color::named(NamedColor::Cyan), Color::rgb(0, 255, 255));
        assert_eq!(Color::named(NamedColor::Green), Color::rgb(0, 128, 0));

        // Check aliasing.
        assert_eq!(
            Color::named(NamedColor::Aqua),
            Color::named(NamedColor::Cyan)
        );
        assert_eq!(
            Color::named(NamedColor::DarkGrey),
            Color::named(NamedColor::DarkGray)
        );
        assert_eq!(
            Color::named(NamedColor::DarkSlateGrey),
            Color::named(NamedColor::DarkSlateGray)
        );
        assert_eq!(
            Color::named(NamedColor::DimGrey),
            Color::named(NamedColor::DimGray)
        );
        assert_eq!(
            Color::named(NamedColor::Fuchsia),
            Color::named(NamedColor::Magenta)
        );
        assert_eq!(
            Color::named(NamedColor::Grey),
            Color::named(NamedColor::Gray)
        );
        assert_eq!(
            Color::named(NamedColor::LightGrey),
            Color::named(NamedColor::LightGray)
        );
        assert_eq!(
            Color::named(NamedColor::LightSlateGrey),
            Color::named(NamedColor::LightSlateGray)
        );
        assert_eq!(
            Color::named(NamedColor::SlateGrey),
            Color::named(NamedColor::SlateGray)
        );
    }

    #[test]
    fn test_hex_literal() {
        // Simple checks.
        assert_eq!(Color::hex_literal(0x000000), Color::rgb(0, 0, 0));
        assert_eq!(Color::hex_literal(0xFFFFFF), Color::rgb(255, 255, 255));
        assert_eq!(Color::hex_literal(0xFF0000), Color::rgb(255, 0, 0));
        assert_eq!(Color::hex_literal(0x00FF00), Color::rgb(0, 255, 0));
        assert_eq!(Color::hex_literal(0x0000FF), Color::rgb(0, 0, 255));
        assert_eq!(Color::hex_literal(0xFFFF00), Color::rgb(255, 255, 0));
        assert_eq!(Color::hex_literal(0xFF00FF), Color::rgb(255, 0, 255));
        assert_eq!(Color::hex_literal(0x00FFFF), Color::rgb(0, 255, 255));
        assert_eq!(Color::hex_literal(0x008000), Color::rgb(0, 128, 0));

        // Spot check for value outside the RGB range (should return default color).
        assert_eq!(Color::hex_literal(0xFFFFFFFF), Color::default());
    }

    #[test]
    fn test_hex_str() {
        // Checks with #.
        assert_eq!(Color::hex_str("#000000"), Color::rgb(0, 0, 0));
        assert_eq!(Color::hex_str("#FFFFFF"), Color::rgb(255, 255, 255));
        assert_eq!(Color::hex_str("#FF0000"), Color::rgb(255, 0, 0));
        assert_eq!(Color::hex_str("#00FF00"), Color::rgb(0, 255, 0));
        assert_eq!(Color::hex_str("#0000FF"), Color::rgb(0, 0, 255));
        assert_eq!(Color::hex_str("#FFFF00"), Color::rgb(255, 255, 0));
        assert_eq!(Color::hex_str("#FF00FF"), Color::rgb(255, 0, 255));
        assert_eq!(Color::hex_str("#00FFFF"), Color::rgb(0, 255, 255));
        assert_eq!(Color::hex_str("#008000"), Color::rgb(0, 128, 0));

        // Checks without #.
        assert_eq!(Color::hex_str("000000"), Color::rgb(0, 0, 0));
        assert_eq!(Color::hex_str("FFFFFF"), Color::rgb(255, 255, 255));
        assert_eq!(Color::hex_str("FF0000"), Color::rgb(255, 0, 0));
        assert_eq!(Color::hex_str("00FF00"), Color::rgb(0, 255, 0));
        assert_eq!(Color::hex_str("0000FF"), Color::rgb(0, 0, 255));
        assert_eq!(Color::hex_str("FFFF00"), Color::rgb(255, 255, 0));
        assert_eq!(Color::hex_str("FF00FF"), Color::rgb(255, 0, 255));
        assert_eq!(Color::hex_str("00FFFF"), Color::rgb(0, 255, 255));
        assert_eq!(Color::hex_str("008000"), Color::rgb(0, 128, 0));

        // Spot check for value outside the RGB range (should return default color).
        assert_eq!(Color::hex_str("#FFFFFFFF"), Color::default());
    }

    #[test]
    fn test_alpha() {
        // Simple check with valid alpha.
        assert_eq!(
            Color::rgb(20, 30, 40).alpha(0.5),
            Color::rgba(20, 30, 40, 0.5)
        );

        // Check clamping at lower bound.
        assert_eq!(
            Color::rgb(20, 30, 40).alpha(-0.1),
            Color::rgba(20, 30, 40, 0.0)
        );

        // Check clamping at upper bound.
        assert_eq!(
            Color::rgb(20, 30, 40).alpha(1.1),
            Color::rgba(20, 30, 40, 1.0)
        );
    }

    #[test]
    fn to_plotly_rgba() {
        assert_eq!(
            Color::rgba(20, 30, 40, 0.5).to_plotly_rgba(),
            Rgba::new(20, 30, 40, 0.5)
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(Color::default(), Color::rgba(0, 0, 0, 1.0));
    }
}
