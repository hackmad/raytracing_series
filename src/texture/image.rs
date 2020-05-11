//! # Image
//!
//! A library for handling image textures.

#![allow(dead_code)]
use super::{clamp, Colour, Float, Point3, RcTexture, Texture};
use image::{Rgb, RgbImage};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

/// Models an image texture
#[derive(Clone)]
pub struct Image {
    /// Width.
    width: u32,

    /// Height.
    height: u32,

    /// The image
    img: Rc<RefCell<RgbImage>>,
}

impl Image {
    /// Creates a new image texture.
    ///
    /// * `t0` - Provides first colour for the imageboard pattern.
    /// * `t1` - Provides second colour for the imageboard pattern.
    pub fn new(path: &str) -> RcTexture {
        // Read image and convert to RGB.
        let img = image::open(path)
            .expect(format!("Unable to open {}", path).as_ref())
            .into_rgb();

        // Read metadata before we wrap it in a Rc<RefCell<RgbImage>> to
        // avoid borrowing it.
        let width = img.width();
        let height = img.height();

        let img = Rc::new(RefCell::new(img));

        Rc::new(Image { img, width, height })
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Image")
            .field("width", &self.width)
            .field("height", &self.height)
            .finish()
    }
}

const COLOUR_SCALE: Float = 1.0 / 255.0;

impl Texture for Image {
    /// Return the stored colour value regardless of texture coordinates
    /// and intersection point.
    ///
    /// * `u` - Paramteric coordinate.
    /// * `v` - Paramteric coordinate.
    /// * `_p` - Intersection point (not used).
    fn value(&self, u: Float, v: Float, _p: &Point3) -> Colour {
        // Clamp input texture coordinates to [0,1] x [1,0]
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0); // Flip V to image coordinates

        let mut i = (u * self.width as Float) as u32;
        let mut j = (v * self.height as Float) as u32;

        // Clamp integer mapping, since actual coordinates should be less
        // than 1.0.
        if i >= self.width {
            i = self.width - 1;
        }

        if j >= self.height {
            j = self.height - 1;
        }

        let img = self.img.borrow();
        let Rgb(p) = img.get_pixel(i, j);

        Colour::new(p[0] as Float, p[1] as Float, p[2] as Float) * COLOUR_SCALE
    }
}
