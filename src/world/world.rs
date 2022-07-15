pub struct World {
    pub renderable: WorldRenderable,
} impl World {
    pub fn new(renderable: WorldRenderable) -> Self {
        Self {
            renderable: renderable,
        }
    }
}

pub struct WorldRenderable {
    pub width: f32,
    pub height: f32,
    pub objects_rendered: Vec<Box<Object::Objects::WorldObject>>,
    pub lights_rendered: Vec<Object::Objects::LightObject>,
} impl WorldRenderable {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width: width,
            height: height,
            objects_rendered: Vec::new(),
            lights_rendered: Vec::new(),
        }
    }
}

pub mod Object {

    pub mod Objects {

        pub struct LightObject {
            pub x: f32,
            pub y: f32,
            pub brightness: (u16, u16, u16),
            pub size: f32,
            pub range: f32,
        } impl LightObject {
            pub fn new(x: f32, y: f32, brightness: (u16, u16, u16), size: f32, range: f32) -> Self {
                Self {
                    x: x,
                    y: y,
                    brightness: brightness,
                    size: size,
                    range: range,
                }
            }
        }

        pub enum WorldObjectData {
            Rectangle((f32, f32, f32, f32, u8, u8, u8)),
            Image((Vec<u8>, u32, u32, f32, f32, f32, f32)),
        }
        pub struct WorldObject {
            pub data: WorldObjectData,
            //pub contains_coordinate_maybe: Box<dyn Fn() -> bool>,
            //pub contains_coordinate_definitely: Box<dyn Fn() -> bool>,

            //                          ( self ,  w ,  h ,  w ,  h , data as rgba )
            pub draw_init: Box<dyn Fn(&Self, u32, u32, f32, f32, &mut Vec<u8>) -> ()>,
            pub draw_required: bool,
            pub draw_again: Box<dyn Fn(&Self, u32, u32, f32, f32, &mut Vec<u8>) -> ()>,
        } impl WorldObject {
            pub fn new(data_and_type: WorldObjectData) -> Self {
                match data_and_type {
                    WorldObjectData::Rectangle(_) => Self {
                        data: data_and_type,
                        draw_init: Box::new(|s: &Self, widthpx: u32, heightpx: u32, widthrel: f32, heightrel: f32, data: &mut Vec<u8>| {
                            if let WorldObjectData::Rectangle((x, y, w, h, r, g, b)) = s.data {
                                let widthf = widthpx as f32 / widthrel; // factors to convert from relative positions to pixel coordinates
                                let heightf = heightpx as f32 / heightrel;

                                let xi = (x * widthf) as usize; // rect pos in pixels
                                let yi = (y * heightf) as usize;
                                let wi = (w * widthf) as usize; // rect size in pixels
                                let hi = (h * heightf) as usize;

                                // draw
                                for y in yi..yi+hi {
                                    let mut index = 4 /* bit depth */ * (widthpx as usize * y + xi); // go down (to y~dyn) and to the right (to xi~const), ending up at the index of the first pixel in this row.
                                    for _/*x*/ in 0..wi {
                                        data[index] = r;
                                        index += 1;
                                        data[index] = g;
                                        index += 1;
                                        data[index] = b;
                                        index += 1;
                                        data[index] = 255;
                                        index += 1;
                                    }
                                }
                            }
                        }),
                        draw_required: false,
                        draw_again: Box::new(|s: &Self, widthpx: u32, heightpx: u32, widthrel: f32, heightrel: f32, data: &mut Vec<u8>| {
                        }),
                    },
                    WorldObjectData::Image(_) => Self {
                        data: data_and_type,
                        draw_init:  Box::new(|s: &Self, widthpx: u32, heightpx: u32, widthrel: f32, heightrel: f32, data: &mut Vec<u8>| {
                            if let WorldObjectData::Image((bytes, widthimg, heightimg, x_rel, y_rel, w_rel, h_rel)) = &s.data {
                                let widthimg = widthimg.to_owned() as usize;
                                let heightimg = heightimg.to_owned() as usize;
                                let widthimgf = widthimg as f32;
                                let heightimgf = heightimg as f32;
                                let widthpxs = widthpx as usize;
                                let heightpxsf = heightpx as usize;
                                let widthpxf = widthpx as f32;
                                let heightpxf = heightpx as f32;

                                // the pixel position and size of the image once it is drawn to the map
                                let x_abs = (x_rel * widthpxf / widthrel) as usize;
                                let y_abs = (y_rel * heightpxf / heightrel) as usize;
                                let w_abs = (w_rel * widthpxf / widthrel) as usize;
                                let h_abs = (h_rel * heightpxf / heightrel) as usize;

                                let byte_width_entire_row = widthpxs * 4;

                                for y_rel_world in 0..h_abs {
                                    let y_abs_world = y_abs + y_rel_world;
                                    let y_abs_img = y_rel_world * heightimg / h_abs;
                                    for x_rel_world in 0..w_abs {
                                        let x_abs_world = x_abs + x_rel_world;
                                        let x_abs_img = x_rel_world * widthimg / w_abs;
                                        let mut bytepos_world = y_abs_world * byte_width_entire_row + x_abs_world * 4;
                                        let mut bytepos_img = (y_abs_img * widthimg + x_abs_img) * 4;
                                        data[bytepos_world] = bytes[bytepos_img];
                                        bytepos_world += 1; bytepos_img += 1;
                                        data[bytepos_world] = bytes[bytepos_img];
                                        bytepos_world += 1; bytepos_img += 1;
                                        data[bytepos_world] = bytes[bytepos_img];
                                        bytepos_world += 1; bytepos_img += 1;
                                        data[bytepos_world] = bytes[bytepos_img];
                                    }
                                }
                            }
                        }),
                        draw_required: false,
                        draw_again:  Box::new(|s: &Self, widthpx: u32, heightpx: u32, widthrel: f32, heightrel: f32, data: &mut Vec<u8>| {}),
                    },
                }
            }
        }

    }

    pub mod Other {

        pub struct Color {
            r: u8,
            g: u8,
            b: u8,
            a: u8,
        }

    }

}