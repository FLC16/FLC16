#![deny(clippy::all)]
#![forbid(unsafe_code)]
#![windows_subsystem = "windows"]

use image;
use log::error;
use native_dialog::FileDialog;
use pixels::{Error, Pixels, SurfaceTexture};
use rodio::source::Source;
use rodio::{OutputStream, Sink};
use waves::{TriangleWave, SquareWave, Noise};
use std::env;
use std::fs;
use std::path::Path;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Icon, WindowBuilder};
use winit_input_helper::WinitInputHelper;
mod blackspace;
mod eval;
mod waves;

const WIDTH: u32 = 256 * 2;
const HEIGHT: u32 = 144 * 2;

const FREQS: [f32; 108] = [
    16.35, 17.32, 18.35, 19.45, 20.60, 21.83, 23.12, 24.50, 25.96, 27.50, 29.14, 30.87, 32.70,
    34.65, 36.71, 38.89, 41.20, 43.65, 46.25, 49.00, 51.91, 55.00, 58.27, 61.74, 65.41, 69.30,
    73.42, 77.78, 82.41, 87.31, 92.50, 98.00, 103.83, 110.0, 116.54, 123.47, 130.81, 138.56,
    146.83, 155.56, 164.81, 174.61, 185.0, 196.0, 207.65, 220.0, 233.08, 246.94, 261.63, 277.18,
    293.66, 311.13, 329.63, 349.23, 369.99, 392.0, 415.3, 440.0, 466.16, 493.88, 523.25, 554.37,
    587.33, 622.25, 659.25, 698.46, 739.99, 783.99, 830.61, 880.0, 932.33, 987.77, 1046.50,
    1108.73, 1174.66, 1244.51, 1318.51, 1396.91, 1479.98, 1567.98, 1661.22, 1760.0, 1864.66,
    1975.53, 2093.0, 2217.46, 2349.32, 2489.02, 2637.02, 2793.83, 2959.96, 3135.96, 3322.44,
    3520.0, 3729.31, 3951.07, 4186.01, 4434.92, 4698.63, 4978.03, 5274.04, 5587.65, 5919.91,
    6271.93, 6644.88, 7040.0, 7458.62, 7902.13,
];

struct World {
    pix_buffer: [[u8; 256]; 144],
    initialized: bool,
    routines: Vec<u32>,
    stack: Vec<u8>,
    heap: Vec<u8>,
    bytes: Vec<u8>,
    timer: Instant,
    counter: f32,
    tx: Sender<String>,
    palette: [[u8; 4]; 16]
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    if (&args.len() != &1usize && &args[1] != "blackspace") || &args.len() == &1usize {
        let event_loop = EventLoop::new();
        let mut input = WinitInputHelper::new();
        let window = {
            let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
            let icon = load_icon();
            WindowBuilder::new()
                .with_title("FLC16")
                .with_resizable(false)
                .with_inner_size(size)
                .with_window_icon(Some(icon))
                .build(&event_loop)
                .unwrap()
        };

        let mut pixels = {
            let window_size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(window_size.width, window_size.height, &window);
            Pixels::new(WIDTH, HEIGHT, surface_texture)?
        };
        let mut world = World::new();
        world.heap[0xff0f] = 2;

        if &args.len() > &1 && Path::new(&args[1]).exists() {
            let contents = fs::read(&args[1]).expect("Something went wrong reading the file");
            world.bytes = contents;
            let evaled = eval::bcode(
                &world.bytes,
                &mut world.pix_buffer,
                false,
                0,
                world.heap,
                world.stack,
                world.routines,
                &world.tx,
                &mut world.palette
            );
            world.initialized = true;
            world.heap = evaled.2;
            world.stack = evaled.1;
            world.routines = evaled.0;
        } else {
            let image = image::load_from_memory(
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/icon/nodisk.png")))
                .expect("Failed to open icon path")
                .into_rgb8();
            let mut x = 0;
            let mut y = 0;
            for pix in image.pixels() {
                world.pix_buffer[y][x] = match pix.0[0] {
                    67 => 0x0d,
                    148 => 0x0e,
                    54 => 0x0a,
                    255 => 0x02,
                    _ => 0x00,
                };
                x = x + 1;
                if x > 255 {
                    x = 0;
                    y = y + 1;
                }
            }
        }

        event_loop.run(move |event, _, control_flow| {
            if let Event::RedrawRequested(_) = event {
                world.draw(pixels.get_frame());
                if pixels
                    .render()
                    .map_err(|e| error!("pixels.render() failed: {}", e))
                    .is_err()
                {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }

            if input.update(&event) {
                if input.quit() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }

                world.heap[0xff01] = if input.key_held(VirtualKeyCode::Up) { 1 } else { 0 };
                world.heap[0xff02] = if input.key_held(VirtualKeyCode::Down) { 1 } else { 0 };
                world.heap[0xff03] = if input.key_held(VirtualKeyCode::Left) { 1 } else { 0 };
                world.heap[0xff04] = if input.key_held(VirtualKeyCode::Right) { 1 } else { 0 };
                world.heap[0xff05] = if input.key_held(VirtualKeyCode::Z) { 1 } else { 0 };
                world.heap[0xff06] = if input.key_held(VirtualKeyCode::X) { 1 } else { 0 };

                let mut addr = [0u16; 6];
                if input.key_pressed(VirtualKeyCode::Up) {
                    addr[0] = 0xff01;
                }
                if input.key_pressed(VirtualKeyCode::Down) {
                    addr[1] = 0xff02;
                }
                if input.key_pressed(VirtualKeyCode::Left) {
                    addr[2] = 0xff03;
                }
                if input.key_pressed(VirtualKeyCode::Right) {
                    addr[3] = 0xff04;
                }
                if input.key_pressed(VirtualKeyCode::Z) {
                    addr[4] = 0xff05;
                }
                if input.key_pressed(VirtualKeyCode::X) {
                    addr[5] = 0xff06;
                }
                if input.key_pressed(VirtualKeyCode::O) {
                    let path = FileDialog::new()
                        .add_filter("FLC Disk", &["flc"])
                        .show_open_single_file()
                        .unwrap();

                    match path {
                        Some(path) => {
                            let contents = fs::read(path).expect("Something went wrong reading the file");
                            world.bytes = contents;
                            let evaled = eval::bcode(
                                &world.bytes,
                                &mut world.pix_buffer,
                                false,
                                0,
                                world.heap.clone(),
                                world.stack.clone(),
                                world.routines.clone(),
                                &world.tx,
                                &mut world.palette
                            );
                            world.initialized = true;
                            world.heap = evaled.2;
                            world.stack = evaled.1;
                            world.routines = evaled.0;
                            world.timer = Instant::now();
                        }
                        None => {}
                    };
                }
                for address in addr {
                    if address != 0 && world.routines[address as usize] != 0 {
                        let evaled = eval::bcode(
                            &world.bytes,
                            &mut world.pix_buffer,
                            true,
                            world.routines[address as usize] as usize,
                            world.heap.clone(),
                            world.stack.clone(),
                            world.routines.clone(),
                            &world.tx,
                            &mut world.palette
                        );
                        world.heap = evaled.2;
                        world.stack = evaled.1;
                        world.routines = evaled.0;
                    }
                }

                world.update();
                window.request_redraw();
            }
        });
    } else {
        blackspace::bs();
    }
    return Result::Ok(());
}

impl World {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel::<String>();

        thread::spawn(move || {
            let (_stream, stream_handle) = OutputStream::try_default().unwrap();
            let mut trisink = Sink::try_new(&stream_handle).unwrap();
            let mut squsink = Sink::try_new(&stream_handle).unwrap();
            let mut noisink = Sink::try_new(&stream_handle).unwrap();
            for received in rx {
                let args = received.as_str().split(" ").collect::<Vec<&str>>();
                match args[0] {
                    "beep" => {
                        let source = TriangleWave::new(FREQS[args[1].parse::<u8>().unwrap() as usize])
                            .take_duration(Duration::from_secs_f32(
                                args[2].parse::<f32>().unwrap() * 0.1,
                            ))
                            .amplify(0.20);
                        trisink.append(source);
                    }
                    "boop" => {
                        let source = SquareWave::new(FREQS[args[1].parse::<u8>().unwrap() as usize])
                            .take_duration(Duration::from_secs_f32(
                                args[2].parse::<f32>().unwrap() * 0.1,
                            ))
                            .amplify(0.20);
                        squsink.append(source);
                    }
                    "noise" => {
                        let source = Noise::new()
                            .take_duration(Duration::from_secs_f32(
                                args[1].parse::<f32>().unwrap() * 0.1,
                            ))
                            .amplify(0.20);
                        noisink.append(source);
                    }
                    "empty" => {
                        trisink.stop(); squsink.stop(); noisink.stop();
                        trisink = Sink::try_new(&stream_handle).unwrap();
                        squsink = Sink::try_new(&stream_handle).unwrap();
                        noisink = Sink::try_new(&stream_handle).unwrap();
                    }
                    _ => {}
                }
            }
        });
        Self {
            pix_buffer: [[0u8; 256]; 144],
            routines: vec![0u32; 65536],
            stack: Vec::with_capacity(256),
            heap: vec![0u8; 65536],
            bytes: Vec::new(),
            initialized: false,
            timer: Instant::now(),
            counter: 0.0,
            tx,
            palette: [[0x00, 0x00, 0x00, 0xff],
                [0xab, 0x52, 0x36, 0xff],
                [0xff, 0xf1, 0xe8, 0xff],
                [0xff, 0x84, 0x26, 0xff],
                [0x5f, 0x57, 0x4f, 0xff],
                [0xff, 0xdd, 0x34, 0xff],
                [0x50, 0xe1, 0x12, 0xff],
                [0x3f, 0xa6, 0x6f, 0xff],
                [0x00, 0xff, 0xcc, 0xff],
                [0x29, 0xad, 0xff, 0xff],
                [0x36, 0x59, 0x87, 0xff],
                [0x00, 0x33, 0xff, 0xff],
                [0xc2, 0xc3, 0xc7, 0xff],
                [0x43, 0x00, 0x67, 0xff],
                [0x94, 0x21, 0x6a, 0xff],
                [0xff, 0x00, 0x4d, 0xff]
            ]
        }
    }

    fn update(&mut self) {
        if self.routines[0xfffe] != 0 {
            let evaled = eval::bcode(
                &self.bytes,
                &mut self.pix_buffer,
                true,
                self.routines[0xfffe] as usize,
                self.heap.clone(),
                self.stack.clone(),
                self.routines.clone(),
                &self.tx,
                &mut self.palette
            );
            self.heap = evaled.2;
            self.stack = evaled.1;
            self.routines = evaled.0;
        }
        let dt = self.timer.elapsed();
        self.counter += dt.as_secs_f32();
        self.timer = Instant::now();
        let thresh = 1.0 / self.heap[0xff0f] as f32;
        if self.counter >= thresh && self.routines[0xff0f] != 0 {
            let evaled = eval::bcode(
                &self.bytes,
                &mut self.pix_buffer,
                true,
                self.routines[0xff0f] as usize,
                self.heap.clone(),
                self.stack.clone(),
                self.routines.clone(),
                &self.tx,
                &mut self.palette
            );
            self.heap = evaled.2;
            self.stack = evaled.1;
            self.routines = evaled.0;
            self.counter = self.counter - thresh;
        } else if self.routines[0xff0f] == 0 {
            self.counter = 0.0;
        }
    }

    fn draw(&mut self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = (&i % WIDTH as usize) / 2;
            let y = (&i / WIDTH as usize) / 2;

            let rgba = self.palette[self.pix_buffer[y][x] as usize];

            pixel.copy_from_slice(&rgba);
        }
        if self.routines[0xffff] != 0 {
            if self.heap[0xff0e] == 1 {
                self.pix_buffer = [[0u8; 256]; 144];
            }
            let evaled = eval::bcode(
                &self.bytes,
                &mut self.pix_buffer,
                true,
                self.routines[0xffff] as usize,
                self.heap.clone(),
                self.stack.clone(),
                self.routines.clone(),
                &self.tx,
                &mut self.palette
            );
            self.heap = evaled.2;
            self.stack = evaled.1;
            self.routines = evaled.0;
        }
    }
}

fn load_icon() -> Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::load_from_memory(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/icon/icon.png")))
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon")
}
