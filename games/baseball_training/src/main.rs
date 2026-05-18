use runtime::{
    runtime_init, runtime_shutdown, runtime_get_delta_time,
    runtime_get_input, runtime_clear, runtime_present, runtime_draw_rect,
    runtime_draw_text,
};

use std::cell::Cell;

thread_local! {
    static UI_JAPANESE: Cell<bool> = const { Cell::new(true) };
}

fn set_ui_language(japanese: bool) {
    UI_JAPANESE.with(|flag| flag.set(japanese));
}

// ==========================================
// 🎨 COLOR PALETTE (Retro Arcade Theme)
// ==========================================
const COLOR_BG: (u8, u8, u8) = (10, 15, 26);             // Midnight Blue
const COLOR_CARD: (u8, u8, u8) = (20, 28, 48);           // High-contrast Navy Blue
const COLOR_CARD_HIGHLIGHT: (u8, u8, u8) = (40, 55, 90); // Active Selection Card
const COLOR_TEXT_WHITE: (u8, u8, u8) = (240, 245, 255);  // High contrast white
const COLOR_TEXT_MUTED: (u8, u8, u8) = (130, 140, 160);  // Clean grey-blue
const COLOR_NEON_GREEN: (u8, u8, u8) = (50, 255, 120);   // Success / Vitality
const COLOR_CYAN: (u8, u8, u8) = (0, 240, 255);          // Selected glow
const COLOR_GOLD: (u8, u8, u8) = (255, 190, 0);          // Koshien Gold
const COLOR_RED: (u8, u8, u8) = (255, 50, 50);           // Defeat / Warning
const COLOR_ORANGE: (u8, u8, u8) = (255, 110, 0);        // Strength

// ==========================================
// 🎲 LIGHTWEIGHT RNG (LCG)
// ==========================================
struct SimpleRng {
    state: u64,
}

impl SimpleRng {
    fn new(seed: u64) -> Self {
        SimpleRng { state: if seed == 0 { 5489 } else { seed } }
    }
    
    // Range in [min, max - 1]
    fn next_range(&mut self, min: u32, max: u32) -> u32 {
        if min >= max {
            return min;
        }
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
        let val = (self.state >> 33) as u32;
        min + (val % (max - min))
    }
}

// ==========================================
// 🔠 BILINGUAL RETRO CHAR RASTERIZER
// ==========================================
fn draw_char(x: i32, y: i32, c: char, pixel_size: i32, r: u8, g: u8, b: u8) {
    let pattern = match c {
        '0' => ".###.\n#...#\n#..##\n#.#.#\n##..#\n#...#\n.###.",
        '1' => "..#..\n.##..\n..#..\n..#..\n..#..\n..#..\n.###.",
        '2' => ".###.\n#...#\n....#\n...#.\n..#..\n.#...\n#####",
        '3' => "#####\n....#\n...#.\n..##.\n....#\n#...#\n.###.",
        '4' => "#...#\n#...#\n#...#\n#####\n....#\n....#\n....#",
        '5' => "#####\n#....\n####.\n....#\n....#\n#...#\n.###.",
        '6' => ".###.\n#....\n#....\n####.\n#...#\n#...#\n.###.",
        '7' => "#####\n....#\n...#.\n..#..\n..#..\n.#...\n.#...",
        '8' => ".###.\n#...#\n#...#\n.###.\n#...#\n#...#\n.###.",
        '9' => ".###.\n#...#\n#...#\n.####\n....#\n....#\n.###.",
        'A' => ".###.\n#...#\n#...#\n#####\n#...#\n#...#\n#...#",
        'B' => "####.\n#...#\n#...#\n####.\n#...#\n#...#\n####.",
        'C' => ".####\n#....\n#....\n#....\n#....\n#....\n.####",
        'D' => "####.\n#...#\n#...#\n#...#\n#...#\n#...#\n####.",
        'E' => "#####\n#....\n#....\n####.\n#....\n#....\n#####",
        'F' => "#####\n#....\n#....\n####.\n#....\n#....\n#....",
        'G' => ".####\n#....\n#....\n#.###\n#...#\n#...#\n.####",
        'H' => "#...#\n#...#\n#...#\n#####\n#...#\n#...#\n#...#",
        'I' => "#####\n..#..\n..#..\n..#..\n..#..\n..#..\n#####",
        'J' => "#####\n....#\n....#\n....#\n#...#\n#...#\n.###.",
        'K' => "#...#\n#..#.\n#.#..\n##...\n#.#..\n#..#.\n#...#",
        'L' => "#....\n#....\n#....\n#....\n#....\n#....\n#####",
        'M' => "#...#\n##.##\n#.#.#\n#...#\n#...#\n#...#\n#...#",
        'N' => "#...#\n##..#\n#.#.#\n#..##\n#...#\n#...#\n#...#",
        'O' => ".###.\n#...#\n#...#\n#...#\n#...#\n#...#\n.###.",
        'P' => "####.\n#...#\n#...#\n####.\n#....\n#....\n#....",
        'Q' => ".###.\n#...#\n#...#\n#...#\n#.#.#\n#..#.\n.##.#",
        'R' => "####.\n#...#\n#...#\n####.\n#.#..\n#..#.\n#...#",
        'S' => ".####\n#....\n#....\n.###.\n....#\n....#\n####.",
        'T' => "#####\n..#..\n..#..\n..#..\n..#..\n..#..\n..#..",
        'U' => "#...#\n#...#\n#...#\n#...#\n#...#\n#...#\n.###.",
        'V' => "#...#\n#...#\n#...#\n#...#\n#...#\n.#.#.\n..#..",
        'W' => "#...#\n#...#\n#...#\n#...#\n#.#.#\n##.##\n#...#",
        'X' => "#...#\n#...#\n.#.#.\n..#..\n.#.#.\n#...#\n#...#",
        'Y' => "#...#\n#...#\n.#.#.\n..#..\n..#..\n..#..\n..#..",
        'Z' => "#####\n....#\n...#.\n..#..\n.#...\n#....\n#####",
        ':' => ".....\n..#..\n.....\n.....\n..#..\n.....\n.....",
        '-' => ".....\n.....\n.....\n.###.\n.....\n.....\n.....",
        '.' => ".....\n.....\n.....\n.....\n.....\n..#..\n.....",
        ' ' => ".....\n.....\n.....\n.....\n.....\n.....\n.....",
        '!' => "..#..\n..#..\n..#..\n..#..\n.....\n..#..\n.....",
        '?' => ".###.\n#...#\n....#\n...#.\n..#..\n.....\n..#..",
        '+' => ".....\n..#..\n..#..\n.###.\n..#..\n..#..\n.....",
        '/' => "....#\n...#.\n..#..\n..#..\n.#...\n#....\n#....",
        '%' => "#...#\n....#\n...#.\n..#..\n.#...\n#....\n#...#",
        '[' => "..##.\n..#..\n..#..\n..#..\n..#..\n..#..\n..##.",
        ']' => ".##..\n..#..\n..#..\n..#..\n..#..\n..#..\n.##..",
        '=' => ".....\n.....\n.###.\n.....\n.###.\n.....\n.....",
        '>' => "#....\n.#...\n..#..\n...#.\n..#..\n.#...\n#....",
        '<' => "....#\n...#.\n..#..\n.#...\n..#..\n...#.\n....#",

        // Katakana 5x7 Font Layouts
        'ア' => "#####\n...#.\n..#..\n.###.\n.#.#.\n#..#.\n#..#.",
        'イ' => ".#...\n.#...\n.##..\n.#.#.\n.#..#\n.#..#\n.#...",
        'ウ' => "..#..\n.###.\n....#\n.####\n#...#\n#...#\n.###.",
        'エ' => "#####\n..#..\n..#..\n.###.\n..#..\n..#..\n#####",
        'オ' => "..#..\n#####\n..#.#\n.###.\n.#.#.\n#..#.\n.##..",
        'カ' => "##.#.\n..##.\n..#..\n.###.\n#..#.\n#..#.\n#..#.",
        'キ' => "#####\n..#..\n#####\n..#..\n.#...\n.#...\n#....",
        'ク' => "#####\n...#.\n..#..\n.#...\n.#...\n#....\n#....",
        'ケ' => "##.#.\n..#..\n.###.\n#..#.\n#..#.\n#..#.\n#..#.",
        'コ' => "#####\n#....\n#....\n#....\n#....\n#....\n#####",
        'サ' => "..#..\n#####\n..#..\n..#..\n..#..\n.#...\n#....",
        'シ' => "#..#.\n.#.#.\n...#.\n..#..\n.#...\n#....\n#####",
        'ス' => "#####\n...#.\n..##.\n.##..\n#.#..\n..#..\n..#..",
        'セ' => "#####\n..#.#\n..#.#\n#####\n..#..\n..#..\n..#..",
        'ソ' => "#..#.\n.#...\n.#...\n..#..\n...#.\n...#.\n....#",
        'タ' => "#####\n#..#.\n#..#.\n#####\n...#.\n..#..\n.#...",
        'チ' => "#####\n..#..\n.###.\n#..#.\n...#.\n..#..\n.#...",
        'ツ' => "#..#.\n.#.##\n...#.\n..#..\n.#...\n#....\n#....",
        'テ' => "#####\n..#..\n#####\n..#..\n..#..\n.#...\n#....",
        'ト' => "..#..\n..#..\n..##.\n..#.#\n..#..\n..#..\n..#..",
        'ナ' => "#####\n..#..\n.###.\n#..#.\n#..#.\n#..#.\n.##..",
        'ニ' => "#####\n.....\n.....\n#####\n.....\n.....\n.....",
        'ヌ' => "#####\n...#.\n.###.\n#..#.\n#..#.\n.###.\n#..#.",
        'ネ' => "##.#.\n..##.\n..#..\n.###.\n#..#.\n#..#.\n#..#.",
        'ノ' => "...#.\n..#..\n.#...\n.#...\n#....\n#....\n#....",
        'ハ' => "..#..\n.#.#.\n#...#\n#...#\n#...#\n#...#\n#...#",
        'ヒ' => "#...#\n#...#\n#####\n#...#\n#...#\n#...#\n#####",
        'フ' => "#####\n...#.\n..#..\n..#..\n.#...\n.#...\n#....",
        'ヘ' => "..#..\n.#.#.\n#...#\n#...#\n.....\n.....\n.....",
        'ホ' => "..#..\n#####\n..#..\n.###.\n#.#.#\n#.#.#\n#...#",
        'マ' => "#####\n...#.\n..##.\n.###.\n..#..\n..#..\n..#..",
        'ミ' => "...#.\n..#..\n.#...\n.#...\n#....\n#....\n#....",
        'ム' => "#####\n#...#\n#...#\n#####\n...#.\n..#..\n.#...",
        'メ' => "#..#.\n.#.#.\n..#..\n.##..\n#.#..\n..#..\n..#..",
        'モ' => "..#..\n#####\n..#..\n#####\n..#..\n.#...\n#....",
        'ヤ' => "#####\n..#..\n.###.\n#..#.\n#..#.\n#..#.\n.##..",
        'ユ' => "#####\n#....\n#..##\n#####\n...#.\n...#.\n...#.",
        'ヨ' => "#####\n#....\n#####\n#....\n#####\n#....\n#####",
        'ラ' => "#####\n...#.\n.###.\n#..#.\n#..#.\n#..#.\n.##..",
        'リ' => "#..#.\n#..#.\n#..#.\n#..#.\n#..#.\n#..#.\n#..#.",
        'ル' => "#####\n#..#.\n#..#.\n#####\n...#.\n..#..\n.#...",
        'レ' => "#....\n#....\n#....\n#....\n#....\n#..#.\n.##..",
        'ロ' => "#####\n#...#\n#...#\n#...#\n#...#\n#...#\n#####",
        'ワ' => "#####\n...#.\n..#..\n.#...\n#....\n#....\n#....",
        'ヲ' => "#####\n..#..\n#####\n..#..\n..#..\n.#...\n#....",
        'ン' => "#....\n.#...\n.#.#.\n..#..\n...#.\n...#.\n....#",
        'ー' => ".....\n.....\n.....\n#####\n.....\n.....\n.....",
        'ッ' => ".....\n.....\n#..#.\n.#.##\n...#.\n..#..\n.#...",
        'ャ' => ".....\n.....\n#####\n..#..\n.###.\n#..#.\n#..#.",
        'ュ' => ".....\n.....\n#####\n#....\n#..##\n#####\n...#.",
        'ョ' => ".....\n.....\n#####\n#....\n#####\n#....\n#####",
        'ァ' => ".....\n..#..\n.###.\n.#.#.\n.....\n.....\n.....",
        'ィ' => ".....\n.#...\n.##..\n.#...\n.....\n.....\n.....",
        'ゥ' => ".....\n.###.\n...#.\n.###.\n.....\n.....\n.....",
        'ェ' => ".....\n.###.\n..#..\n.###.\n.....\n.....\n.....",
        'ォ' => ".....\n..#..\n.###.\n.#.#.\n.....\n.....\n.....",

        // Essential Baseball Kanji
        '球' => "#.###\n###.#\n#.###\n####.\n#.#.#\n#.###\n#.###",
        '速' => "#####\n#...#\n#####\n..#..\n.###.\n#...#\n#####",
        '変' => "..#..\n#####\n.#.#.\n#####\n..#..\n.#.#.\n#...#",
        '化' => "#..#.\n#..#.\n####.\n#..#.\n#..#.\n#..#.\n#..#.",
        '守' => "#####\n..#..\n#####\n..#..\n..#..\n..#..\n..#..",
        '備' => "#.#.#\n#####\n#.#.#\n#####\n#.#.#\n#####\n#.#.#",
        '体' => "#..#.\n##.##\n#.###\n#..#.\n#..#.\n#..#.\n#..#.",
        '力' => "#####\n..#..\n..#..\n..#..\n.#...\n.#...\n#....",
        '手' => "#####\n..#..\n#####\n..#..\n..#..\n..#..\n.##..",
        '選' => "#####\n#.#.#\n#####\n..#..\n.###.\n#...#\n#####",
        '練' => "#.#.#\n#####\n#.#.#\n#####\n#.#.#\n#####\n#.#.#",
        '習' => "#####\n#.#.#\n#####\n..#..\n..#..\n..#..\n#####",
        '休' => "#..#.\n##.##\n#.###\n#..#.\n#..#.\n#..#.\n#..#.",
        '養' => "#####\n#...#\n#####\n..#..\n.###.\n#...#\n#####",
        '試' => "#####\n#...#\n#####\n..#..\n.###.\n#...#\n#####",
        '合' => "..#..\n.#.#.\n#####\n#...#\n#####\n#...#\n#####",
        '甲' => "#####\n#...#\n#####\n..#..\n..#..\n..#..\n..#..",
        '子' => "#####\n..#..\n..#..\n.##..\n..#..\n..#..\n.##..",
        '園' => "#####\n#...#\n#...#\n#####\n#...#\n#...#\n#####",
        '週' => "#####\n#...#\n#####\n..#..\n.###.\n#...#\n#####",
        '目' => "#####\n#...#\n#...#\n#####\n#...#\n#...#\n#####",
        '監' => "#####\n#...#\n#####\n..#..\n.###.\n#...#\n#####",
        '督' => "#.#.#\n#####\n#.#.#\n#####\n#.#.#\n#####\n#.#.#",
        '活' => "#.###\n##.##\n#.###\n#.###\n#.###\n##.##\n#.###",
        '動' => "#.#.#\n#####\n#.#.#\n#####\n#.#.#\n#####\n#.#.#",
        '案' => "#####\n#...#\n#####\n..#..\n.###.\n#...#\n#####",
        '内' => "#####\n#...#\n#.#.#\n#.#.#\n#...#\n#...#\n#...#",
        '外' => "#.#.#\n#####\n#.#.#\n#####\n#.#.#\n#####\n#.#.#",
        '打' => "#..#.\n####.\n#..#.\n#..#.\n#..#.\n#..#.\n#..#.",
        '投' => "#.#.#\n#####\n#.#.#\n#####\n#.#.#\n#####\n#.#.#",

        _ =>   ".....\n.....\n.....\n.....\n.....\n.....\n.....",
    };

    let mut row = 0;
    let mut col = 0;
    for ch in pattern.chars() {
        if ch == '\n' {
            row += 1;
            col = 0;
            continue;
        }
        if ch == '#' {
            runtime_draw_rect(
                x + col * pixel_size,
                y + row * pixel_size,
                pixel_size as u32,
                pixel_size as u32,
                r, g, b
            );
        }
        col += 1;
    }
}

// Normalize text for the pixel font: fullwidth/hiragana/punctuation + missing kanji
fn normalize_display_text(text: &str) -> String {
    let mut result = String::with_capacity(text.len() * 2);
    for c in text.chars() {
        match c {
            '警' => result.push_str("ケイ"),
            '告' => result.push_str("コク"),
            '高' => result.push_str("コウ"),
            '校' => result.push_str("コウ"),
            '敗' => result.push_str("ハイ"),
            '北' => result.push_str("ボク"),
            '勝' => result.push_str("ショウ"),
            '利' => result.push_str("リ"),
            '指' => result.push_str("シ"),
            '名' => result.push_str("メイ"),
            '\u{3000}' => result.push(' '),
            c if ('\u{FF01}'..='\u{FF5E}').contains(&c) => {
                if let Some(h) = char::from_u32(c as u32 - 0xFEE0) {
                    result.push(h);
                }
            }
            c if ('\u{3041}'..='\u{3096}').contains(&c) => {
                if let Some(k) = char::from_u32(c as u32 + 0x60) {
                    result.push(k);
                }
            }
            '。' => result.push('.'),
            '、' => result.push(','),
            '！' => result.push('!'),
            '？' => result.push('?'),
            '：' => result.push(':'),
            '・' => result.push('-'),
            '★' | '■' => result.push('*'),
            'ァ' => result.push('ァ'),
            'ィ' => result.push('ィ'),
            'ゥ' => result.push('ゥ'),
            'ェ' => result.push('ェ'),
            'ォ' => result.push('ォ'),
            _ => result.push(c),
        }
    }
    result
}

// Helper to separate Katakana modifiers dynamically
fn get_katakana_modifiers(c: char) -> (char, bool, bool) {
    match c {
        'ガ' => ('カ', true, false), 'ギ' => ('キ', true, false), 'グ' => ('ク', true, false), 'ゲ' => ('ケ', true, false), 'ゴ' => ('コ', true, false),
        'ザ' => ('サ', true, false), 'ジ' => ('シ', true, false), 'ズ' => ('ス', true, false), 'ゼ' => ('セ', true, false), 'ゾ' => ('ソ', true, false),
        'ダ' => ('タ', true, false), 'ヂ' => ('チ', true, false), 'ヅ' => ('ツ', true, false), 'デ' => ('テ', true, false), 'ド' => ('ト', true, false),
        'バ' => ('ハ', true, false), 'ビ' => ('ヒ', true, false), 'ブ' => ('フ', true, false), 'ベ' => ('ヘ', true, false), 'ボ' => ('ホ', true, false),
        'パ' => ('ハ', false, true), 'ピ' => ('ヒ', false, true), 'プ' => ('フ', false, true), 'ペ' => ('ヘ', false, true), 'ポ' => ('ホ', false, true),
        _ => (c, false, false),
    }
}

fn draw_string(mut x: i32, y: i32, text: &str, pixel_size: i32, r: u8, g: u8, b: u8) {
    if UI_JAPANESE.with(|flag| flag.get()) {
        let font_size = pixel_size as f32 * 2.4 + 8.0;
        runtime_draw_text(x, y + pixel_size, text, font_size, r, g, b);
        return;
    }

    let normalized = normalize_display_text(text);
    for c in normalized.chars() {
        let (base_char, is_dakuten, is_handakuten) = get_katakana_modifiers(c);
        draw_char(x, y, base_char, pixel_size, r, g, b);
        if is_dakuten {
            // Tiny Dakuten double dot overlay
            runtime_draw_rect(x + 4 * pixel_size, y - 1 * pixel_size, pixel_size as u32, pixel_size as u32, r, g, b);
            runtime_draw_rect(x + 5 * pixel_size, y - 2 * pixel_size, pixel_size as u32, pixel_size as u32, r, g, b);
        } else if is_handakuten {
            // Tiny Handakuten circle overlay
            runtime_draw_rect(x + 4 * pixel_size, y - 2 * pixel_size, 2 * pixel_size as u32, 2 * pixel_size as u32, r, g, b);
        }
        x += 6 * pixel_size;
    }
}

// Grade converter
fn get_grade(val: u32) -> (String, (u8, u8, u8)) {
    if val >= 95 {
        ("S".to_string(), COLOR_GOLD)
    } else if val >= 80 {
        ("A".to_string(), COLOR_CYAN)
    } else if val >= 65 {
        ("B".to_string(), (0, 180, 255))
    } else if val >= 50 {
        ("C".to_string(), COLOR_NEON_GREEN)
    } else if val >= 40 {
        ("D".to_string(), (200, 255, 100))
    } else if val >= 30 {
        ("E".to_string(), (255, 230, 0))
    } else if val >= 20 {
        ("F".to_string(), COLOR_ORANGE)
    } else {
        ("G".to_string(), COLOR_RED)
    }
}

// ==========================================
// 🎒 CORE GAME MODEL & STRUCTS
// ==========================================
#[derive(Clone, Debug)]
struct Player {
    name: String,
    hp: i32,
    velocity: u32,   // 球速 (100 - 165 km/h)
    breaking: u32,   // 変化球 (0 - 5 level)
    contact: u32,    // ミート (10 - 100)
    power: u32,      // パワー (10 - 100)
    speed: u32,      // 走力 (10 - 100)
    fielding: u32,   // 守備力 (10 - 100)
    role_desc: String,
    growth_type: u32, // 0: Ace, 1: Slugger, 2: Speedster, 3: Glove Wizard, 4: Balanced
}

impl Player {
    fn new(name: &str, role: &str, g_type: u32, rng: &mut SimpleRng) -> Self {
        Player {
            name: name.to_string(),
            hp: 100,
            velocity: rng.next_range(115, 130),
            breaking: rng.next_range(0, 2),
            contact: rng.next_range(20, 45),
            power: rng.next_range(20, 50),
            speed: rng.next_range(20, 45),
            fielding: rng.next_range(20, 45),
            role_desc: role.to_string(),
            growth_type: g_type,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Screen {
    Title,
    WelcomePopup,
    MainHub,
    TrainingMenu,
    RosterList,
    PlayerDetail,
    TrainingAnimation,
    MatchIntro,
    MatchAutoplay,
    MatchResult,
    EndingDraft,
}

// Scoreboard match results holder
struct MatchScoreboard {
    saru_runs: Vec<u32>,
    rival_runs: Vec<u32>,
    saru_total: u32,
    rival_total: u32,
    logs: Vec<String>,
}

// ==========================================
// 🎨 RETRO ILLUSTRATION DRAWERS
// ==========================================
fn draw_large_baseball(x: i32, y: i32) {
    let color_white = (240, 240, 245);
    let color_red = (255, 60, 60);
    let color_shadow = (180, 185, 195);
    for r in -40..40 {
        let span = ((40.0 * 40.0 - r as f32 * r as f32).sqrt()) as i32;
        if span > 0 {
            runtime_draw_rect(x + 40 - span + 6, y + 40 + r + 6, (span * 2) as u32, 1, 15, 18, 25);
            runtime_draw_rect(x + 40 - span, y + 40 + r, (span * 2) as u32, 1, color_white.0, color_white.1, color_white.2);
            let shadow_start = span - 8;
            if shadow_start > 0 {
                runtime_draw_rect(x + 40 + shadow_start, y + 40 + r, 8, 1, color_shadow.0, color_shadow.1, color_shadow.2);
            }
        }
    }
    for r in -32..32 {
        let offset = (r * r) / 45;
        runtime_draw_rect(x + 25 + offset, y + 40 + r, 3, 2, color_red.0, color_red.1, color_red.2);
        runtime_draw_rect(x + 55 - offset, y + 40 + r, 3, 2, color_red.0, color_red.1, color_red.2);
    }
}

fn draw_large_bat(x: i32, y: i32) {
    let color_wood = (222, 158, 88);
    let color_wood_dark = (165, 110, 50);
    let color_grip = (245, 245, 245);
    let color_plate = (230, 235, 240);
    let color_plate_border = (120, 125, 135);

    for r in 0..50 {
        let width = if r < 25 { 60 } else { (60.0 - (r - 25) as f32 * 2.4) as i32 };
        let start_x = if r < 25 { 10 } else { 10 + ((r - 25) as f32 * 1.2) as i32 };
        runtime_draw_rect(x + start_x + 10, y + 35 + r, width as u32, 1, color_plate.0, color_plate.1, color_plate.2);
        runtime_draw_rect(x + start_x + 10, y + 35 + r, 2, 1, color_plate_border.0, color_plate_border.1, color_plate_border.2);
        runtime_draw_rect(x + start_x + 10 + width - 2, y + 35 + r, 2, 1, color_plate_border.0, color_plate_border.1, color_plate_border.2);
    }

    for i in 0..45 {
        let bx = x + i * 2;
        let by = y + 85 - i * 2;
        let thickness = if i < 12 { 5 } else if i < 28 { 8 } else { 12 };
        let color = if i < 10 { color_grip } else if i > 38 { color_wood_dark } else { color_wood };
        runtime_draw_rect(bx + 4, by + 4, thickness, thickness, 15, 18, 25);
        runtime_draw_rect(bx, by, thickness, thickness, color.0, color.1, color.2);
    }
}

fn draw_card_panel(x: i32, y: i32, w: u32, h: u32, border_color: (u8, u8, u8)) {
    runtime_draw_rect(x + 6, y + 6, w, h, 5, 8, 15);
    runtime_draw_rect(x, y, w, h, COLOR_CARD.0, COLOR_CARD.1, COLOR_CARD.2);
    runtime_draw_rect(x, y, w, 2, border_color.0, border_color.1, border_color.2);
    runtime_draw_rect(x, y + h as i32 - 2, w, 2, border_color.0, border_color.1, border_color.2);
    runtime_draw_rect(x, y, 2, h, border_color.0, border_color.1, border_color.2);
    runtime_draw_rect(x + w as i32 - 2, y, 2, h, border_color.0, border_color.1, border_color.2);
}

// ==========================================
// 🎮 MAIN GAME IMPLEMENTATION
// ==========================================
fn main() {
    runtime_init();

    // 🎲 Seed Randomness from System Time
    use std::time::SystemTime;
    let seed = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_nanos() as u64;
    let mut rng = SimpleRng::new(seed);

    // 🏆 Game States
    let mut screen = Screen::Title;
    let mut week = 1;
    let max_weeks = 12;
    let mut selected_index = 0;
    let mut active_player_idx = 0;
    
    // Multi-language defaults to Japanese (Katakana Famicom-style / Kanji)
    let mut lang_japanese = true;

    // Shake & Flash Effects
    let mut shake_x = 0;
    let mut shake_y = 0;
    let mut shake_timer = 0.0f32;
    let mut flash_timer = 0.0f32;

    // ⚾ Roster of 9 Players
    let mut roster = vec![
        Player::new("タナカ", "ACE PITCHER", 0, &mut rng),
        Player::new("ヤマダ", "CLEANUP BATTER", 1, &mut rng),
        Player::new("サトウ", "CAPTAIN RUNNER", 2, &mut rng),
        Player::new("スズキ", "SPEED STAR", 2, &mut rng),
        Player::new("タカハシ", "GLOVE WIZARD", 3, &mut rng),
        Player::new("ワタナベ", "INFELDER ALLY", 4, &mut rng),
        Player::new("イトウ", "OUTFIELDER SHIELD", 4, &mut rng),
        Player::new("ナカムラ", "ROOKIE CLUTCH", 0, &mut rng),
        Player::new("コバヤシ", "ROOKIE SLUGGER", 1, &mut rng),
    ];

    // Give specific players specialty boosts
    roster[0].velocity = 138; roster[0].breaking = 2; roster[0].contact = 25;
    roster[1].power = 65; roster[1].contact = 45; roster[1].speed = 25;
    roster[3].speed = 68; roster[3].power = 20;
    roster[4].fielding = 65; roster[4].speed = 45;

    // Training Animation Data
    let mut active_drill_name = "";
    let mut floating_gains: Vec<(String, String)> = Vec::new();
    let mut animation_time = 0.0f32;

    // Match Simulation Data
    let mut active_match_title = "";
    let mut current_scoreboard = MatchScoreboard {
        saru_runs: vec![],
        rival_runs: vec![],
        saru_total: 0,
        rival_total: 0,
        logs: vec![],
    };
    let mut match_tick_timer = 0.0f32;
    let mut match_inning_step = 0; // 0..6 corresponding to 7th, 8th, 9th top/bottom

    // Keyboard trigger lock keys
    let mut up_pressed = false;
    let mut down_pressed = false;
    let mut left_pressed = false;
    let mut right_pressed = false;
    let mut action_pressed = false;
    let mut action_b_pressed = false;

    // Welcome dialogue flags
    let mut has_proclaimed = false;

    // Main Game Loop
    let is_running = true;
    while is_running {
        let dt = runtime_get_delta_time();
        animation_time += dt;

        let input = runtime_get_input();
        if input.quit {
            break;
        }

        // Handle edge-triggered input
        let trig_up = if input.up { if !up_pressed { up_pressed = true; true } else { false } } else { up_pressed = false; false };
        let trig_down = if input.down { if !down_pressed { down_pressed = true; true } else { false } } else { down_pressed = false; false };
        let _trig_left = if input.left { if !left_pressed { left_pressed = true; true } else { false } } else { left_pressed = false; false };
        let _trig_right = if input.right { if !right_pressed { right_pressed = true; true } else { false } } else { right_pressed = false; false };
        let trig_action = if input.action_a { if !action_pressed { action_pressed = true; true } else { false } } else { action_pressed = false; false };
        let trig_action_b = if input.action_b { if !action_b_pressed { action_b_pressed = true; true } else { false } } else { action_b_pressed = false; false };

        if shake_timer > 0.0 {
            shake_timer -= dt;
            let nanos = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().as_nanos();
            shake_x = ((nanos % 14) as i32 - 7) * (shake_timer * 2.0) as i32;
            shake_y = ((nanos % 8) as i32 - 4) * (shake_timer * 2.0) as i32;
        } else {
            shake_x = 0;
            shake_y = 0;
        }
        if flash_timer > 0.0 { flash_timer -= dt; }

        set_ui_language(lang_japanese);
        runtime_clear();

        // 🔄 SCREEN STATE MACHINE
        match screen {
            Screen::Title => {
                // Title Screen Render
                let px = shake_x;
                let py = shake_y;
                runtime_draw_rect(px, py, 1280, 720, COLOR_BG.0, COLOR_BG.1, COLOR_BG.2);
                runtime_draw_rect(px, py + 120, 1280, 3, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);

                let title_glow = ((animation_time * 2.5).sin() * 20.0 + 235.0) as u8;
                draw_string(px + 140, py + 180, "KOSHIEN SUCCESS", 7, title_glow, 250, 255);
                draw_string(px + 145, py + 185, "KOSHIEN SUCCESS", 7, 20, 25, 40);

                if lang_japanese {
                    draw_string(px + 280, py + 280, "甲子園 監督育成シミュレーター", 2, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 360, py + 520, "[Enter] で 監督室へ", 2, title_glow, title_glow, 255);
                    draw_string(px + 390, py + 560, "[Backspace] 言語切替", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                } else {
                    draw_string(px + 380, py + 280, "HIGH SCHOOL BASEBALL ROSTER SIMULATOR", 2, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 360, py + 520, "PRESS [ENTER] TO INITIATE OFFICE", 2, title_glow, title_glow, 255);
                    draw_string(px + 400, py + 560, "[BACKSPACE] TOGGLE BILINGUAL STATE", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                }

                draw_large_bat(px + 450, py + 340);
                draw_large_baseball(px + 690, py + 340);
                draw_string(px + 420, py + 650, "POWERED BY SARU OS RETRO GRAPHICS", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);

                // Interactions
                if trig_action_b {
                    lang_japanese = !lang_japanese;
                    shake_timer = 0.15;
                }
                if trig_action {
                    flash_timer = 0.2;
                    screen = if has_proclaimed { Screen::MainHub } else { Screen::WelcomePopup };
                }
            }

            Screen::WelcomePopup => {
                let px = shake_x;
                let py = shake_y;
                runtime_draw_rect(px, py, 1280, 720, COLOR_BG.0, COLOR_BG.1, COLOR_BG.2);
                draw_card_panel(px + 100, py + 100, 1080, 520, COLOR_GOLD);

                if lang_japanese {
                    draw_string(px + 150, py + 150, "サル高校 野球部へ ようこそ!", 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 150, py + 230, "あなたは 今日から 野球部の 監督です。", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 150, py + 270, "9人の 球員を 育成してください。", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 150, py + 310, "12週間、毎週 練習を 割り当てられます。", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 150, py + 350, "選手を 練習させると 体力を 消費します。", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 150, py + 390, "甲子園 優勝を 目指して 特訓を 開始 です!", 2, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);
                    draw_string(px + 150, py + 480, "[Enter] で チームを 確定", 2, COLOR_NEON_GREEN.0, COLOR_NEON_GREEN.1, COLOR_NEON_GREEN.2);
                } else {
                    draw_string(px + 150, py + 150, "WELCOME TO SARU HIGH BASEBALL CLUB!", 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 150, py + 230, "YOU ARE THE NEW COACH COMMENCING DUTY TODAY.", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 150, py + 270, "TRAIN A ROSTER OF 9 INDIVIDUAL RECRUITS RANDOMLY INITIALIZED.", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 150, py + 310, "CHOOSE SPECIALIZED PRACTICE DRILLS EACH WEEK TO BOOST THEIR STATS.", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 150, py + 350, "MONITOR PLAYER HP CAREFULLY TO PREVENT SEVERE DRILL INJURIES.", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 150, py + 390, "WIN AUTOPLAY INNING MATCHES TO ACQUIRE KOSHIEN TROPHIES!", 2, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);
                    draw_string(px + 150, py + 480, "PRESS [ENTER] TO COMMENCE HEADQUARTERS", 2, COLOR_NEON_GREEN.0, COLOR_NEON_GREEN.1, COLOR_NEON_GREEN.2);
                }

                if trig_action {
                    has_proclaimed = true;
                    screen = Screen::MainHub;
                    selected_index = 0;
                }
            }

            Screen::MainHub => {
                let px = shake_x;
                let py = shake_y;
                runtime_draw_rect(px, py, 1280, 720, COLOR_BG.0, COLOR_BG.1, COLOR_BG.2);

                // Sidebar navigation
                draw_card_panel(px + 40, py + 40, 360, 640, COLOR_CARD_HIGHLIGHT);
                
                if lang_japanese {
                    draw_string(px + 70, py + 70, "監督室", 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 70, py + 120, "12週間の サマーキャンプ", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                } else {
                    draw_string(px + 70, py + 70, "COACH OFFICE", 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 70, py + 120, "12-WEEK SUMMER ROUTE", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                }

                let hub_choices = if lang_japanese {
                    vec!["1. 練習 (PRACTICE)", "2. 選手一覧 (ROSTER)", "3. チーム休養 (REST)", "4. 次の週へ (ADVANCE)"]
                } else {
                    vec!["1. DRILLS PRACTICE", "2. ROSTER LIST", "3. RECOVER ALL HP", "4. NEXT SCHEDULE WEEK"]
                };

                for i in 0..4 {
                    let btn_y = py + 200 + i as i32 * 100;
                    let is_sel = selected_index == i;
                    let border_c = if is_sel { COLOR_CYAN } else { COLOR_CARD };
                    draw_card_panel(px + 60, btn_y, 320, 80, border_c);
                    let label_glow = if is_sel { COLOR_TEXT_WHITE } else { COLOR_TEXT_MUTED };
                    draw_string(px + 80, btn_y + 25, hub_choices[i], 2, label_glow.0, label_glow.1, label_glow.2);
                }

                // Right detailed schedule and overview
                draw_card_panel(px + 430, py + 40, 810, 640, COLOR_CARD_HIGHLIGHT);
                
                if lang_japanese {
                    draw_string(px + 470, py + 80, "サル高校 野球部 運営表", 3, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);
                    draw_string(px + 470, py + 160, &format!("スケジュール: 第{}週 / {}", week, max_weeks), 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                } else {
                    draw_string(px + 470, py + 80, "SARU HIGH CLUB MANAGEMENT DESK", 3, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);
                    draw_string(px + 470, py + 160, &format!("SCHEDULE: WEEK {} OF {}", week, max_weeks), 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                }

                // Next Event Notification
                let sched_desc = match week {
                    1..=3 => if lang_japanese { "予告: 第4週に プレシーズン試合!" } else { "UPCOMING: WEEK 4 PRACTICE EXHIBITION GAME!" },
                    4 => if lang_japanese { "試合日: 第4週 プレシーズン試合!" } else { "MATCH DAY: WEEK 4 PRACTICE EXHIBITION MATCH!" },
                    5..=7 => if lang_japanese { "予告: 第8週に 地方予選 開始!" } else { "UPCOMING: WEEK 8 REGIONAL CHAMPIONSHIP QUALIFIERS!" },
                    8 => if lang_japanese { "試合日: 第8週 地方予選 本番!" } else { "MATCH DAY: WEEK 8 REGIONAL CHAMPIONSHIP QUALIFIERS!" },
                    9..=11 => if lang_japanese { "予告: 第12週に 甲子園 決勝!" } else { "UPCOMING: WEEK 12 KOSHIEN GRAND FINALS!" },
                    12 => if lang_japanese { "決勝日: 第12週 甲子園 優勝決戦!" } else { "CHAMPIONSHIP DAY: WEEK 12 KOSHIEN GRAND FINALS!" },
                    _ => if lang_japanese { "記録: キャンプ 終了!" } else { "RECORD: ALL TOURNAMENT DRILLS FINISHED!" },
                };
                draw_card_panel(px + 470, py + 220, 730, 90, COLOR_GOLD);
                draw_string(px + 490, py + 250, sched_desc, 2, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);

                // Quick overview of Roster HP average
                let avg_hp: i32 = roster.iter().map(|p| p.hp).sum::<i32>() / roster.len() as i32;
                let hp_color = if avg_hp > 60 { COLOR_NEON_GREEN } else if avg_hp > 30 { COLOR_ORANGE } else { COLOR_RED };
                
                if lang_japanese {
                    draw_string(px + 470, py + 360, "チーム コンディション ガイド:", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 470, py + 410, &format!("チーム平均 体力: {} HP", avg_hp), 2, hp_color.0, hp_color.1, hp_color.2);
                    draw_string(px + 470, py + 460, "体力が 低いと 怪我の ペナルティが あります。", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 470, py + 520, "[Enter] で 項目を 選択", 2, COLOR_NEON_GREEN.0, COLOR_NEON_GREEN.1, COLOR_NEON_GREEN.2);
                    draw_string(px + 470, py + 560, "[Backspace] で タイトルへ", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                } else {
                    draw_string(px + 470, py + 360, "TEAM HEALTH MONITOR OVERVIEW:", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 470, py + 410, &format!("ROSTER MEAN PHYSICAL HP: {} / 100", avg_hp), 2, hp_color.0, hp_color.1, hp_color.2);
                    draw_string(px + 470, py + 460, "LOW HP CAUSES COLLAPSE AND REDUCES STAT GROWTH YIELDS.", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 470, py + 520, "PRESS [ENTER] TO SELECT OPTION", 2, COLOR_NEON_GREEN.0, COLOR_NEON_GREEN.1, COLOR_NEON_GREEN.2);
                    draw_string(px + 470, py + 560, "PRESS [BACKSPACE] FOR TITLE SCREEN", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                }

                // Controls
                if trig_up {
                    selected_index = if selected_index == 0 { 3 } else { selected_index - 1 };
                }
                if trig_down {
                    selected_index = if selected_index == 3 { 0 } else { selected_index + 1 };
                }
                if trig_action {
                    flash_timer = 0.15;
                    match selected_index {
                        0 => { screen = Screen::TrainingMenu; selected_index = 0; }
                        1 => { screen = Screen::RosterList; selected_index = 0; }
                        2 => {
                            // Team Retreat (Recover all HP)
                            for p in &mut roster {
                                p.hp = (p.hp + 40).min(100);
                            }
                            shake_timer = 0.2;
                            floating_gains = roster.iter().map(|p| (p.name.clone(), "+40 HP".to_string())).collect();
                            active_drill_name = if lang_japanese { "チーム 簡易休養" } else { "TEAM COOLDOWN RETREAT" };
                            animation_time = 0.0;
                            screen = Screen::TrainingAnimation;
                        }
                        3 => {
                            // Advance schedule week
                            if week == 4 || week == 8 || week == 12 {
                                // Match Day!
                                active_match_title = match week {
                                    4 => if lang_japanese { "第4週: プレシーズン (VS 東京)" } else { "WEEK 4: PRE-SEASON GAME (VS TOKYO)" },
                                    8 => if lang_japanese { "第8週: 地方予選 (VS 大阪)" } else { "WEEK 8: REGIONAL QUALIFIER (VS OSAKA)" },
                                    _ => if lang_japanese { "決勝: 甲子園 グランドファイナル" } else { "CHAMPIONSHIP: KOSHIEN GRAND FINALS" },
                                };
                                match_tick_timer = 0.0;
                                match_inning_step = 0;
                                
                                // Simulate inning narrative scoreboard
                                let saru_bat = roster.iter().map(|p| p.contact + p.power).sum::<u32>() / roster.len() as u32;
                                let saru_pit = roster.iter().map(|p| p.velocity / 3 + p.breaking * 15 + p.fielding).sum::<u32>() / roster.len() as u32;

                                let mut rival_bat = 30 + week * 5;
                                let mut rival_pit = 30 + week * 5;
                                if week == 12 {
                                    rival_bat += 15; rival_pit += 15; // Koshien finals hard
                                }

                                let mut s_runs = vec![];
                                let mut r_runs = vec![];
                                let mut logs = vec![];
                                
                                logs.push(if lang_japanese { format!("{} 試合 開始!", active_match_title) } else { format!("{} INITIATED!", active_match_title) });

                                // Inning simulation
                                for i in 7..=9 {
                                    // Top half (Rival attack)
                                    let r_chance = rng.next_range(0, 100);
                                    let r_score = if r_chance + rival_bat > saru_pit + 50 {
                                        rng.next_range(1, 4)
                                    } else {
                                        0
                                    };
                                    r_runs.push(r_score);
                                    if r_score > 0 {
                                        logs.push(if lang_japanese { format!("{}回 表: ライバル校の 攻撃。 {}点 先制!", i, r_score) } else { format!("Top of {}th: Rival scores {} runs!", i, r_score) });
                                    } else {
                                        logs.push(if lang_japanese { format!("{}回 表: エース 田中の 制球! 連続三振!", i) } else { format!("Top of {}th: Tanaka shuts down the inning!", i) });
                                    }

                                    // Bottom half (Saru attack)
                                    let s_chance = rng.next_range(0, 100);
                                    let s_score = if s_chance + saru_bat > rival_pit + 45 {
                                        rng.next_range(1, 4)
                                    } else {
                                        0
                                    };
                                    s_runs.push(s_score);
                                    if s_score > 0 {
                                        logs.push(if lang_japanese { format!("{}回 裏: 山田の 豪快な ホームラン! {}点 返す!", i, s_score) } else { format!("Bottom of {}th: Yamada hits deep, scoring {} runs!", i, s_score) });
                                    } else {
                                        logs.push(if lang_japanese { format!("{}回 裏: 残念! 凡打で チェンジ。", i) } else { format!("Bottom of {}th: Out on first base.", i) });
                                    }
                                }

                                let s_total: u32 = s_runs.iter().sum();
                                let r_total: u32 = r_runs.iter().sum();

                                // Prevent exact ties inside finals
                                if s_total == r_total {
                                    if rng.next_range(0, 2) == 0 {
                                        s_runs[2] += 1;
                                    } else {
                                        r_runs[2] += 1;
                                    }
                                }

                                let s_total_final: u32 = s_runs.iter().sum();
                                let r_total_final: u32 = r_runs.iter().sum();

                                current_scoreboard = MatchScoreboard {
                                    saru_runs: s_runs,
                                    rival_runs: r_runs,
                                    saru_total: s_total_final,
                                    rival_total: r_total_final,
                                    logs,
                                };
                                screen = Screen::MatchIntro;
                            } else {
                                // Standard week advance
                                week += 1;
                                shake_timer = 0.25;
                            }
                        }
                        _ => {}
                    }
                }
                if trig_action_b {
                    screen = Screen::Title;
                }
            }

            Screen::TrainingMenu => {
                let px = shake_x;
                let py = shake_y;
                runtime_draw_rect(px, py, 1280, 720, COLOR_BG.0, COLOR_BG.1, COLOR_BG.2);

                draw_card_panel(px + 40, py + 40, 360, 640, COLOR_CARD_HIGHLIGHT);
                
                if lang_japanese {
                    draw_string(px + 70, py + 70, "特訓 項目", 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 70, py + 120, "選手を 強化 せよ", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 70, py + 620, "[Backspace] 戻る", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                } else {
                    draw_string(px + 70, py + 70, "PRACTICE LIST", 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 70, py + 120, "CHOOSE TEAM ROUTINE", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 70, py + 620, "[BACKSPACE] RETURN OFFICE", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                }

                let drill_names = if lang_japanese {
                    vec![
                        "1. 球速 練習 (VEL)",
                        "2. 変化球 練習 (BRK)",
                        "3. ミート 練習 (CON)",
                        "4. パワー 練習 (POW)",
                        "5. 走力 練習 (SPD)",
                        "6. 守備 練習 (FLD)"
                    ]
                } else {
                    vec![
                        "1. VELOCITY TRAINING (VEL)",
                        "2. BREAKING BALL DRILLS (BRK)",
                        "3. CONTACT CORRELATION (CON)",
                        "4. BAT WEIGHT SLUGGING (POW)",
                        "5. SPEED LAP SPRINTING (SPD)",
                        "6. GLOVE WORK & FIELDING (FLD)"
                    ]
                };

                for i in 0..6 {
                    let is_sel = selected_index == i;
                    let btn_y = py + 180 + i as i32 * 80;
                    let border_c = if is_sel { COLOR_CYAN } else { COLOR_CARD };
                    draw_card_panel(px + 60, btn_y, 320, 65, border_c);
                    let label_glow = if is_sel { COLOR_TEXT_WHITE } else { COLOR_TEXT_MUTED };
                    draw_string(px + 70, btn_y + 20, drill_names[i], 2, label_glow.0, label_glow.1, label_glow.2);
                }

                // Drill details card
                draw_card_panel(px + 430, py + 40, 810, 640, COLOR_CARD_HIGHLIGHT);
                let effect_desc = match selected_index {
                    0 => if lang_japanese { ("球速を 上げる (VELOCITY UP)", "投手の 球速が 大きく 上昇。") } else { ("IMPROVES VELOCITY SPEEDS", "BOOSTS VELOCITY KM/H FOR PITCHERS DRAMATICALLY.") },
                    1 => if lang_japanese { ("変化球を 上げる (BREAKING UP)", "投手の 変化球 レベルを 強化 します。") } else { ("IMPROVES DECEPTIVE BREAKING", "RAISES BREAKING BALL LEVEL COEFFICIENT.") },
                    2 => if lang_japanese { ("ミートを 強化 (CONTACT UP)", "バッターの ミート 精度を 強化 します。") } else { ("IMPROVES CONTACT DIAMETERS", "INCREASES SWING CONTACT PRECISION GRADES.") },
                    3 => if lang_japanese { ("パワーを 強化 (POWER UP)", "バッターの ヒット パワーを 強化 します。") } else { ("IMPROVES BAT SLUGGING POWER", "ENHANCES HOMERUN LAUNCH VELOCITY METRIC.") },
                    4 => if lang_japanese { ("走力を 上げる (SPEED UP)", "選手の 走力を 上げる。盗塁も 上昇。") } else { ("IMPROVES SPEED & LAP SPRINT", "INCREASES RUNNING AGILITY ON THE FIELD BASES.") },
                    _ => if lang_japanese { ("守備を 強化 (FIELDING UP)", "チームの 守備統率を 最適化 します。") } else { ("IMPROVES GLOVE FIELDING GRADES", "MINIMIZES RIVAL EXTRA BASE ADVANCES.") },
                };

                draw_string(px + 470, py + 80, effect_desc.0, 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                draw_string(px + 470, py + 140, effect_desc.1, 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);

                if lang_japanese {
                    draw_string(px + 470, py + 220, "特訓の 効果:", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 470, py + 270, "・体力 消耗 (HP COST): -15 HP (チーム全員)", 2, COLOR_RED.0, COLOR_RED.1, COLOR_RED.2);
                    draw_string(px + 470, py + 320, "・選手特性 により 上昇幅が 変化 します。", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                } else {
                    draw_string(px + 470, py + 220, "DRILL STATISTICS YIELD OVERVIEW:", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 470, py + 270, "- HP CONSUMED: -15 HP (APPLIES TO WHOLE ROSTER)", 2, COLOR_RED.0, COLOR_RED.1, COLOR_RED.2);
                    draw_string(px + 470, py + 320, "- EACH RECRUIT GROWS ACCORDING TO INDIVIDUAL SPEC SPECIALTY.", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                }

                // Check HP and warning
                let has_fatal_hp = roster.iter().any(|p| p.hp <= 15);
                if has_fatal_hp {
                    draw_card_panel(px + 470, py + 400, 730, 90, COLOR_RED);
                    if lang_japanese {
                        draw_string(px + 490, py + 430, "警告: 体力が 低すぎる 選手が います! 怪我の 危険!", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    } else {
                        draw_string(px + 490, py + 430, "WARNING: INSUFFICIENT PLAYER HP DETECTED. RISK OF SEVERE INJURY!", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    }
                }

                if lang_japanese {
                    draw_string(px + 470, py + 520, "[Enter] で 特訓を 実行", 2, COLOR_NEON_GREEN.0, COLOR_NEON_GREEN.1, COLOR_NEON_GREEN.2);
                } else {
                    draw_string(px + 470, py + 520, "PRESS [ENTER] TO EXECUTE TRAINING ROUTINE", 2, COLOR_NEON_GREEN.0, COLOR_NEON_GREEN.1, COLOR_NEON_GREEN.2);
                }

                if trig_up {
                    selected_index = if selected_index == 0 { 5 } else { selected_index - 1 };
                }
                if trig_down {
                    selected_index = if selected_index == 5 { 0 } else { selected_index + 1 };
                }
                if trig_action_b {
                    screen = Screen::MainHub;
                    selected_index = 0;
                }
                if trig_action {
                    // Execute training drill!
                    floating_gains.clear();
                    
                    for p in &mut roster {
                        let did_collapse = p.hp <= 15 && rng.next_range(0, 10) < 3; // Collapse penalty
                        p.hp = (p.hp - 15).max(0);

                        if did_collapse {
                            // Collapse
                            p.hp = 0;
                            floating_gains.push((p.name.clone(), if lang_japanese { "怪我! DOWN".to_string() } else { "COLLAPSED!".to_string() }));
                            continue;
                        }

                        // Apply random stat yields based on Specialty
                        match selected_index {
                            0 => {
                                // Velocity practice
                                let gain = if p.growth_type == 0 { rng.next_range(2, 5) } else { rng.next_range(0, 2) };
                                p.velocity = (p.velocity + gain).min(165);
                                if gain > 0 {
                                    floating_gains.push((p.name.clone(), format!("+{} KM/H", gain)));
                                }
                            }
                            1 => {
                                // Breaking
                                let gain = if p.growth_type == 0 { rng.next_range(1, 2) } else { 0 };
                                p.breaking = (p.breaking + gain).min(5);
                                if gain > 0 {
                                    floating_gains.push((p.name.clone(), format!("BRK +{}", gain)));
                                }
                            }
                            2 => {
                                // Contact
                                let gain = if p.growth_type == 1 || p.growth_type == 4 { rng.next_range(3, 7) } else { rng.next_range(1, 4) };
                                p.contact = (p.contact + gain).min(100);
                                floating_gains.push((p.name.clone(), format!("CON +{}", gain)));
                            }
                            3 => {
                                // Power
                                let gain = if p.growth_type == 1 { rng.next_range(4, 9) } else { rng.next_range(1, 4) };
                                p.power = (p.power + gain).min(100);
                                floating_gains.push((p.name.clone(), format!("POW +{}", gain)));
                            }
                            4 => {
                                // Speed
                                let gain = if p.growth_type == 2 { rng.next_range(4, 9) } else { rng.next_range(1, 4) };
                                p.speed = (p.speed + gain).min(100);
                                floating_gains.push((p.name.clone(), format!("SPD +{}", gain)));
                            }
                            _ => {
                                // Fielding
                                let gain = if p.growth_type == 3 { rng.next_range(4, 9) } else { rng.next_range(1, 4) };
                                p.fielding = (p.fielding + gain).min(100);
                                floating_gains.push((p.name.clone(), format!("FLD +{}", gain)));
                            }
                        }
                    }

                    active_drill_name = drill_names[selected_index];
                    animation_time = 0.0;
                    screen = Screen::TrainingAnimation;
                    flash_timer = 0.2;
                }
            }

            Screen::TrainingAnimation => {
                // Beautiful retro base running animation + floating logs
                let px = shake_x;
                let py = shake_y;
                runtime_draw_rect(px, py, 1280, 720, COLOR_BG.0, COLOR_BG.1, COLOR_BG.2);

                // Draw base diamond lines
                let cx = px + 640;
                let cy = py + 260;

                // Home
                runtime_draw_rect(cx - 15, cy + 120, 30, 20, 240, 240, 245);
                // 1st
                runtime_draw_rect(cx + 120 - 10, cy - 10, 20, 20, 240, 240, 245);
                // 2nd
                runtime_draw_rect(cx - 10, cy - 120, 20, 20, 240, 240, 245);
                // 3rd
                runtime_draw_rect(cx - 120 - 10, cy - 10, 20, 20, 240, 240, 245);

                // Run phase runner
                let phase = (animation_time * 5.0) % 4.0;
                let (rx, ry) = if phase < 1.0 {
                    // Home to 1st
                    let t = phase;
                    (cx - 10 + (t * 130.0) as i32, cy + 120 - (t * 130.0) as i32)
                } else if phase < 2.0 {
                    // 1st to 2nd
                    let t = phase - 1.0;
                    (cx + 120 - (t * 130.0) as i32, cy - 10 - (t * 110.0) as i32)
                } else if phase < 3.0 {
                    // 2nd to 3rd
                    let t = phase - 2.0;
                    (cx - (t * 120.0) as i32, cy - 120 + (t * 110.0) as i32)
                } else {
                    // 3rd to Home
                    let t = phase - 3.0;
                    (cx - 120 + (t * 110.0) as i32, cy - 10 + (t * 130.0) as i32)
                };
                
                // Draw running pixel player (cyan jersey)
                runtime_draw_rect(rx - 8, ry - 16, 16, 24, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);
                runtime_draw_rect(rx - 4, ry - 24, 8, 8, 255, 220, 180); // Skin head

                // Pitcher mound ball throwing animation
                let ball_t = (animation_time * 3.0) % 1.0;
                let bx = cx + (1.0 - ball_t) as i32 * 0;
                let by = cy - 20 + (ball_t * 120.0) as i32;
                runtime_draw_rect(bx - 3, by - 3, 6, 6, 255, 255, 255); // baseball ball

                // Floating yields card panel
                draw_card_panel(px + 40, py + 460, 1200, 220, COLOR_GOLD);
                if lang_japanese {
                    draw_string(px + 80, py + 490, &format!("特訓 実行: {}", active_drill_name), 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                } else {
                    draw_string(px + 80, py + 490, &format!("DRILL SUCCESSFUL: {}", active_drill_name), 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                }

                // Display dynamic stat gains floating up
                for i in 0..floating_gains.len().min(5) {
                    let (p_name, g_val) = &floating_gains[i];
                    let slide_offset = ((animation_time * 60.0) as i32) % 40;
                    let display_x = px + 80 + i as i32 * 230;
                    draw_string(display_x, py + 580 - slide_offset, &format!("{} {}", p_name, g_val), 2, COLOR_NEON_GREEN.0, COLOR_NEON_GREEN.1, COLOR_NEON_GREEN.2);
                }

                if animation_time >= 2.0 || trig_action {
                    // Drill completes, advance week automatically
                    if week < max_weeks {
                        week += 1;
                        screen = Screen::MainHub;
                        selected_index = 0;
                    } else {
                        // Koshien completed, trigger EndingDraft ceremony
                        screen = Screen::EndingDraft;
                        selected_index = 0;
                    }
                }
            }

            Screen::RosterList => {
                let px = shake_x;
                let py = shake_y;
                runtime_draw_rect(px, py, 1280, 720, COLOR_BG.0, COLOR_BG.1, COLOR_BG.2);

                draw_card_panel(px + 40, py + 40, 480, 640, COLOR_CARD_HIGHLIGHT);
                
                if lang_japanese {
                    draw_string(px + 70, py + 70, "部員 名簿", 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 70, py + 120, "9人の 選手 たち", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 70, py + 620, "[Backspace] 戻る", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                } else {
                    draw_string(px + 70, py + 70, "ROSTER ROSTER", 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 70, py + 120, "SQUAD LIST OF 9 MEMBERS", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 70, py + 620, "[BACKSPACE] RETURN OFFICE", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                }

                for i in 0..9 {
                    let is_sel = selected_index == i;
                    let list_y = py + 160 + i as i32 * 50;
                    let text_c = if is_sel { COLOR_TEXT_WHITE } else { COLOR_TEXT_MUTED };
                    if is_sel {
                        runtime_draw_rect(px + 60, list_y - 2, 440, 36, COLOR_CARD_HIGHLIGHT.0, COLOR_CARD_HIGHLIGHT.1, COLOR_CARD_HIGHLIGHT.2);
                    }
                    draw_string(px + 80, list_y + 8, &format!("{}  {}", i + 1, roster[i].name), 2, text_c.0, text_c.1, text_c.2);
                    
                    // Show short specialty tags
                    let spec_tag = match roster[i].growth_type {
                        0 => "PITCHER",
                        1 => "SLUGGER",
                        2 => "SPEED",
                        3 => "DEFENSE",
                        _ => "BALANCE",
                    };
                    draw_string(px + 360, list_y + 8, spec_tag, 2, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);
                }

                // Detailed Preview Card
                draw_card_panel(px + 550, py + 40, 690, 640, COLOR_CARD_HIGHLIGHT);
                let p = &roster[selected_index];
                
                if lang_japanese {
                    draw_string(px + 590, py + 80, &format!("プレビュー: {}", p.name), 3, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);
                    draw_string(px + 590, py + 140, &format!("特性: {}", p.role_desc), 2, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    
                    // Draw HP bar
                    draw_string(px + 590, py + 220, "体力 (HP):", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                } else {
                    draw_string(px + 590, py + 80, &format!("PREVIEW: {}", p.name), 3, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);
                    draw_string(px + 590, py + 140, &format!("SPECIALTY: {}", p.role_desc), 2, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    
                    // Draw HP bar
                    draw_string(px + 590, py + 220, "ENERGY VITALITY (HP):", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                }

                let hp_bar_w = (p.hp * 3) as u32;
                let hp_c = if p.hp > 60 { COLOR_NEON_GREEN } else if p.hp > 30 { COLOR_ORANGE } else { COLOR_RED };
                runtime_draw_rect(px + 590, py + 260, 300, 25, 30, 40, 60); // Background bar
                runtime_draw_rect(px + 590, py + 260, hp_bar_w, 25, hp_c.0, hp_c.1, hp_c.2); // Filled HP bar
                draw_string(px + 910, py + 265, &format!("{} / 100", p.hp), 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);

                // Quick stats preview
                if lang_japanese {
                    draw_string(px + 590, py + 330, &format!("・球速 (VEL):  {} KM/H", p.velocity), 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 590, py + 380, &format!("・変化球 (BRK): LEVEL {}", p.breaking), 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 590, py + 430, &format!("・ミート (CON):      {}", p.contact), 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 590, py + 480, &format!("・パワー (POW):      {}", p.power), 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);

                    draw_string(px + 590, py + 560, "[Enter] で 詳細カードを 開く", 2, COLOR_NEON_GREEN.0, COLOR_NEON_GREEN.1, COLOR_NEON_GREEN.2);
                } else {
                    draw_string(px + 590, py + 330, &format!("- VELOCITY (VEL):    {} KM/H", p.velocity), 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 590, py + 380, &format!("- BREAKING (BRK):    LEVEL {}", p.breaking), 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 590, py + 430, &format!("- CONTACT (CON):     {}", p.contact), 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 590, py + 480, &format!("- POWER (POW):       {}", p.power), 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);

                    draw_string(px + 590, py + 560, "PRESS [ENTER] TO OPEN DETAILED CARD", 2, COLOR_NEON_GREEN.0, COLOR_NEON_GREEN.1, COLOR_NEON_GREEN.2);
                }

                if trig_up {
                    selected_index = if selected_index == 0 { 8 } else { selected_index - 1 };
                }
                if trig_down {
                    selected_index = if selected_index == 8 { 0 } else { selected_index + 1 };
                }
                if trig_action_b {
                    screen = Screen::MainHub;
                    selected_index = 1;
                }
                if trig_action {
                    active_player_idx = selected_index;
                    screen = Screen::PlayerDetail;
                }
            }

            Screen::PlayerDetail => {
                let px = shake_x;
                let py = shake_y;
                runtime_draw_rect(px, py, 1280, 720, COLOR_BG.0, COLOR_BG.1, COLOR_BG.2);

                let p = &roster[active_player_idx];
                draw_card_panel(px + 100, py + 60, 1080, 600, COLOR_GOLD);

                if lang_japanese {
                    draw_string(px + 150, py + 100, &format!("選手 特性 詳細: {}", p.name), 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 150, py + 150, &format!("特技: {}", p.role_desc), 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 150, py + 600, "[Backspace] 名簿 戻る", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                } else {
                    draw_string(px + 150, py + 100, &format!("DETAILED PLAYER STATS CARD: {}", p.name), 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 150, py + 150, &format!("ROLE SUMMARY: {}", p.role_desc), 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 150, py + 600, "[BACKSPACE] RETURN TO ROSTER LIST", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                }

                // Render all 6 stats with visual letter grades and progress bars
                let stats = [
                    (if lang_japanese { "球速 (VEL)" } else { "VELOCITY (VEL)" }, p.velocity as u32, 165, "KM/H"),
                    (if lang_japanese { "変化球 (BRK)" } else { "BREAKING (BRK)" }, p.breaking * 20, 100, "LVL"),
                    (if lang_japanese { "ミート (CON)" } else { "CONTACT (CON)" }, p.contact, 100, "CON"),
                    (if lang_japanese { "パワー (POW)" } else { "POWER (POW)" }, p.power, 100, "POW"),
                    (if lang_japanese { "走力 (SPD)" } else { "SPEED (SPD)" }, p.speed, 100, "SPD"),
                    (if lang_japanese { "守備 (FLD)" } else { "FIELDING (FLD)" }, p.fielding, 100, "FLD"),
                ];

                for i in 0..6 {
                    let (label, val, max_val, suffix) = stats[i];
                    let card_y = py + 200 + i as i32 * 60;
                    
                    draw_string(px + 150, card_y + 10, label, 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    
                    // Draw visual bar
                    let bar_fill_w = ((val as f32 / max_val as f32) * 300.0) as u32;
                    runtime_draw_rect(px + 450, card_y + 8, 300, 20, 30, 45, 75); // Background
                    runtime_draw_rect(px + 450, card_y + 8, bar_fill_w, 20, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2); // Fill
                    
                    // Grades letter display
                    let scaled_val = if i == 0 { (p.velocity - 100) * 100 / 65 } else if i == 1 { p.breaking * 20 } else { val };
                    let (grade_str, grade_c) = get_grade(scaled_val);
                    draw_string(px + 780, card_y + 10, &grade_str, 2, grade_c.0, grade_c.1, grade_c.2);

                    let display_val = if i == 0 { format!("{} {}", val, suffix) } else if i == 1 { format!("LEVEL {} {}", p.breaking, suffix) } else { format!("{} {}", val, suffix) };
                    draw_string(px + 830, card_y + 10, &display_val, 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                }

                if trig_action_b {
                    screen = Screen::RosterList;
                }
            }

            Screen::MatchIntro => {
                let px = shake_x;
                let py = shake_y;
                runtime_draw_rect(px, py, 1280, 720, COLOR_BG.0, COLOR_BG.1, COLOR_BG.2);

                draw_card_panel(px + 100, py + 100, 1080, 520, COLOR_RED);
                
                if lang_japanese {
                    draw_string(px + 150, py + 150, "! 試合日 到来 !", 4, COLOR_RED.0, COLOR_RED.1, COLOR_RED.2);
                    draw_string(px + 150, py + 240, &format!("対戦相手 : {}", active_match_title), 3, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 150, py + 320, "監督として、ベンチから 選手たちを 見守りましょう!", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 150, py + 360, "試合は 自動 シミュレーションで 決着 されます。", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 150, py + 450, "[Enter] で プレイボール!", 2, COLOR_NEON_GREEN.0, COLOR_NEON_GREEN.1, COLOR_NEON_GREEN.2);
                } else {
                    draw_string(px + 150, py + 150, "! MATCH DAY COMMENCING !", 4, COLOR_RED.0, COLOR_RED.1, COLOR_RED.2);
                    draw_string(px + 150, py + 240, &format!("MATCHUP: {}", active_match_title), 3, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    draw_string(px + 150, py + 320, "SUPPORT YOUR ROSTER PLAYERS AS MANAGER FROM THE BENCH!", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 150, py + 360, "INNING PROGRESSIONS WILL BE AUTOMATICALLY AUTO-PLAYED.", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    draw_string(px + 150, py + 450, "PRESS [ENTER] FOR PLAYBALL!", 2, COLOR_NEON_GREEN.0, COLOR_NEON_GREEN.1, COLOR_NEON_GREEN.2);
                }

                if trig_action {
                    flash_timer = 0.25;
                    screen = Screen::MatchAutoplay;
                    match_tick_timer = 0.0;
                    match_inning_step = 0;
                }
            }

            Screen::MatchAutoplay => {
                let px = shake_x;
                let py = shake_y;
                runtime_draw_rect(px, py, 1280, 720, COLOR_BG.0, COLOR_BG.1, COLOR_BG.2);

                // Stadium Inning Scoreboard panel
                draw_card_panel(px + 40, py + 40, 1200, 200, COLOR_CARD_HIGHLIGHT);
                
                if lang_japanese {
                    draw_string(px + 80, py + 60, "スコアボード (SCOREBOARD)", 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                } else {
                    draw_string(px + 80, py + 60, "STADIUM SCOREBOARD", 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                }

                // Drawing header columns
                draw_string(px + 80, py + 120, "TEAM       7  8  9  |  R  H  E", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);

                // Saru High runs display
                let mut saru_runs_str = String::new();
                for i in 0..3 {
                    if i < current_scoreboard.saru_runs.len() && match_inning_step >= (i * 2 + 1) {
                        saru_runs_str.push_str(&format!("  {}  ", current_scoreboard.saru_runs[i]));
                    } else {
                        saru_runs_str.push_str("  -  ");
                    }
                }
                let saru_total_curr = if match_inning_step >= 5 { current_scoreboard.saru_total } else {
                    let mut s = 0;
                    for k in 0..3 {
                        if k * 2 + 1 <= match_inning_step {
                            s += current_scoreboard.saru_runs[k];
                        }
                    }
                    s
                };
                draw_string(px + 80, py + 150, &format!("SARU HIGH  {}|  {}", saru_runs_str, saru_total_curr), 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);

                // Rival runs display
                let mut rival_runs_str = String::new();
                for i in 0..3 {
                    if i < current_scoreboard.rival_runs.len() && match_inning_step >= (i * 2) {
                        rival_runs_str.push_str(&format!("  {}  ", current_scoreboard.rival_runs[i]));
                    } else {
                        rival_runs_str.push_str("  -  ");
                    }
                }
                let rival_total_curr = if match_inning_step >= 6 { current_scoreboard.rival_total } else {
                    let mut r = 0;
                    for k in 0..3 {
                        if k * 2 <= match_inning_step {
                            r += current_scoreboard.rival_runs[k];
                        }
                    }
                    r
                };
                draw_string(px + 80, py + 180, &format!("RIVAL CO   {}|  {}", rival_runs_str, rival_total_curr), 2, COLOR_RED.0, COLOR_RED.1, COLOR_RED.2);

                // Play narrative scrolling logs
                draw_card_panel(px + 40, py + 260, 1200, 420, COLOR_CARD_HIGHLIGHT);
                
                if lang_japanese {
                    draw_string(px + 80, py + 280, "ベンチ 監督 要録:", 2, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);
                } else {
                    draw_string(px + 80, py + 280, "BENCH MANAGEMENT PLAY-BY-PLAY FEED:", 2, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);
                }

                // Render visible logs
                let log_count = current_scoreboard.logs.len();
                let show_limit = if match_inning_step < log_count { match_inning_step + 1 } else { log_count };
                for idx in 0..show_limit {
                    let display_y = py + 330 + idx as i32 * 45;
                    draw_string(px + 80, display_y, &current_scoreboard.logs[idx], 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                }

                if lang_japanese {
                    draw_string(px + 80, py + 630, "[Enter] で 解説を 早送り", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                } else {
                    draw_string(px + 80, py + 630, "PRESS [ENTER] TO EXPEDITE PLAY-BY-PLAY FEED", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                }

                // Automatic narrative stepping timer
                match_tick_timer += dt;
                if match_tick_timer >= 1.5 || trig_action {
                    match_tick_timer = 0.0;
                    if match_inning_step < current_scoreboard.logs.len() - 1 {
                        match_inning_step += 1;
                        shake_timer = 0.15;
                    } else {
                        // Game ends, go to Result
                        screen = Screen::MatchResult;
                    }
                }
            }

            Screen::MatchResult => {
                let px = shake_x;
                let py = shake_y;
                runtime_draw_rect(px, py, 1280, 720, COLOR_BG.0, COLOR_BG.1, COLOR_BG.2);

                let is_victory = current_scoreboard.saru_total >= current_scoreboard.rival_total;
                let theme_c = if is_victory { COLOR_GOLD } else { COLOR_RED };

                draw_card_panel(px + 100, py + 100, 1080, 520, theme_c);

                if is_victory {
                    if lang_japanese {
                        draw_string(px + 150, py + 150, "* 試合終了 - サル高校 勝利! *", 4, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                        draw_string(px + 150, py + 240, &format!("最終 スコア : SARU {} - {} RIVAL", current_scoreboard.saru_total, current_scoreboard.rival_total), 3, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                        draw_string(px + 150, py + 320, "甲子園 スカウトが 選手たちを 重視 しています!", 2, COLOR_NEON_GREEN.0, COLOR_NEON_GREEN.1, COLOR_NEON_GREEN.2);
                        draw_string(px + 150, py + 370, "ボーナス: チーム全員の 能力が 最高 上昇!", 2, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);
                    } else {
                        draw_string(px + 150, py + 150, "* MATCH CONCLUDED - SARU HIGH VICTORY! *", 4, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                        draw_string(px + 150, py + 240, &format!("FINAL SCOREBOARD: SARU {} - {} RIVAL", current_scoreboard.saru_total, current_scoreboard.rival_total), 3, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                        draw_string(px + 150, py + 320, "PRO SCOUTS ARE DEEPLY IMPRESSED BY YOUR OUTSTANDING COACHING!", 2, COLOR_NEON_GREEN.0, COLOR_NEON_GREEN.1, COLOR_NEON_GREEN.2);
                        draw_string(px + 150, py + 370, "REWARD: ALL SQUAD MEMBER RATINGS RECEIVE EXTENSIVE BONUS BUFFS!", 2, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);
                    }
                } else {
                    if lang_japanese {
                        draw_string(px + 150, py + 150, "* 試合終了 - サル高校 敗北 *", 4, COLOR_RED.0, COLOR_RED.1, COLOR_RED.2);
                        draw_string(px + 150, py + 240, &format!("最終 スコア : SARU {} - {} RIVAL", current_scoreboard.saru_total, current_scoreboard.rival_total), 3, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                        draw_string(px + 150, py + 320, "残念! 敗北により 上昇幅は 縮小 です.", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                        draw_string(px + 150, py + 370, "監督として 次の 特訓を 工夫 しましょう。", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    } else {
                        draw_string(px + 150, py + 150, "[!] MATCH CONCLUDED - SARU HIGH DEFEAT [!]", 4, COLOR_RED.0, COLOR_RED.1, COLOR_RED.2);
                        draw_string(px + 150, py + 240, &format!("FINAL SCOREBOARD: SARU {} - {} RIVAL", current_scoreboard.saru_total, current_scoreboard.rival_total), 3, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                        draw_string(px + 150, py + 320, "UNFORTUNATE! MINIMAL REWARDS GAINED FROM THIS TOURNAMENT WEEK.", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                        draw_string(px + 150, py + 370, "ADJUST DRILLS METHODOLOGY TO ENHANCE BASELINE STATS SECURITIES.", 2, COLOR_TEXT_MUTED.0, COLOR_TEXT_MUTED.1, COLOR_TEXT_MUTED.2);
                    }
                }

                if lang_japanese {
                    draw_string(px + 150, py + 480, "[Enter] で 監督室に 戻る", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                } else {
                    draw_string(px + 150, py + 480, "PRESS [ENTER] TO RETURN TO OFFICE DESK", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                }

                if trig_action {
                    flash_timer = 0.2;
                    
                    // Apply victory/defeat stat bonuses
                    let bonus_range = if is_victory { (3, 8) } else { (1, 3) };
                    for p in &mut roster {
                        p.contact = (p.contact + rng.next_range(bonus_range.0, bonus_range.1)).min(100);
                        p.power = (p.power + rng.next_range(bonus_range.0, bonus_range.1)).min(100);
                        p.speed = (p.speed + rng.next_range(bonus_range.0, bonus_range.1)).min(100);
                        p.fielding = (p.fielding + rng.next_range(bonus_range.0, bonus_range.1)).min(100);
                    }

                    if week >= max_weeks {
                        screen = Screen::EndingDraft;
                    } else {
                        week += 1;
                        screen = Screen::MainHub;
                        selected_index = 0;
                    }
                }
            }

            Screen::EndingDraft => {
                let px = shake_x;
                let py = shake_y;
                runtime_draw_rect(px, py, 1280, 720, COLOR_BG.0, COLOR_BG.1, COLOR_BG.2);

                draw_card_panel(px + 60, py + 60, 1160, 600, COLOR_GOLD);
                
                if lang_japanese {
                    draw_string(px + 100, py + 90, "* プロ野球 ドラフト 記録式 *", 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 100, py + 140, "12週間の 特訓が 完了 しました。 スカウトの 指策:", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                } else {
                    draw_string(px + 100, py + 90, "* NPB PROFESSIONAL LEAGUE SCOUT DRAFT *", 3, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                    draw_string(px + 100, py + 140, "12 WEEKS SUMMER ROUTE FINISHED. SQUAD DRAFT RESULTS ARCHIVE:", 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                }

                // Check draft rounds based on individual player stats
                let mut drafted_count = 0;
                for i in 0..9 {
                    let p = &roster[i];
                    let avg_stat = (p.contact + p.power + p.speed + p.fielding) / 4;
                    let (draft_desc, color_g) = if avg_stat >= 75 || p.velocity >= 155 {
                        drafted_count += 1;
                        (if lang_japanese { "ドラフト 1位 指名! (1st Round)" } else { "1st ROUND DRAFT!" }, COLOR_GOLD)
                    } else if avg_stat >= 55 || p.velocity >= 140 {
                        drafted_count += 1;
                        (if lang_japanese { "ドラフト 3位 指名! (3rd Round)" } else { "3rd ROUND DRAFT" }, COLOR_CYAN)
                    } else if avg_stat >= 40 || p.velocity >= 130 {
                        drafted_count += 1;
                        (if lang_japanese { "育成 指名! (Training Round)" } else { "TRAINING DRAFT" }, COLOR_NEON_GREEN)
                    } else {
                        (if lang_japanese { "社会人 野球 (Semi-Pro League)" } else { "SEMI-PRO LEAGUE" }, COLOR_TEXT_MUTED)
                    };

                    let draw_y = py + 200 + i as i32 * 40;
                    draw_string(px + 120, draw_y, &format!("{}  {}", i + 1, p.name), 2, COLOR_TEXT_WHITE.0, COLOR_TEXT_WHITE.1, COLOR_TEXT_WHITE.2);
                    
                    let spec_tag = match roster[i].growth_type {
                        0 => "PITCHER",
                        1 => "SLUGGER",
                        2 => "SPEED",
                        3 => "DEFENSE",
                        _ => "BALANCE",
                    };
                    draw_string(px + 360, draw_y, spec_tag, 2, COLOR_CYAN.0, COLOR_CYAN.1, COLOR_CYAN.2);
                    draw_string(px + 590, draw_y, draft_desc, 2, color_g.0, color_g.1, color_g.2);
                }

                // Coach Rating Card
                if lang_japanese {
                    draw_string(px + 100, py + 590, &format!("指名数: {} / 9 選手指名! [Enter] で タイトルへ", drafted_count), 2, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                } else {
                    draw_string(px + 100, py + 590, &format!("DRAFTED COUNT: {} OF 9 SQUAD MEMBERS DRAFTED! PRESS [ENTER] TO REBOOT", drafted_count), 2, COLOR_GOLD.0, COLOR_GOLD.1, COLOR_GOLD.2);
                }

                if trig_action {
                    // Game finished, fully reboot
                    flash_timer = 0.3;
                    week = 1;
                    roster = vec![
                        Player::new("タナカ", "ACE PITCHER", 0, &mut rng),
                        Player::new("ヤマダ", "CLEANUP BATTER", 1, &mut rng),
                        Player::new("サトウ", "CAPTAIN RUNNER", 2, &mut rng),
                        Player::new("スズキ", "SPEED STAR", 2, &mut rng),
                        Player::new("タカハシ", "GLOVE WIZARD", 3, &mut rng),
                        Player::new("ワタナベ", "INFELDER ALLY", 4, &mut rng),
                        Player::new("イトウ", "OUTFIELDER SHIELD", 4, &mut rng),
                        Player::new("ナカムラ", "ROOKIE CLUTCH", 0, &mut rng),
                        Player::new("コバヤシ", "ROOKIE SLUGGER", 1, &mut rng),
                    ];
                    roster[0].velocity = 138; roster[0].breaking = 2; roster[0].contact = 25;
                    roster[1].power = 65; roster[1].contact = 45; roster[1].speed = 25;
                    roster[3].speed = 68; roster[3].power = 20;
                    roster[4].fielding = 65; roster[4].speed = 45;
                    screen = Screen::Title;
                }
            }
        }

        // Draw active screen flash
        if flash_timer > 0.0 {
            runtime_draw_rect(0, 0, 1280, 720, 255, 255, 255);
        }

        runtime_present();
    }

    runtime_shutdown();
}
