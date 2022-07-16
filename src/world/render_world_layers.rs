pub struct Layer {
    /// the original x-position of this layer on the screen in pixels
    pub pos_x_start: usize,
    /// the original y-position of this layer on the screen in pixels
    pub pos_y_start: usize,
    /// the current x-position of this layer on the screen in pixels
    pub pos_x: usize,
    /// the current y-position of this layer on the screen in pixels
    pub pos_y: usize,
    /// the width of this layer on the screen in pixels
    pub pos_w: usize,
    /// the height of this layer on the screen in pixels
    pub pos_h: usize,
    /// the width of the entire screen in pixels
    pub width: usize,
    /// the height of the entire screen in pixels
    pub height: usize,
    pub pixel_data: Vec<Vec<Pixel>>,
}
impl Layer {
    pub fn new(x: usize, y: usize, w: usize, h: usize, width: usize, height: usize) -> Self {
        Self {
            pos_x_start: x,
            pos_y_start: y,
            pos_x: x,
            pos_y: y,
            pos_w: w,
            pos_h: h,
            width: width,
            height: height,
            pixel_data: { // generate 2d vec of transparent pixels //
                let mut v = vec![];
                for _ in 0..height {
                    let mut v2 = vec![];
                    for _ in 0..width {
                        v2.push(Pixel::Transparent);
                    }
                    v.push(v2);
                }
                v
            },
        }
    }
    pub fn draw_onto(&self, image_bytes: &mut Vec<(u8, u8, u8)>, width: usize, height: usize) {
        let width_left = self.pos_x;
        let width_line = width;
        let mut line_start_index = self.pos_y * width_line;
        for line in 0..self.pos_h {
            let mut index = line_start_index + width_left;
            for pixel in self.pixel_data[line].iter() {
                match *pixel {
                    Pixel::Transparent => {
                    },
                    Pixel::Opaque { r, g, b } => {
                        image_bytes[index] = (r, g, b);
                    },
                    Pixel::SemiTransparent { r, g, b, a } => {
                        let na = 1.0 - a;
                        let old = image_bytes[index];
                        image_bytes[index] = ((na * old.0 as f32 + a * r) as u8, (na * old.1 as f32 + a * g) as u8, (na * old.2 as f32 + a * b) as u8);
                    },
                }
                index += 1;
            }
            line_start_index += width_line;
        }
    }
}

pub enum Pixel {
    Transparent,
    Opaque { r: u8, g: u8, b: u8 },
    /// rgb values range from 0 to 255, alpha from 0 t 1.
    SemiTransparent { r: f32, g: f32, b: f32, a: f32 },
}
impl Copy for Pixel {
}
impl Clone for Pixel {
    fn clone(&self) -> Self {
        match self {
            Self::Transparent => Self::Transparent,
            Self::Opaque { r, g, b } => Self::Opaque { r: r.clone(), g: g.clone(), b: b.clone() },
            Self::SemiTransparent { r, g, b, a } => Self::SemiTransparent { r: r.clone(), g: g.clone(), b: b.clone(), a: a.clone() },
        }
    }
}