use std::time::Duration;

use super::world::World;



pub struct WorldRenderer {
    pub world: super::world::World,
    pub width: usize,
    pub height: usize,
    pub lights_renderer: LightMap,
    pub objects_renderer: ObjectNoLightRenderer,
} impl WorldRenderer {
    pub fn new(world: super::world::World, width: usize, height: usize) -> Self {
        Self {
            world: world,
            width: width,
            height: height,
            lights_renderer: LightMap::new(width, height, 2),
            objects_renderer: ObjectNoLightRenderer::new(width, height),
        }
    }

    pub fn init(&mut self) {
        self.objects_renderer.draw_init(&mut self.world);
    }

    pub fn render(&mut self, image_data: &mut Vec<u8>) -> [Duration; 3] {
        // draw objects to Vec<Layer>
        let start_time = wasm_timer::Instant::now();
        self.objects_renderer.draw_all(&mut self.world);
        let elapsed_time_objects = start_time.elapsed();
        
        // draw light/brightness to Vec<(u16, u16, u16)>
        let start_time = wasm_timer::Instant::now();
        self.lights_renderer.calculate(&self.world, self.width, self.height);
        let elapsed_time_brightness = start_time.elapsed();

        let start_time = wasm_timer::Instant::now();
        super::render_world::render_joiner::join(image_data, self);
        let elapsed_time_join = start_time.elapsed();

        [elapsed_time_brightness, elapsed_time_objects, elapsed_time_join]

    }
}


pub struct LightMap {
    width: usize,
    height: usize,
    inaccuracy: usize,
    data: Vec<(u16, u16, u16)>, // brightness in (rgb) format
} impl LightMap {
    pub fn new(w: usize, h: usize, inaccuracy: usize) -> Self {
        let w = w / inaccuracy;
        let h = h / inaccuracy;
        let len = w * h;
        Self {
            width: w,
            height: h,
            inaccuracy: inaccuracy,
            data: vec![(0, 0, 0); len]
        }
    }
    pub fn calculate(&mut self, world: &super::world::World, width: usize, height: usize) {
        {

            let mut index = 0;
            for y in 0..self.height {
                let Y = ((y * 2) as f32 / (self.height - 1) as f32 - 1f32) * world.height; // convert the value from pixels to a relative value from -1 to 1
                for x in 0..self.width {
                    let X = ((x * 2) as f32 / (self.width - 1) as f32 - 1f32) * world.width; // convert the value from pixels to a relative value from -1 to 1
                    self.data[index] = {
                        let mut r: u16 = 0;
                        let mut g: u16 = 0;
                        let mut b: u16 = 0;
                        for light_source in &world.lights_rendered {
                            let dist_x = light_source.x - X;
                            let dist_y = light_source.y - Y;

                            let light_source_max_reach = light_source.size + light_source.range;
                            let light_source_max_reach_squared = light_source_max_reach * light_source_max_reach;

                            if dist_x > light_source_max_reach || dist_y > light_source_max_reach { continue; }

                            let dist_squared = dist_x * dist_x + dist_y * dist_y; // don't use sqrt because it's rather slow

                            if dist_squared >= light_source_max_reach_squared { continue; }

                            let light_source_size_squared = light_source.size * light_source.size;

                            if light_source_size_squared >= dist_squared {
                                r = r.saturating_add(light_source.brightness.0);
                                g = g.saturating_add(light_source.brightness.1);
                                b = b.saturating_add(light_source.brightness.2);
                            } else {
                                let factor_at_size = light_source_size_squared / light_source_max_reach_squared;
                                let factor = dist_squared / light_source_max_reach_squared; // the sqrt of this factor is the actual factor. The factor is always less than 1.
                                let factor = (1.0 - factor) /* the width of the outer ring */ / (1.0 - factor_at_size) /* the maximum size of the outer ring (i.e. the value that 1-f1 will have at its insidemost point) */;
                                let factor = factor * factor; // this just makes it look a bit nicer, there should be almost no performance impact
                                let factor_int = (factor * u16::MAX as f32) as u32;
                                r = r.saturating_add(((light_source.brightness.0 as u32 * factor_int) >> 16) as u16);
                                g = g.saturating_add(((light_source.brightness.1 as u32 * factor_int) >> 16) as u16);
                                b = b.saturating_add(((light_source.brightness.2 as u32 * factor_int) >> 16) as u16);
                            };
                        };
                        (r, g, b)
                    };
                    self.data[index];
                    index += 1;
                }
            }
        }
    }
    // pub fn calculate_and_join(&mut self, world: &super::world::WorldRenderable, image_data: &mut Vec<u8>, width: u32, height: u32, objects: &ObjectNoLightRenderer) {
    //     {
    //         let image_data_byte_width_without_modified_pixels = 4 * (width - self.inaccuracy) ;
    //         let image_data_byte_count_row = 4 * width ;
    //         let image_data_byte_count_inaccuracy_rows = 4 * (width * self.inaccuracy) ;
    //         let image_data_byte_count_inaccuracy_pixels = 4 * self.inaccuracy ;

    //         let mut thing: &mut [u8] = image_data.as_mut_slice();

    //         let mut index = 0;
    //         for y in 0..self.height {
    //             // BRIGHTNESS

    //             // BRIGHTNESS
    //             // JOIN

    //             let mut image_data_byte_pos = 4 * (width * y) ;

    //             // JOIN
    //             let Y = ((y * 2) as f32 / (self.height - 1) as f32 - 1f32) * world.height; // convert the value from pixels to a relative value from -1 to 1

    //             for x in 0..self.width {
    //                 // BRIGHTNESS

    //                 let X = ((x * 2) as f32 / (self.width - 1) as f32 - 1f32) * world.width; // convert the value from pixels to a relative value from -1 to 1
    //                 let this_light = {
    //                     let mut r: u16 = 0;
    //                     let mut g: u16 = 0;
    //                     let mut b: u16 = 0;
    //                     for light_source in &world.lights_rendered {
    //                         let dist_x = light_source.x - X;
    //                         let dist_y = light_source.y - Y;

    //                         let light_source_max_reach = light_source.size + light_source.range;
    //                         let light_source_max_reach_squared = light_source_max_reach * light_source_max_reach;

    //                         if dist_x > light_source_max_reach || dist_y > light_source_max_reach { continue; }

    //                         let dist_squared = dist_x * dist_x + dist_y * dist_y; // don't use sqrt because it's rather slow

    //                         if dist_squared >= light_source_max_reach_squared { continue; }

    //                         let light_source_size_squared = light_source.size * light_source.size;

    //                         if light_source_size_squared >= dist_squared {
    //                             r = r.saturating_add(light_source.brightness.0);
    //                             g = g.saturating_add(light_source.brightness.1);
    //                             b = b.saturating_add(light_source.brightness.2);
    //                         } else {
    //                             let factor_at_size = light_source_size_squared / light_source_max_reach_squared;
    //                             let factor = dist_squared / light_source_max_reach_squared; // the sqrt of this factor is the actual factor. The factor is always less than 1.
    //                             let factor = (1.0 - factor) /* the width of the outer ring */ / (1.0 - factor_at_size) /* the maximum size of the outer ring (i.e. the value that 1-f1 will have at its insidemost point) */;
    //                             let factor = factor * factor; // this just makes it look a bit nicer, there should be almost no performance impact
    //                             let factor_int = (factor * u16::MAX as f32) as u32;
    //                             r = r.saturating_add(((light_source.brightness.0 as u32 * factor_int) >> 16) as u16);
    //                             g = g.saturating_add(((light_source.brightness.1 as u32 * factor_int) >> 16) as u16);
    //                             b = b.saturating_add(((light_source.brightness.2 as u32 * factor_int) >> 16) as u16);
    //                         };
    //                     };
    //                     (r, g, b)
    //                 };
    //                 self.data[index];
    //                 index += 1;

    //                 // BRIGHTNESS
    //                 // JOIN
    //                 for _ in 0..self.inaccuracy {
    //                     for _ in 0..self.inaccuracy {
    //                         thing[image_data_byte_pos] = render_joiner::multiply_factor(objects.data[image_data_byte_pos], this_light.0);
    //                         image_data_byte_pos += 1;
    //                         thing[image_data_byte_pos] = render_joiner::multiply_factor(objects.data[image_data_byte_pos], this_light.1);
    //                         image_data_byte_pos += 1;
    //                         thing[image_data_byte_pos] = render_joiner::multiply_factor(objects.data[image_data_byte_pos], this_light.2);
    //                         image_data_byte_pos += 1;
    //                         //thing[image_data_byte_pos] = 255;
    //                         image_data_byte_pos += 1;
    //                     }
    //                     image_data_byte_pos += image_data_byte_width_without_modified_pixels;
    //                 }
    //                 image_data_byte_pos -= image_data_byte_count_inaccuracy_rows; // return to previous position
    //                 image_data_byte_pos += image_data_byte_count_inaccuracy_pixels; // go one pixel to the right
    //                 // JOIN
    //             }
    //         }
    //     }
    // }
}


pub struct ObjectNoLightRenderer {
    width: usize,
    height: usize,
    buffer: Vec<(u8, u8, u8)>
} impl ObjectNoLightRenderer {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            width: w, height: h, buffer: vec![(0,0,0); w*h],
        }
    }
    pub fn draw_init(&mut self, world: &mut crate::world::world::World) {
        // call draw_init on the objects
        for object in world.objects_rendered.iter_mut() {
            (object.fns.draw_init)(&mut object.state);
        }
    }
    pub fn draw_all(&mut self, world: &mut World) {
        self.draw_to_layers(world);
        self.draw_to_buffer(world);
    }
    pub fn draw_to_layers(&mut self, world: &mut World) {
        let elapsed_time = world.start_time.elapsed();
        for object in world.objects_rendered.iter_mut() {
            (object.fns.draw_again)(&mut object.state, &elapsed_time);
        }
    }
    pub fn draw_to_buffer(&mut self, world: &crate::world::world::World) {
        for object in world.objects_rendered.iter() {
            object.state.layer.draw_onto(&mut self.buffer, self.width, self.height);
        }
    }
}


pub mod render_joiner {

    use super::{ObjectNoLightRenderer, LightMap, WorldRenderer};

    pub fn join(buffer: &mut Vec<u8>, data: &WorldRenderer) {
        // there are multiple functions which can join light and object data
        join5(buffer, data)
    }

    fn join5(buffer: &mut Vec<u8>, data: &WorldRenderer) {
        let mut objects_index = 0;
        let mut buffer_index = 0;
        for y_buffer in 0..data.height { // for each line of pixels on the screen
            let lights_index_row = (y_buffer * data.lights_renderer.height / data.height) * data.lights_renderer.width;
            for x_buffer in 0..data.width { // for each pixel in this line
                let lights_index_pixel = lights_index_row + x_buffer * data.lights_renderer.width / data.width;
                let light = data.lights_renderer.data[lights_index_pixel];
                let obj = &data.objects_renderer.buffer[objects_index];
                buffer[buffer_index] = multiply_factor(obj.0, light.0);
                buffer_index += 1;
                buffer[buffer_index] = multiply_factor(obj.1, light.1);
                buffer_index += 1;
                buffer[buffer_index] = multiply_factor(obj.2, light.2);
                buffer_index += 2;
                objects_index += 1;
            }
        }
    }

    // ALL OF THESE WERE MADE BEFORE OBJECTS WERE DRAWN WITH LAYERS!

    // // this is join3, just more optimized (i think)
    // fn join4(buffer: &mut Vec<u8>, width: u32, height: u32, objects: &ObjectNoLightRenderer, light_map: &LightMap) {
    //     let widthsize = width ;
    //     //let heightsize = height ;
    //     let inaccuracy = light_map.inaccuracy;
    //     let inaccuracysize = inaccuracy ;
    //     let light_map_width_size = light_map.width ;
    //     let light_map_height_size = light_map.height ;
    //     let row_length_in_bytes_minus_light_pixel_width = (width - inaccuracy)  * 4;
    //     for y in 0..light_map_height_size {
    //         let Y = y * inaccuracysize;
    //         let light_index_start_of_row = y * light_map_width_size;
    //         let pixel_index_start_of_row = Y * widthsize;
    //         for x in 0..light_map_width_size {
    //             let X = x * inaccuracysize;
    //             let light_index = light_index_start_of_row + x;
    //             let mut pixel_index = (pixel_index_start_of_row + X)  * 4;
    //             let light = light_map.data[light_index ];
    //             for _ in 0..inaccuracysize {
    //                 for _ in 0..inaccuracysize {
    //                     buffer[pixel_index] = multiply_factor(objects.data[pixel_index], light.0);
    //                     pixel_index += 1;
    //                     buffer[pixel_index] = multiply_factor(objects.data[pixel_index], light.1);
    //                     pixel_index += 1;
    //                     buffer[pixel_index] = multiply_factor(objects.data[pixel_index], light.2);
    //                     pixel_index += 1;
    //                     buffer[pixel_index] = 255;
    //                     pixel_index += 1;
    //                 } // ^^  this loop increments pixel_index by 'light_pixel_width'  ^^
    //                 pixel_index += row_length_in_bytes_minus_light_pixel_width;
    //             }
    //         }
    //     }
    // }

    // // this is way faster than any other join functions so far
    // fn join3(buffer: &mut Vec<u8>, width: u32, height: u32, objects: &ObjectNoLightRenderer, light_map: &LightMap) {
    //     let inaccuracy = light_map.inaccuracy;
    //     let inaccuracysize = inaccuracy ;
    //     let row_length_in_bytes_minus_light_pixel_width = (width - inaccuracy)  * 4;
    //     for y in 0..light_map.height {
    //         let Y = y * inaccuracy;
    //         let light_index_start_of_row = y * light_map.width;
    //         let pixel_index_start_of_row = Y * width;
    //         for x in 0..light_map.width {
    //             let X = x * light_map.inaccuracy;
    //             let light_index = light_index_start_of_row + x;
    //             let mut pixel_index = (pixel_index_start_of_row + X)  * 4;
    //             let light = light_map.data[light_index ];
    //             for Y in 0..inaccuracysize {
    //                 for X in 0..inaccuracysize {
    //                     buffer[pixel_index] = multiply_factor(objects.data[pixel_index], light.0);
    //                     pixel_index += 1;
    //                     buffer[pixel_index] = multiply_factor(objects.data[pixel_index], light.1);
    //                     pixel_index += 1;
    //                     buffer[pixel_index] = multiply_factor(objects.data[pixel_index], light.2);
    //                     pixel_index += 1;
    //                     buffer[pixel_index] = 255;
    //                     pixel_index += 1;
    //                 } // ^^  this loop increments pixel_index by 'light_pixel_width'  ^^
    //                 pixel_index += row_length_in_bytes_minus_light_pixel_width;
    //             }
    //         }
    //     }
    // }

    // // ?
    // fn join2(buffer: &mut Vec<u8>, width: u32, height: u32, objects: &ObjectNoLightRenderer, light_map: &LightMap) {
    //     let height = height ;
    //     let bfwidth4 = 4 * width ;
    //     let lmwidth = light_map.width ;
    //     let lmheight = light_map.height ;
    //     let inaccuracy = light_map.inaccuracy ;
    //     let mut bfline = 0;
    //     for lmline in 0..lmheight {
    //         for bflined in 0..inaccuracy {
    //             let mut pos_buffer = bfwidth4 * bfline;
    //             let mut pos_light_map = lmwidth * lmline;
    //             bfline += 1;
    //             for _ in 0..lmwidth {
    //                 let light = light_map.data[pos_light_map];
    //                 for _ in 0..inaccuracy {
    //                     buffer[pos_buffer] = multiply_factor(objects.data[pos_buffer], light.0);
    //                     pos_buffer += 1;
    //                     buffer[pos_buffer] = multiply_factor(objects.data[pos_buffer], light.1);
    //                     pos_buffer += 1;
    //                     buffer[pos_buffer] = multiply_factor(objects.data[pos_buffer], light.2);
    //                     pos_buffer += 1;
    //                     buffer[pos_buffer] = 255;
    //                     pos_buffer += 1;
    //                 }
    //                 pos_light_map += 1;
    //             }
    //         }
    //     }
    // }

    // // first edition. might work in some edge cases where join3 fails (if they exist), but is WAY slower.
    // fn join1(buffer: &mut Vec<u8>, width: u32, height: u32, objects: &ObjectNoLightRenderer, light_map: &LightMap) {
    //     for y in 0..height {
    //         for x in 0..width {
    //             let index_pixel = x + y * width;
    //             let light_x = x * light_map.width / width;
    //             let light_y = y * light_map.height / height;
    //             let light_index = light_x + light_y * light_map.width;
    //             let lighting_info = light_map.data[light_index ];
    //             let index_rgba = (index_pixel * 4) ;
    //             buffer[index_rgba+0] = multiply_factor(objects.data[index_rgba], lighting_info.0);
    //             buffer[index_rgba+1] = multiply_factor(objects.data[index_rgba+1], lighting_info.1);
    //             buffer[index_rgba+2] = multiply_factor(objects.data[index_rgba+2], lighting_info.2);
    //             buffer[index_rgba+3] = 255; // alpha?
    //         }
    //     }
    // }

    // fn only_light(buffer: &mut Vec<u8>, width: u32, height: u32, objects: &ObjectNoLightRenderer, light_map: &LightMap) {
    //     for y in 0..light_map.height {
    //         for x in 0..light_map.width {
    //             let light = light_map.data[(y*light_map.width+x) ];
    //             let mut i = 4 * (y*width+x) ;
    //             buffer[i] = multiply_factor(255, light.0);
    //             i += 1;
    //             buffer[i] = multiply_factor(255, light.1);
    //             i += 1;
    //             buffer[i] = multiply_factor(255, light.2);
    //             i += 1;
    //             i += 1;
    //         }
    //     }
    // }
    // fn only_objects(buffer: &mut Vec<u8>, width: u32, height: u32, objects: &ObjectNoLightRenderer, light_map: &LightMap) {
    //     for i in 0..buffer.len() {
    //         buffer[i] = objects.data[i];
    //     }
    // }

    pub fn multiply_factor(n1: u8, n2: u16) -> u8 {
        ((n1 as usize * n2 as usize) >> 16) as u8
    }
}