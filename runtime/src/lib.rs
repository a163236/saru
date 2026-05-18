use std::sync::Mutex;
use std::time::Instant;

// SDL3のアイテムをインポート
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

#[no_mangle]
pub extern "C" fn runtime_init() {
    let mut runtime = RUNTIME.lock().unwrap();
    if runtime.is_some() {
        return;
    }
    
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
        // quit および start（Escapeでの終了要求）は毎フレームクリアする（トリガー型）
        rt.input.quit = false;
        rt.input.start = false;

        let elapsed = rt.init_time.elapsed().as_secs_f32();

        while let Some(event) = rt.event_pump.poll_event() {
            match event {
                Event::Quit { .. } => {
                    // 起動直後の誤判定を防ぐため、0.2秒以上経過している場合のみ終了を受け付ける
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
                        Keycode::Escape => {
                            // キーが離された時は常にリセット
                            rt.input.start = false;
                        }
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
        rt.canvas.set_draw_color(sdl3::pixels::Color::RGB(10, 12, 18)); // 高級感のあるダークネイビー背景
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

