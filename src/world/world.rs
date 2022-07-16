pub struct World {
    pub width: f32,
    pub height: f32,
    pub objects_rendered: Vec<Object::Objects::WorldObject>,
    pub lights_rendered: Vec<Object::Objects::LightObject>,
    pub start_time: wasm_timer::Instant,
} impl World {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width: width,
            height: height,
            objects_rendered: Vec::new(),
            lights_rendered: Vec::new(),
            start_time: wasm_timer::Instant::now(),
        }
    }
}

pub mod Object {

    pub mod Objects {
        use std::time::Duration;

        use crate::world::render_world_layers::{Layer, Pixel};


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
            Rectangle { color: Pixel, },
            Image { rgba: Vec<u8>, width: usize, height: usize, },
        }
        pub struct WorldObject_State {
            pub width: usize,
            pub height: usize,
            pub data: WorldObjectData,
            pub layer: Layer,
        }
        pub struct WorldObject_Fns {
            pub draw_init: Box<dyn Fn(&mut WorldObject_State) -> ()>,
            pub draw_again: Box<dyn Fn(&mut WorldObject_State, &Duration) -> ()>,
        }
        pub struct WorldObject {
            pub state: WorldObject_State,
            pub fns: WorldObject_Fns,
        } impl WorldObject {
            pub fn new_rel(data_and_type: WorldObjectData, pos_x: f32, pos_y: f32, pos_w: f32, pos_h: f32, width: usize, height: usize) -> Self {
                let w = width as f32;
                let h = height as f32;
                Self::new_abs(data_and_type, (pos_x * w).round() as usize, (pos_y * h).round() as usize, (pos_w * w).round() as usize, (pos_h * h).round() as usize, width, height)
            }
            pub fn new_abs(data_and_type: WorldObjectData, pos_x: usize, pos_y: usize, pos_w: usize, pos_h: usize, width: usize, height: usize) -> Self {
                let state = WorldObject_State {
                    width: width,
                    height: height,
                    data: data_and_type,
                    layer: Layer::new(pos_x, pos_y, pos_w, pos_h, width, height),
                };
                match state.data {
                    WorldObjectData::Rectangle {..} => Self {
                        state: state,
                        fns: WorldObject_Fns {
                            draw_init: Box::new(|state: &mut WorldObject_State| {
                                if let WorldObjectData::Rectangle { color, } = &mut state.data {
                                    let layer = &mut state.layer;
                                    for y in 0..layer.pos_h {
                                        let line = &mut layer.pixel_data[y];
                                        for x in 0..layer.pos_w {
                                            line[x] = color.clone();
                                        }
                                    }
                                }
                            }),
                            draw_again: Box::new(|state: &mut WorldObject_State, duration: &Duration| {
                            }),
                        }
                    },
                    WorldObjectData::Image {..} => Self {
                        state: state,
                        fns: WorldObject_Fns {
                            draw_init: Box::new(|state: &mut WorldObject_State| {
                                if let WorldObjectData::Image { rgba, width, height } = &mut state.data {
                                    for y in 0..state.layer.pos_h {
                                        let img_index_line = (y * *height / state.layer.pos_h) * *width;
                                        for x in 0..state.layer.pos_w {
                                            let img_index = (img_index_line + x * *width / state.layer.pos_w) * 4;
                                            state.layer.pixel_data[y][x] = Pixel::Opaque { r: rgba[img_index], g: rgba[img_index+1], b: rgba[img_index+2], };
                                        }
                                    }
                                }
                            }),
                            draw_again:  Box::new(|state: &mut WorldObject_State, duration: &Duration| {
                                state.layer.pos_x = state.layer.pos_x_start + (duration.as_millis() / 10 % 100) as usize;
                            }),
                        }
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