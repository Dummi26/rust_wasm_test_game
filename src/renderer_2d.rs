use gloo::events::EventListener;
use wasm_bindgen::{closure::Closure, JsCast};
use web_sys::{CanvasRenderingContext2d as RenderingContext, Performance, Window, Document};
use std::{rc::Rc, cell::RefCell, time::Duration, collections::VecDeque, sync::mpsc::{self, Sender, Receiver}};

use crate::{world};

pub struct ImpInfo {
    pub context: RenderingContext,
    pub document: Document,
    pub window: Window,
    pub width: u32,
    pub height: u32,
    pub image_bytes: Vec<u8>,
    pub world_renderer: world::render_world::WorldRenderer,
    pub message_sender: Sender<Interactions>,
    pub message_receiver: Receiver<Interactions>,
}

pub enum Interactions {
    MouseDown { button: i16, x: i32, y: i32, },
    MouseMove { button: i16, x: i32, y: i32, },
}

fn render(info: &mut ImpInfo) {
    let time_start = wasm_timer::Instant::now();

    let width = info.width;
    let height = info.height;
    let window_width_f = info.window.inner_width().unwrap().as_f64().unwrap() as f32;
    let window_height_f = info.window.inner_height().unwrap().as_f64().unwrap() as f32;
    let image_bytes = &mut info.image_bytes;
    let world_renderer = &mut info.world_renderer;
    let context = &info.context;
    let document = &info.document;
    let message_receiver = &info.message_receiver;

    // if the data vec is too long or too short, adjust its size.
    let byte_length = (width * height * 4) as usize;
    if image_bytes.len() != byte_length {
        let mut len = image_bytes.len();
        while len > byte_length {
            image_bytes.pop();
            len -= 1;
        }
        while len < byte_length {
            image_bytes.push(255u8);
            len += 1;
        }
    }
    // handle channel
    loop {
        match message_receiver.try_recv() {
            Ok(received) => {
                match received {
                    Interactions::MouseDown { button, x, y } => {},
                    Interactions::MouseMove { button, x, y } => {
                        let light = &mut world_renderer.world.renderable.lights_rendered[0];
                        light.x = (-1.0 + 2.0 * x as f32 / window_width_f) * world_renderer.world.renderable.width;
                        light.y = (-1.0 + 2.0 * y as f32 / window_height_f) * world_renderer.world.renderable.height;
                    },
                }
            },
            Err(_) => {
                break;
            },
        }
    }

    // OTHER
    /*
    world_renderer.world.renderable.lights_rendered[0].x += 5.0;
    if world_renderer.world.renderable.lights_rendered[0].x > world_renderer.world.renderable.width * 2.0 {
        world_renderer.world.renderable.lights_rendered[0].x = -world_renderer.world.renderable.width * 2.0;
    }
    */
    world_renderer.world.renderable.lights_rendered[1].x += 7.0;
    if world_renderer.world.renderable.lights_rendered[1].x > world_renderer.world.renderable.width * 2.0 {
        world_renderer.world.renderable.lights_rendered[1].x = -world_renderer.world.renderable.width * 2.0;
    }
    world_renderer.world.renderable.lights_rendered[2].x += -3.0;
    if world_renderer.world.renderable.lights_rendered[2].x < -world_renderer.world.renderable.width * 2.0 {
        world_renderer.world.renderable.lights_rendered[2].x = world_renderer.world.renderable.width * 2.0;
    }
    // render data to array
    let durations;
    {
        durations = world_renderer.render(image_bytes);
    }
    // put image data
    let image_data = web_sys::ImageData::new_with_u8_clamped_array(wasm_bindgen::Clamped(image_bytes), width).unwrap();
    match context.put_image_data(&image_data, 0.0, 0.0) { Ok(_) => {}, Err(_) => {}, }
    let time_render = time_start.elapsed();
    document.set_title(format!("Took {}={}+{}+{}ms to render {}x{}px", time_render.as_millis(), durations[0].as_millis(), durations[1].as_millis(), durations[2].as_millis(), width, height).as_str());
}

pub fn init_renderer(gl: RenderingContext, width: u32, height: u32, window: Window, document: Document) {

    web_sys::console::log_1(&"Initializing renderer...".into());
    //let start_time = wasm_timer::Instant::now();

    let mut world_renderer = world::render_world::WorldRenderer::new(world::world::World::new(world::world::WorldRenderable::new(1600f32/9f32, 100f32)), width, height);
    let width = world_renderer.width;
    let height = world_renderer.height;

    world_renderer.world.renderable.lights_rendered.push(world::world::Object::Objects::LightObject::new(0.0, 0.0, (50000, 50000, 50000), 50.0, 50.0));
    world_renderer.world.renderable.lights_rendered.push(world::world::Object::Objects::LightObject::new(0.0, -100.0, (00000, 50000, 50000), 25.0, 50.0));
    world_renderer.world.renderable.lights_rendered.push(world::world::Object::Objects::LightObject::new(0.0, 100.0, (50000, 20000, 20000), 25.0, 75.0));

    /*
    world_renderer.world.renderable.objects_rendered.push(Box::new(world::world::Object::Objects::WorldObject::new(world::world::Object::Objects::WorldObjectData::
        Rectangle((0.0, 25.0, 1600f32/9f32, 50.0, 255, 255, 255))
    )));
    world_renderer.world.renderable.objects_rendered.push(Box::new(world::world::Object::Objects::WorldObject::new(world::world::Object::Objects::WorldObjectData::
        Rectangle((16f32/9f32 * 20.0, 10.0, 16f32/9f32 * 20.0, 20.0, 255, 0, 0))
    )));
    world_renderer.world.renderable.objects_rendered.push(Box::new(world::world::Object::Objects::WorldObject::new(world::world::Object::Objects::WorldObjectData::
        Rectangle((16f32/9f32 * 60.0, 10.0, 16f32/9f32 * 20.0, 20.0, 0, 255, 0))
    )));
    world_renderer.world.renderable.objects_rendered.push(Box::new(world::world::Object::Objects::WorldObject::new(world::world::Object::Objects::WorldObjectData::
        Rectangle((16f32/9f32 * 40.0, 70.0, 16f32/9f32 * 20.0, 20.0, 0, 0, 255))
    )));
    */
    
    {
        let img = crate::assets::image_loader_hardcoded::get_image1_raw_bytes();
        world_renderer.world.renderable.objects_rendered.push(Box::new(world::world::Object::Objects::WorldObject::new(world::world::Object::Objects::WorldObjectData::
            Image((img.0, img.1, img.2, 0.0, 0.0, 1600.0/9.0, 100.0))
        )));
    }

    world_renderer.init();

    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let outer_f = f.clone();

    let image_data = Vec::<u8>::new();

    let message_channel: (Sender<Interactions>, Receiver<Interactions>) = mpsc::channel();
    let message_sender = message_channel.0;
    let message_receiver = message_channel.1;

    // immut_info will be passed to interactions::* -- ENSURE THAT ALL ITS CONTENTS ARE STATIC AND DO NOT CHANGE
    let mut imp_info = ImpInfo {
        context: gl,
        document: document,
        window: window,
        width,
        height,
        image_bytes: image_data,
        world_renderer: world_renderer,
        message_sender: message_sender, // this one can't change, as a copy of it is passed to the EventListeners!
        message_receiver: message_receiver,
    };

    // interactions
    { //    see https://www.w3.org/TR/DOM-Level-3-Events/#event-types
        let sender = imp_info.message_sender.clone();
        EventListener::new(&imp_info.document, "mousedown", move |event| {crate::interactions::mouse::down(event.to_owned(), sender.clone());}).forget();
        let sender = imp_info.message_sender.clone();
        EventListener::new(&imp_info.document, "mousemove", move |event| {crate::interactions::mouse::moved(event.to_owned(), sender.clone());}).forget();
    }

    *outer_f.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        render(&mut imp_info);
        imp_info.window.request_animation_frame(f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("failed requesting animation frame");
    }) as Box<dyn FnMut()>));

    let window = web_sys::window().unwrap();
    window.request_animation_frame(outer_f.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .expect("failed requesting animation frame");
}