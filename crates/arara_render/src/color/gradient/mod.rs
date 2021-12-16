//! # Overview
//!
//! Rust color scales library for data visualization, charts, games, maps, generative art and others.
//!
//! ## Usage
//!
//! Using preset gradient:
//! ```
//! let g = colorgrad::rainbow();
//!
//! assert_eq!(g.domain(), (0.0, 1.0)); // all preset gradients are in the domain [0..1]
//! assert_eq!(g.at(0.5).rgba_u8(), (175, 240, 91, 255));
//! assert_eq!(g.at(0.5).to_hex_string(), "#aff05b");
//! ```
//!
//! Custom gradient:
//! ```
//! use colorgrad::Color;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let g = colorgrad::CustomGradient::new()
//!     .colors(&[
//!         Color::from_rgb_u8(255, 0, 0),
//!         Color::from_rgb_u8(0, 255, 0),
//!     ])
//!     .build()?;
//!
//! assert_eq!(g.at(0.0).rgba_u8(), (255, 0, 0, 255));
//! assert_eq!(g.at(0.0).to_hex_string(), "#ff0000");
//! assert_eq!(g.at(1.0).to_hex_string(), "#00ff00");
//! # Ok(())
//! # }
//! ```
//!
//! ## Examples
//!
//! ### Gradient Image
//!
//! ```rust
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let grad = colorgrad::CustomGradient::new()
//!         .html_colors(&["deeppink", "gold", "seagreen"])
//!         .build()?;
//!
//!     let width = 1500;
//!     let height = 70;
//!
//!     let mut imgbuf = image::ImageBuffer::new(width, height);
//!
//!     for (x, _, pixel) in imgbuf.enumerate_pixels_mut() {
//!         let (r, g, b, a) = grad.at(x as f32 / width as f32).rgba_u8();
//!         *pixel = image::Rgba([r, g, b, a]);
//!     }
//!
//!     imgbuf.save("gradient.png")?;
//!     Ok(())
//! }
//! ```
//!
//! Example output:
//!
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/example-gradient.png)
//!
//! ### Colored Noise
//!
//! ```rust,ignore
//! use noise::NoiseFn;
//!
//! fn main() {
//!     let scale = 0.015;
//!
//!     let grad = colorgrad::rainbow().sharp(5, 0.15);
//!     let ns = noise::OpenSimplex::new();
//!     let mut imgbuf = image::ImageBuffer::new(600, 350);
//!
//!     for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
//!         let t = ns.get([x as f32 * scale, y as f32 * scale]);
//!         let (r, g, b, a) = grad.at(remap(t, -0.5, 0.5, 0.0, 1.0)).rgba_u8();
//!         *pixel = image::Rgba([r, g, b, a]);
//!     }
//!
//!     imgbuf.save("noise.png").unwrap();
//! }
//!
//! // Map t which is in range [a, b] to range [c, d]
//! fn remap(t: f32, a: f32, b: f32, c: f32, d: f32) -> f32 {
//!     (t - a) * ((d - c) / (b - a)) + c
//! }
//! ```
//!
//! Example output:
//!
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/example-noise.png)
//!
//! ## Preset Gradients
//!
//! [colorgrad::cubehelix_default()](fn.cubehelix_default.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/cubehelix_default.png)
//!
//! [colorgrad::turbo()](fn.turbo.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/turbo.png)
//!
//! [colorgrad::spectral()](fn.spectral.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/spectral.png)
//!
//! [colorgrad::viridis()](fn.viridis.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/viridis.png)
//!
//! [colorgrad::magma()](fn.magma.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/magma.png)
//!
//! [colorgrad::rainbow()](fn.rainbow.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/rainbow.png)
//!
//! [colorgrad::sinebow()](fn.sinebow.html)
//! ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/sinebow.png)
//!
//! See more complete gradient preview and examples at [Github](https://github.com/mazznoer/colorgrad-rs).

#![allow(clippy::many_single_char_names)]

mod spline;
mod presets;

use std::{error, fmt};

pub use presets::*;
use spline::*;
use crate::Color;

/// Color blending mode
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BlendMode {
    Rgb,
    LinearRgb,
    Hsv,
    Oklab,
}

/// Interpolation mode
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum Interpolation {
    Linear,
    Basis,
    CatmullRom,
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum CustomGradientError {
    InvalidHtmlColor(Vec<String>),
    WrongDomainCount,
    WrongDomain,
}

impl fmt::Display for CustomGradientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CustomGradientError::InvalidHtmlColor(ref colors) => {
                write!(
                    f,
                    "Invalid html colors:{}",
                    colors
                        .iter()
                        .map(|x| format!("'{}'", x))
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            CustomGradientError::WrongDomainCount => f.write_str("Wrong domain count"),
            CustomGradientError::WrongDomain => f.write_str("Wrong domain"),
        }
    }
}

impl error::Error for CustomGradientError {}

pub trait GradientBase {
    fn at(&self, t: f32) -> Color;
}

/// The gradient
pub struct Gradient {
    pub gradient: Box<dyn GradientBase + Send + Sync>,
    pub dmin: f32,
    pub dmax: f32,
}

impl fmt::Debug for Gradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Gradient")
            .field("dmin", &self.dmin)
            .field("dmax", &self.dmax)
            .finish()
    }
}

impl Gradient {
    /// Get color at certain position
    pub fn at(&self, t: f32) -> Color {
        self.gradient.at(t)
    }

    /// Get color at certain position
    pub fn repeat_at(&self, t: f32) -> Color {
        let t = norm(t, self.dmin, self.dmax);
        self.gradient
            .at(self.dmin + modulo(t, 1.0) * (self.dmax - self.dmin))
    }

    /// Get color at certain position
    pub fn reflect_at(&self, t: f32) -> Color {
        let t = norm(t, self.dmin, self.dmax);
        self.gradient
            .at(self.dmin + (modulo(1.0 + t, 2.0) - 1.0).abs() * (self.dmax - self.dmin))
    }

    /// Get n colors evenly spaced across gradient
    pub fn colors(&self, n: usize) -> Vec<Color> {
        linspace(self.dmin, self.dmax, n)
            .iter()
            .map(|&t| self.gradient.at(t))
            .collect()
    }

    /// Get the gradient's domain min and max
    pub fn domain(&self) -> (f32, f32) {
        (self.dmin, self.dmax)
    }

    /// Get new hard-edge gradient
    ///
    /// ```
    /// let g = colorgrad::rainbow();
    /// ```
    /// ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/preset/rainbow.png)
    ///
    /// ```
    /// let g = colorgrad::rainbow().sharp(11, 0.);
    /// ```
    /// ![img](https://raw.githubusercontent.com/mazznoer/colorgrad-rs/master/docs/images/rainbow-sharp.png)
    pub fn sharp(&self, segment: usize, smoothness: f32) -> Gradient {
        if segment < 2 {
            let gradbase = SharpGradient {
                colors: vec![self.gradient.at(self.dmin)],
                pos: vec![self.dmin, self.dmax],
                n: 0,
                dmin: self.dmin,
                dmax: self.dmax,
            };

            return Gradient {
                gradient: Box::new(gradbase),
                dmin: self.dmin,
                dmax: self.dmax,
            };
        }

        if smoothness > 0.0 {
            return sharp_gradient_x(self, segment, smoothness);
        }

        sharp_gradient(self, segment)
    }
}

#[derive(Debug)]
struct LinearGradient {
    colors: Vec<Color>,
    pos: Vec<f32>,
    count: usize,
    dmin: f32,
    dmax: f32,
    mode: BlendMode,
}

impl GradientBase for LinearGradient {
    fn at(&self, t: f32) -> Color {
        if t < self.dmin {
            return self.colors[0].clone();
        }

        if t > self.dmax {
            return self.colors[self.count].clone();
        }

        for (pos, col) in self.pos.windows(2).zip(self.colors.windows(2)) {
            if (pos[0] <= t) && (t <= pos[1]) {
                let t = (t - pos[0]) / (pos[1] - pos[0]);

                match self.mode {
                    BlendMode::Rgb => return col[0].interpolate_rgb(&col[1], t),
                    // BlendMode::LinearRgb => return col[0].interpolate_linear_rgb(&col[1], t),
                    // BlendMode::Hsv => return col[0].interpolate_hsv(&col[1], t),
                    // BlendMode::Oklab => return col[0].interpolate_oklab(&col[1], t),
                    _ => todo!()
                }
            }
        }

        self.colors[0].clone()
    }
}

#[derive(Debug)]
struct SharpGradient {
    colors: Vec<Color>,
    pos: Vec<f32>,
    n: usize,
    dmin: f32,
    dmax: f32,
}

impl GradientBase for SharpGradient {
    fn at(&self, t: f32) -> Color {
        if t < self.dmin {
            return self.colors[0].clone();
        }

        if t > self.dmax {
            return self.colors[self.n].clone();
        }

        for (pos, col) in self.pos.windows(2).zip(self.colors.iter()) {
            if (pos[0] <= t) && (t <= pos[1]) {
                return col.clone();
            }
        }

        self.colors[0].clone()
    }
}

fn sharp_gradient(grad: &Gradient, n: usize) -> Gradient {
    let (dmin, dmax) = grad.domain();

    let gradbase = SharpGradient {
        colors: grad.colors(n),
        pos: linspace(dmin, dmax, n + 1),
        n: n - 1,
        dmin,
        dmax,
    };

    Gradient {
        gradient: Box::new(gradbase),
        dmin,
        dmax,
    }
}

#[derive(Debug)]
struct SharpGradientX {
    colors: Vec<Color>,
    pos: Vec<f32>,
    dmin: f32,
    dmax: f32,
    last_idx: usize,
}

impl GradientBase for SharpGradientX {
    fn at(&self, t: f32) -> Color {
        if t < self.dmin {
            return self.colors[0].clone();
        }

        if t > self.dmax {
            return self.colors[self.last_idx].clone();
        }

        for (i, (pos, col)) in self.pos.windows(2).zip(self.colors.windows(2)).enumerate() {
            if (pos[0] <= t) && (t <= pos[1]) {
                if i & 1 == 0 {
                    return col[0].clone();
                }

                let t = (t - pos[0]) / (pos[1] - pos[0]);
                return col[0].interpolate_rgb(&col[1], t);
            }
        }

        self.colors[0].clone()
    }
}

fn sharp_gradient_x(grad: &Gradient, n: usize, t: f32) -> Gradient {
    let mut colors = Vec::with_capacity(n * 2);

    for c in grad.colors(n) {
        colors.push(c.clone());
        colors.push(c.clone());
    }

    let (dmin, dmax) = grad.domain();
    let t = t.clamp(0.0, 1.0) * (dmax - dmin) / n as f32 / 4.0;
    let p = linspace(dmin, dmax, n + 1);
    let mut pos = Vec::with_capacity(n * 2);
    let mut j = 0;

    for i in 0..n {
        pos.push(p[i]);

        if j > 0 {
            pos[j] += t;
        }

        j += 1;
        pos.push(p[i + 1]);

        if j < colors.len() - 1 {
            pos[j] -= t;
        }

        j += 1;
    }

    let gradbase = SharpGradientX {
        colors,
        pos,
        last_idx: n * 2 - 1,
        dmin,
        dmax,
    };

    Gradient {
        gradient: Box::new(gradbase),
        dmin,
        dmax,
    }
}

/// Create custom gradient
///
/// # Examples
///
/// ```
/// # use std::error::Error;
/// use colorgrad::Color;
///
/// # fn main() -> Result<(), Box<dyn Error>> {
/// let grad = colorgrad::CustomGradient::new()
///     .colors(&[
///         Color::rgb(1.0, 0, 0),
///         Color::rgb(0.0, 0.0, 1.0),
///     ])
///     .build()?;
///
/// assert_eq!(grad.domain(), (0.0, 1.0)); // default domain
/// assert_eq!(grad.at(0.0).rgba_u8(), (255, 0, 0, 255));
/// assert_eq!(grad.at(1.0).rgba_u8(), (0, 0, 255, 255));
/// # Ok(())
/// # }
/// ```
#[derive(Debug)]
pub struct CustomGradient {
    colors: Vec<Color>,
    pos: Vec<f32>,
    mode: BlendMode,
    interpolation: Interpolation,
    invalid_html_colors: Vec<String>,
}

impl CustomGradient {
    #[allow(clippy::new_without_default)]
    pub fn new() -> CustomGradient {
        CustomGradient {
            colors: Vec::new(),
            pos: Vec::new(),
            mode: BlendMode::Rgb,
            interpolation: Interpolation::Linear,
            invalid_html_colors: Vec::new(),
        }
    }

    /// Set gradient color
    pub fn colors<'a>(&'a mut self, colors: &[Color]) -> &'a mut CustomGradient {
        for c in colors {
            self.colors.push(c.clone());
        }
        self
    }

    /// Set the gradient domain and/or color position.
    pub fn domain<'a>(&'a mut self, pos: &[f32]) -> &'a mut CustomGradient {
        self.pos = pos.to_vec();
        self
    }

    /// Set the color blending mode
    #[allow(clippy::needless_lifetimes)]
    pub fn mode<'a>(&'a mut self, mode: BlendMode) -> &'a mut CustomGradient {
        self.mode = mode;
        self
    }

    /// Set the interpolation mode
    #[allow(clippy::needless_lifetimes)]
    pub fn interpolation<'a>(&'a mut self, mode: Interpolation) -> &'a mut CustomGradient {
        self.interpolation = mode;
        self
    }

    /// Build the gradient
    pub fn build(&self) -> Result<Gradient, CustomGradientError> {
        if !self.invalid_html_colors.is_empty() {
            return Err(CustomGradientError::InvalidHtmlColor(
                self.invalid_html_colors.clone(),
            ));
        }

        let colors = if self.colors.is_empty() {
            vec![
                Color::rgb(0.0, 0.0, 0.0),
                Color::rgb(1.0, 1.0, 1.0),
            ]
        } else if self.colors.len() == 1 {
            vec![self.colors[0].clone(), self.colors[0].clone()]
        } else {
            self.colors.to_vec()
        };

        let pos = if self.pos.is_empty() {
            linspace(0.0, 1.0, colors.len())
        } else if self.pos.len() == colors.len() {
            for p in self.pos.windows(2) {
                if p[0] > p[1] {
                    return Err(CustomGradientError::WrongDomain);
                }
            }
            self.pos.to_vec()
        } else if self.pos.len() == 2 {
            if self.pos[0] >= self.pos[1] {
                return Err(CustomGradientError::WrongDomain);
            }
            linspace(self.pos[0], self.pos[1], colors.len())
        } else {
            return Err(CustomGradientError::WrongDomainCount);
        };

        if let Interpolation::Linear = self.interpolation {
            let gradbase = LinearGradient {
                colors: colors.to_vec(),
                pos: pos.to_vec(),
                count: colors.len() - 1,
                dmin: pos[0],
                dmax: pos[pos.len() - 1],
                mode: self.mode,
            };

            return Ok(Gradient {
                gradient: Box::new(gradbase),
                dmin: pos[0],
                dmax: pos[pos.len() - 1],
            });
        }

        Ok(spline_gradient(
            &colors,
            &pos,
            self.mode,
            self.interpolation,
        ))
    }
}

// ---

pub fn linspace(min: f32, max: f32, n: usize) -> Vec<f32> {
    if n == 1 {
        return vec![min];
    }

    let d = max - min;
    let l = n as f32 - 1.0;
    (0..n).map(|i| min + (i as f32 * d) / l).collect()
}

#[inline]
fn modulo(x: f32, y: f32) -> f32 {
    (x % y + y) % y
}

#[inline]
// Map t from range [a, b] to range [0, 1]
fn norm(t: f32, a: f32, b: f32) -> f32 {
    (t - a) * (1.0 / (b - a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linspace() {
        assert_eq!(linspace(0.0, 1.0, 0), vec![]);
        assert_eq!(linspace(0.0, 1.0, 1), vec![0.0]);
        assert_eq!(linspace(0.0, 1.0, 2), vec![0.0, 1.0]);
        assert_eq!(linspace(0.0, 1.0, 3), vec![0.0, 0.5, 1.0]);
        assert_eq!(linspace(-1.0, 1.0, 5), vec![-1.0, -0.5, 0.0, 0.5, 1.0]);
        assert_eq!(linspace(0.0, 100.0, 5), vec![0.0, 25.0, 50.0, 75.0, 100.0]);
    }
}