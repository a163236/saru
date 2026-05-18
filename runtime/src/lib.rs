use std::sync::Mutex;
use std::sync::OnceLock;
use std::time::Instant;

use fontdue::{Font, FontSettings};
use sdl3::Sdl;
use sdl3::video::Window;
use sdl3::render::Canvas;
use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Keycode;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub action_a: bool,
    pub action_b: bool,
    pub start: bool,
    pub quit: bool,
}

pub struct RuntimeContext {
    pub sdl: Sdl,
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub init_time: Instant,
    pub last_time: Instant,
    pub delta_time: f32,
    pub input: InputState,
}

unsafe impl Send for RuntimeContext {}
unsafe impl Sync for RuntimeContext {}

static RUNTIME: Mutex<Option<RuntimeContext>> = Mutex::new(None);
static JAPANESE_FONT: OnceLock<Font> = OnceLock::new();

fn load_japanese_font() -> Font {
    let paths = [
        "/usr/share/fonts/truetype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc",
        "/usr/share/fonts/truetype/noto/NotoSansJP-Regular.otf",
        "/mnt/c/Windows/Fonts/NotoSansJP-VF.ttf",
        "/mnt/c/Windows/Fonts/meiryo.ttc",
        "/mnt/c/Windows/Fonts/YuGothR.ttc",
    ];

    for path in paths {
        if let Ok(data) = std::fs::read(path) {
            if let Ok(font) = Font::from_bytes(data, FontSettings::default()) {
                println!("SaruOS Runtime: loaded Japanese font from {}", path);
                return font;
            }
        }
    }

    let embedded = include_bytes!("../fonts/NotoSansJP-VF.ttf");
    Font::from_bytes(embedded.as_slice(), FontSettings::default())
        .expect("Failed to load embedded Japanese font")
}

fn japanese_font() -> &'static Font {
    JAPANESE_FONT.get_or_init(load_japanese_font)
}

#[no_mangle]
pub extern "C" fn runtime_init() {
    let mut runtime = RUNTIME.lock().unwrap();
    if runtime.is_some() {
        return;
    }

    let _ = japanese_font();

    let sdl = sdl3::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video.window("SaruOS Game Window", 1280, 720)
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas();
    let event_pump = sdl.event_pump().unwrap();

    *runtime = Some(RuntimeContext {
        sdl,
        canvas,
        event_pump,
        init_time: Instant::now(),
        last_time: Instant::now(),
        delta_time: 0.0,
        input: InputState {
            up: false,
            down: false,
            left: false,
            right: false,
            action_a: false,
            action_b: false,
            start: false,
            quit: false,
        },
    });

    println!("SaruOS Runtime initialized with SDL3 graphics & input!");
}

#[no_mangle]
pub extern "C" fn runtime_shutdown() {
    let mut runtime = RUNTIME.lock().unwrap();
    *runtime = None;
    println!("SaruOS Runtime shutdown!");
}

#[no_mangle]
pub extern "C" fn runtime_get_delta_time() -> f32 {
    let mut runtime = RUNTIME.lock().unwrap();
    if let Some(ref mut rt) = *runtime {
        let now = Instant::now();
        rt.delta_time = now.duration_since(rt.last_time).as_secs_f32();
        rt.last_time = now;
        rt.delta_time
    } else {
        0.0
    }
}

#[no_mangle]
pub extern "C" fn runtime_get_input() -> InputState {
    let mut runtime = RUNTIME.lock().unwrap();
    if let Some(ref mut rt) = *runtime {
        rt.input.quit = false;
        rt.input.start = false;

        let elapsed = rt.init_time.elapsed().as_secs_f32();

        while let Some(event) = rt.event_pump.poll_event() {
            match event {
                Event::Quit { .. } => {
                    if elapsed > 0.2 {
                        rt.input.quit = true;
                    }
                }
                Event::KeyDown { keycode: Some(key), .. } => {
                    match key {
                        Keycode::Up | Keycode::W => rt.input.up = true,
                        Keycode::Down | Keycode::S => rt.input.down = true,
                        Keycode::Left | Keycode::A => rt.input.left = true,
                        Keycode::Right | Keycode::D => rt.input.right = true,
                        Keycode::Return | Keycode::J => rt.input.action_a = true,
                        Keycode::Space | Keycode::K => rt.input.action_b = true,
                        Keycode::Escape => {
                            if elapsed > 0.2 {
                                rt.input.start = true;
                            }
                        }
                        _ => {}
                    }
                }
                Event::KeyUp { keycode: Some(key), .. } => {
                    match key {
                        Keycode::Up | Keycode::W => rt.input.up = false,
                        Keycode::Down | Keycode::S => rt.input.down = false,
                        Keycode::Left | Keycode::A => rt.input.left = false,
                        Keycode::Right | Keycode::D => rt.input.right = false,
                        Keycode::Return | Keycode::J => rt.input.action_a = false,
                        Keycode::Space | Keycode::K => rt.input.action_b = false,
                        Keycode::Escape => rt.input.start = false,
                        _ => {}
                    }
                }
                _ => {}
            }
        }
        rt.input
    } else {
        InputState {
            up: false,
            down: false,
            left: false,
            right: false,
            action_a: false,
            action_b: false,
            start: false,
            quit: false,
        }
    }
}

#[no_mangle]
pub extern "C" fn runtime_clear() {
    let mut runtime = RUNTIME.lock().unwrap();
    if let Some(ref mut rt) = *runtime {
        rt.canvas.set_draw_color(sdl3::pixels::Color::RGB(10, 12, 18));
        rt.canvas.clear();
    }
}

#[no_mangle]
pub extern "C" fn runtime_present() {
    let mut runtime = RUNTIME.lock().unwrap();
    if let Some(ref mut rt) = *runtime {
        rt.canvas.present();
    }
}

#[no_mangle]
pub extern "C" fn runtime_draw_rect(x: i32, y: i32, w: u32, h: u32, r: u8, g: u8, b: u8) {
    let mut runtime = RUNTIME.lock().unwrap();
    if let Some(ref mut rt) = *runtime {
        rt.canvas.set_draw_color(sdl3::pixels::Color::RGB(r, g, b));
        let rect = sdl3::rect::Rect::new(x, y, w, h);
        let _ = rt.canvas.fill_rect(rect);
    }
}

#[no_mangle]
pub extern "C" fn runtime_draw_text(x: i32, y: i32, text: &str, size: f32, r: u8, g: u8, b: u8) {
    let font = japanese_font();
    let mut runtime = RUNTIME.lock().unwrap();
    let Some(ref mut rt) = *runtime else { return; };

    rt.canvas.set_draw_color(sdl3::pixels::Color::RGB(r, g, b));

    // y is the top of the line box; anchor all glyphs to one shared baseline.
    let ascent = font
        .horizontal_line_metrics(size)
        .map(|m| m.ascent)
        .unwrap_or(size * 0.85);
    let baseline_y = y as f32 + ascent;
    let mut cursor_x = x as f32;

    for ch in text.chars() {
        let (metrics, bitmap) = font.rasterize(ch, size);
        let width = metrics.width;
        if width == 0 {
            cursor_x += metrics.advance_width;
            continue;
        }

        // fontdue: ymin is the bottom edge offset from baseline (y-down screen coords).
        let glyph_top = baseline_y - metrics.height as f32 - metrics.ymin as f32;

        for (idx, coverage) in bitmap.iter().enumerate() {
            if *coverage < 48 {
                continue;
            }
            let gx = idx % width;
            let gy = idx / width;
            let px = cursor_x + metrics.xmin as f32 + gx as f32;
            let py = glyph_top + gy as f32;
            let rect = sdl3::rect::Rect::new(px as i32, py as i32, 1, 1);
            let _ = rt.canvas.fill_rect(rect);
        }
        cursor_x += metrics.advance_width;
    }
}
