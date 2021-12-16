use glam::{Vec3, Vec4};


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Rgba {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    },
}

impl Default for Color {
    fn default() -> Self {
        Self::WHITE
    }
}

impl Color {
    pub const ALICE_BLUE: Color = Color::rgb(0.94, 0.97, 1.0);
    pub const ANTIQUE_WHITE: Color = Color::rgb(0.98, 0.92, 0.84);
    pub const AQUAMARINE: Color = Color::rgb(0.49, 1.0, 0.83);
    pub const AZURE: Color = Color::rgb(0.94, 1.0, 1.0);
    pub const BEIGE: Color = Color::rgb(0.96, 0.96, 0.86);
    pub const BISQUE: Color = Color::rgb(1.0, 0.89, 0.77);
    pub const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
    pub const BLUE: Color = Color::rgb(0.0, 0.0, 1.0);
    pub const CRIMSON: Color = Color::rgb(0.86, 0.08, 0.24);
    pub const CYAN: Color = Color::rgb(0.0, 1.0, 1.0);
    pub const DARK_GRAY: Color = Color::rgb(0.25, 0.25, 0.25);
    pub const DARK_GREEN: Color = Color::rgb(0.0, 0.5, 0.0);
    pub const FUCHSIA: Color = Color::rgb(1.0, 0.0, 1.0);
    pub const GOLD: Color = Color::rgb(1.0, 0.84, 0.0);
    pub const GRAY: Color = Color::rgb(0.5, 0.5, 0.5);
    pub const GREEN: Color = Color::rgb(0.0, 1.0, 0.0);
    pub const INDIGO: Color = Color::rgb(0.29, 0.0, 0.51);
    pub const LIME_GREEN: Color = Color::rgb(0.2, 0.8, 0.2);
    pub const MAROON: Color = Color::rgb(0.5, 0.0, 0.0);
    pub const MIDNIGHT_BLUE: Color = Color::rgb(0.1, 0.1, 0.44);
    pub const NAVY: Color = Color::rgb(0.0, 0.0, 0.5);
    pub const NONE: Color = Color::rgba(0.0, 0.0, 0.0, 0.0);
    pub const OLIVE: Color = Color::rgb(0.5, 0.5, 0.0);
    pub const ORANGE: Color = Color::rgb(1.0, 0.65, 0.0);
    pub const ORANGE_RED: Color = Color::rgb(1.0, 0.27, 0.0);
    pub const PINK: Color = Color::rgb(1.0, 0.08, 0.58);
    pub const PURPLE: Color = Color::rgb(0.5, 0.0, 0.5);
    pub const RED: Color = Color::rgb(1.0, 0.0, 0.0);
    pub const SALMON: Color = Color::rgb(0.98, 0.5, 0.45);
    pub const SEA_GREEN: Color = Color::rgb(0.18, 0.55, 0.34);
    pub const SILVER: Color = Color::rgb(0.75, 0.75, 0.75);
    pub const TEAL: Color = Color::rgb(0.0, 0.5, 0.5);
    pub const TOMATO: Color = Color::rgb(1.0, 0.39, 0.28);
    pub const TURQUOISE: Color = Color::rgb(0.25, 0.88, 0.82);
    pub const VIOLET: Color = Color::rgb(0.93, 0.51, 0.93);
    pub const WHITE: Color = Color::rgb(1.0, 1.0, 1.0);
    pub const YELLOW: Color = Color::rgb(1.0, 1.0, 0.0);
    pub const YELLOW_GREEN: Color = Color::rgb(0.6, 0.8, 0.2);

    pub const fn rgb(r: f32, g: f32, b: f32) -> Color {
        Color::Rgba {
            r,
            g,
            b,
            a: 1.0,
        }
    }

    pub const fn rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color::Rgba {
            r,
            g,
            b,
            a,
        }
    }

    pub fn r(self) -> f32 {
        match self {
            Color::Rgba { r, .. } => r,
        }
    }

    pub fn g(self) -> f32 {
        match self {
            Color::Rgba { g, .. } => g,
        }
    }

    pub fn b(self) -> f32 {
        match self {
            Color::Rgba { b, .. } => b,
        }
    }

    pub fn a(self) -> f32 {
        match self {
            Color::Rgba { a, .. } => a,
        }
    }

    pub fn as_rgba_f32(self: Color) -> [f32; 4] {
        match self {
            Color::Rgba {
                r,
                g,
                b,
                a
            } => [r, g, b, a],
        }
    }

    pub fn hex<T: AsRef<str>>(hex: T) -> Result<Color, HexColorError> {
        let hex = hex.as_ref();

        // RGB
        if hex.len() == 3 {
            let mut data = [0; 6];
            for (i, ch) in hex.chars().enumerate() {
                data[i * 2] = ch as u8;
                data[i * 2 + 1] = ch as u8;
            }
            return decode_rgb(&data);
        }

        // RGBA
        if hex.len() == 4 {
            let mut data = [0; 8];
            for (i, ch) in hex.chars().enumerate() {
                data[i * 2] = ch as u8;
                data[i * 2 + 1] = ch as u8;
            }
            return decode_rgba(&data);
        }

        // RRGGBB
        if hex.len() == 6 {
            return decode_rgb(hex.as_bytes());
        }

        // RRGGBBAA
        if hex.len() == 8 {
            return decode_rgba(hex.as_bytes());
        }

        Err(HexColorError::Length)
    }

    /// Blend this color with the other one, in the RGB color-space. `t` in the range [0..1].
    pub fn interpolate_rgb(&self, other: &Color, t: f32) -> Color {
        Color::Rgba {
            r: self.r() + t * (other.r() - self.r()),
            g: self.g() + t * (other.g() - self.g()),
            b: self.b() + t * (other.b() - self.b()),
            a: self.a() + t * (other.a() - self.a()),
        }
    }
}

impl From<Color> for [f32; 4] {
    fn from(color: Color) -> Self {
        color.as_rgba_f32()
    }
}

impl From<[f32; 4]> for Color {
    fn from([r, g, b, a]: [f32; 4]) -> Self {
        Color::rgba(r, g, b, a)
    }
}

impl From<Color> for Vec4 {
    fn from(color: Color) -> Self {
        let color: [f32; 4] = color.into();
        Vec4::new(color[0], color[1], color[2], color[3])
    }
}

impl From<Vec4> for Color {
    fn from(vec4: Vec4) -> Self {
        Color::rgba(vec4.x, vec4.y, vec4.z, vec4.w)
    }
}

impl From<Color> for [f32; 3] {
    fn from(color: Color) -> Self {
        [color.r(), color.g(), color.b()]
    }
}

impl From<[f32; 3]> for Color {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Color::rgba(r, g, b, 1.0)
    }
}

impl From<Vec3> for Color {
    fn from(vec3: Vec3) -> Self {
        Color::rgba(vec3.x, vec3.y, vec3.z, 1.0)
    }
}

#[derive(Debug)]
pub enum HexColorError {
    Length,
    Hex(hex::FromHexError),
}

fn decode_rgb(data: &[u8]) -> Result<Color, HexColorError> {
    let mut buf = [0; 3];
    match hex::decode_to_slice(data, &mut buf) {
        Ok(_) => {
            let r = buf[0] as f32 / 255.0;
            let g = buf[1] as f32 / 255.0;
            let b = buf[2] as f32 / 255.0;
            Ok(Color::rgb(r, g, b))
        }
        Err(err) => Err(HexColorError::Hex(err)),
    }
}

fn decode_rgba(data: &[u8]) -> Result<Color, HexColorError> {
    let mut buf = [0; 4];
    match hex::decode_to_slice(data, &mut buf) {
        Ok(_) => {
            let r = buf[0] as f32 / 255.0;
            let g = buf[1] as f32 / 255.0;
            let b = buf[2] as f32 / 255.0;
            let a = buf[3] as f32 / 255.0;
            Ok(Color::rgba(r, g, b, a))
        }
        Err(err) => Err(HexColorError::Hex(err)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hex_color() {
        assert_eq!(Color::hex("FFFFFF").unwrap(), Color::rgb(1.0, 1.0, 1.0));
        assert_eq!(Color::hex("000000").unwrap(), Color::rgb(0.0, 0.0, 0.0));
        assert!(Color::hex("------").is_err());

        assert_eq!(
            Color::hex("FFFFFFFF").unwrap(),
            Color::rgba(1.0, 1.0, 1.0, 1.0)
        );
        assert_eq!(
            Color::hex("00000000").unwrap(),
            Color::rgba(0.0, 0.0, 0.0, 0.0)
        );
        assert!(Color::hex("--------").is_err());
        assert!(Color::hex("12345678").is_err());
    }

    #[test]
    fn conversions_vec4() {
        let starting_vec4 = Vec4::new(0.4, 0.5, 0.6, 1.0);
        let starting_color = Color::from(starting_vec4);

        assert_eq!(starting_vec4, Vec4::from(starting_color),);

        let transformation = Vec4::new(0.5, 0.5, 0.5, 1.0);

        assert_eq!(
            starting_color * transformation,
            Color::from(starting_vec4 * transformation),
        );
    }
}