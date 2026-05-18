use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;
use serde::Deserialize;

use runtime::{
    runtime_init, runtime_shutdown, runtime_get_delta_time,
    runtime_get_input, runtime_clear, runtime_present, runtime_draw_rect
};

#[derive(Debug, Deserialize)]
struct GameMeta {
    name: String,
    #[serde(default)]
    name_en: String,
    exec: String,
    resolution: String,
    controller_required: bool,
    version: String,
}

struct GameInfo {
    dir_path: PathBuf,
    meta: GameMeta,
}

fn scan_games(games_dir: &str) -> Vec<GameInfo> {
    let mut games = Vec::new();
    let path = Path::new(games_dir);
    if !path.exists() {
        println!("ゲームディレクトリ {} が存在しません。", games_dir);
        return games;
    }

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let dir = entry.path();
            if dir.is_dir() {
                let meta_path = dir.join("meta.json");
                if meta_path.exists() {
                    if let Ok(content) = fs::read_to_string(&meta_path) {
                        if let Ok(meta) = serde_json::from_str::<GameMeta>(&content) {
                            games.push(GameInfo {
                                dir_path: dir,
                                meta,
                            });
                        }
                    }
                }
            }
        }
    }
    // アルファベット順にソートしてリストを安定させる
    games.sort_by(|a, b| a.meta.name.cmp(&b.meta.name));
    games
}

fn launch_game(game: &GameInfo) {
    println!("=== ゲーム起動プロセス開始 ===");
    
    runtime_shutdown();
    sleep(Duration::from_millis(100));

    let exe_name = &game.meta.exec;
    let target_dir = Path::new("/home/keita/saru/target/debug");
    let exe_path = target_dir.join(exe_name);

    if !exe_path.exists() {
        println!("エラー: 実行ファイル {:?} が見つかりません。先にビルドしてください。", exe_path);
        runtime_init();
        return;
    }

    println!("実行中: {:?}", exe_path);
    let mut child = Command::new(&exe_path)
        .current_dir(&game.dir_path)
        .spawn()
        .expect("ゲームの起動に失敗しました");

    let status = child.wait().expect("ゲームプロセスの待機に失敗しました");
    println!("ゲームが終了しました。ステータス: {:?}", status);
    
    sleep(Duration::from_millis(150));

    runtime_init();
    println!("=== ランチャーUIに復帰しました ===");
}

// 🔠 レトロゲーム風の3x5ピクセルフォントを描画する関数
fn draw_char(x: i32, y: i32, c: char, pixel_size: i32, r: u8, g: u8, b: u8) {
    let pattern = match c {
        '0' => "###\
                #.#\
                #.#\
                #.#\
                ###",
        '1' => "..#\
                ..#\
                ..#\
                ..#\
                ..#",
        '2' => "###\
                ..#\
                ###\
                #..\
                ###",
        '3' => "###\
                ..#\
                ###\
                ..#\
                ###",
        '4' => "#.#\
                #.#\
                ###\
                ..#\
                ..#",
        '5' => "###\
                #..\
                ###\
                ..#\
                ###",
        '6' => "###\
                #..\
                ###\
                #.#\
                ###",
        '7' => "###\
                ..#\
                ..#\
                ..#\
                ..#",
        '8' => "###\
                #.#\
                ###\
                #.#\
                ###",
        '9' => "###\
                #.#\
                ###\
                ..#\
                ###",
        'A' => "###\
                #.#\
                ###\
                #.#\
                #.#",
        'B' => "###\
                #.#\
                ##.\
                #.#\
                ###",
        'C' => "###\
                #..\
                #..\
                #..\
                ###",
        'D' => "##.\
                #.#\
                #.#\
                #.#\
                ##.",
        'E' => "###\
                #..\
                ###\
                #..\
                ###",
        'F' => "###\
                #..\
                ###\
                #..\
                #..",
        'G' => "###\
                #..\
                #.#\
                #.#\
                ###",
        'H' => "#.#\
                #.#\
                ###\
                #.#\
                #.#",
        'I' => "###\
                .#.\
                .#.\
                .#.\
                ###",
        'J' => "..#\
                ..#\
                ..#\
                #.#\
                ###",
        'K' => "#.#\
                #.#\
                ##.\
                #.#\
                #.#",
        'L' => "#..\
                #..\
                #..\
                #..\
                ###",
        'M' => "###\
                ###\
                #.#\
                #.#\
                #.#",
        'N' => "#.#\
                ###\
                ###\
                #.#\
                #.#",
        'O' => "###\
                #.#\
                #.#\
                #.#\
                ###",
        'P' => "###\
                #.#\
                ###\
                #..\
                #..",
        'Q' => "###\
                #.#\
                #.#\
                ###\
                ..#",
        'R' => "###\
                #.#\
                ###\
                #.#\
                #.#",
        'S' => "###\
                #..\
                ###\
                ..#\
                ###",
        'T' => "###\
                .#.\
                .#.\
                .#.\
                .#.",
        'U' => "#.#\
                #.#\
                #.#\
                #.#\
                ###",
        'V' => "#.#\
                #.#\
                #.#\
                #.#\
                .#.",
        'W' => "#.#\
                #.#\
                #.#\
                ###\
                ###",
        'X' => "#.#\
                #.#\
                .#.\
                #.#\
                #.#",
        'Y' => "#.#\
                #.#\
                ###\
                ..#\
                ..#",
        'Z' => "###\
                ..#\
                .#.\
                #..\
                ###",
        ':' => "...\
                .#.\
                ...\
                .#.\
                ...",
        '-' => "...\
                ...\
                ###\
                ...\
                ...",
        '.' => "...\
                ...\
                ...\
                ...\
                .#.",
        ' ' => "...\
                ...\
                ...\
                ...\
                ...",
        _ =>   "###\
                #.#\
                ###\
                #.#\
                #.#",
    };

    for (idx, ch) in pattern.chars().enumerate() {
        if ch == '#' {
            let px = idx % 3;
            let py = idx / 3;
            runtime_draw_rect(
                x + px as i32 * pixel_size,
                y + py as i32 * pixel_size,
                pixel_size as u32,
                pixel_size as u32,
                r, g, b
            );
        }
    }
}

fn draw_string(mut x: i32, y: i32, text: &str, pixel_size: i32, r: u8, g: u8, b: u8) {
    for c in text.to_uppercase().chars() {
        draw_char(x, y, c, pixel_size, r, g, b);
        x += 4 * pixel_size;
    }
}


// 🎮 コントローラのピクセルアートを描画する関数
fn draw_controller_icon(x: i32, y: i32, is_selected: bool) {
    let (r, g, b) = if is_selected { (0, 220, 255) } else { (120, 125, 130) };
    
    // コントローラのボディ
    runtime_draw_rect(x + 2, y + 10, 44, 26, r, g, b);
    runtime_draw_rect(x + 6, y + 6, 36, 34, r, g, b);
    
    // グリップ
    runtime_draw_rect(x + 2, y + 32, 10, 10, r, g, b);
    runtime_draw_rect(x + 36, y + 32, 10, 10, r, g, b);

    // 十字キー (濃いグレー)
    runtime_draw_rect(x + 10, y + 18, 10, 4, 30, 35, 40);
    runtime_draw_rect(x + 13, y + 15, 4, 10, 30, 35, 40);
    
    // ボタン (ネオンオレンジ / イエロー)
    runtime_draw_rect(x + 30, y + 20, 4, 4, 255, 120, 0);
    runtime_draw_rect(x + 35, y + 15, 4, 4, 255, 220, 0);
}

// 🚀 宇宙船のピクセルアートを描画する関数
fn draw_spaceship_icon(x: i32, y: i32, is_selected: bool) {
    let (r, g, b) = if is_selected { (255, 220, 0) } else { (120, 125, 130) };

    // 機体（中央）
    runtime_draw_rect(x + 20, y + 4, 8, 38, r, g, b);
    runtime_draw_rect(x + 16, y + 14, 16, 24, r, g, b);
    
    // 翼
    runtime_draw_rect(x + 8, y + 26, 32, 10, r, g, b);
    runtime_draw_rect(x + 2, y + 32, 44, 6, r, g, b);
    
    // スラスターの炎 (ネオンレッド)
    runtime_draw_rect(x + 20, y + 42, 8, 4, 255, 50, 50);
}

fn main() {
    println!("=== SaruOS ランチャー 起動 ===");
    let games = scan_games("/home/keita/saru/games");
    if games.is_empty() {
        println!("エラー: ゲームが見つかりませんでした。 /games/sample_game を配置してください。");
        return;
    }

    runtime_init();

    let mut selected_index = 0;
    let mut is_running = true;
    
    let mut up_pressed = false;
    let mut down_pressed = false;
    let mut action_pressed = false;

    // 脈動アニメーション用の時間管理
    let mut animation_time = 0.0f32;

    while is_running {
        let dt = runtime_get_delta_time();
        animation_time += dt * 5.0; // 脈動スピード

        let input = runtime_get_input();

        if input.quit {
            is_running = false;
            break;
        }

        if input.down {
            if !down_pressed {
                selected_index = (selected_index + 1) % games.len();
                down_pressed = true;
            }
        } else {
            down_pressed = false;
        }

        if input.up {
            if !up_pressed {
                if selected_index == 0 {
                    selected_index = games.len() - 1;
                } else {
                    selected_index -= 1;
                }
                up_pressed = true;
            }
        } else {
            up_pressed = false;
        }

        if input.action_a {
            if !action_pressed {
                action_pressed = true;
                let game_to_launch = &games[selected_index];
                launch_game(game_to_launch);
            }
        } else {
            action_pressed = false;
        }

        if input.start {
            is_running = false;
            break;
        }

        runtime_clear();

        // 1. ヘッダーバーの描画 (ネオン感のある高級ダークブルー)
        runtime_draw_rect(0, 0, 1280, 90, 15, 20, 35);
        // 高級感のあるライン
        runtime_draw_rect(0, 88, 1280, 2, 0, 180, 255);
        
        // ヘッダー内のロゴマーク (プレミアムネオンオレンジ)
        runtime_draw_rect(40, 25, 40, 40, 255, 120, 0);
        // ロゴテキストとサブタイトル
        draw_string(95, 22, "SARU OS", 4, 240, 240, 245);
        draw_string(95, 52, "GAME PLATFORM", 2, 0, 220, 255);

        // 2. ゲームリストエリアの描画
        // 脈動計算： -3 から +3 ピクセル
        let pulse = (animation_time.sin() * 3.0) as i32;

        for (i, game) in games.iter().enumerate() {
            let item_y = 130 + (i * 120) as i32;
            
            if i == selected_index {
                // 選択されているゲーム：光り輝くシアンのネオン枠
                runtime_draw_rect(45 - pulse, item_y - pulse, (1190 + pulse * 2) as u32, (100 + pulse * 2) as u32, 0, 220, 255);
                // 選択インジケータ
                runtime_draw_rect(55 - pulse, item_y + 10, 12, 80, 255, 255, 255);
                // ゲームカード内部 (プレミアムダークブルー)
                runtime_draw_rect(80 - pulse, item_y + 5, (1145 + pulse * 2) as u32, 90, 25, 30, 50);
            } else {

                // 選択されていないゲーム：落ち着いたダークグレー
                runtime_draw_rect(45, item_y, 1190, 100, 35, 40, 48);
                // ゲームカード内部
                runtime_draw_rect(80, item_y + 5, 1145, 90, 48, 52, 60);
            }
            
            // アイコンエリアの描画
            let icon_x = if i == selected_index { 105 - pulse } else { 105 };
            let icon_y = if i == selected_index { item_y + 25 - pulse/2 } else { item_y + 25 };
            
            // 各ゲームに応じた美しいカスタムピクセルアートを表示！
            if game.meta.exec == "sample_game" {
                draw_controller_icon(icon_x, icon_y, i == selected_index);
            } else {
                draw_spaceship_icon(icon_x, icon_y, i == selected_index);
            }
            
            // ゲームタイトルとバージョンを描画
            let text_x = if i == selected_index { 180 - pulse } else { 180 };
            let title = if !game.meta.name_en.is_empty() {
                &game.meta.name_en
            } else {
                &game.meta.exec
            };
            let version_str = format!("V{}", game.meta.version);
            
            if i == selected_index {
                // 選択中：白の大きなタイトルと黄色のバージョン表記
                draw_string(text_x, item_y + 20, title, 4, 255, 255, 255);
                draw_string(text_x, item_y + 60, &version_str, 2, 255, 220, 0);
            } else {
                // 非選択中：落ち着いたグレーのタイトルとバージョン表記
                draw_string(text_x, item_y + 22, title, 3, 180, 185, 190);
                draw_string(text_x, item_y + 58, &version_str, 2, 130, 135, 140);
            }
        }

        // 3. フッターバーの描画 (操作ガイド)
        runtime_draw_rect(0, 630, 1280, 90, 20, 22, 28);
        runtime_draw_rect(0, 630, 1280, 2, 0, 180, 255); // フッター上部のライン

        // ガイド [↑↓] 選択 (シアン)
        runtime_draw_rect(50, 660, 65, 30, 0, 180, 255);
        draw_string(50 + (65 - 32)/2, 668, "MOVE", 2, 255, 255, 255);
        draw_string(130, 667, "SELECT", 2, 200, 205, 210);
        
        // ガイド [A / Enter] 起動 (グリーン)
        runtime_draw_rect(280, 660, 120, 30, 0, 200, 100);
        draw_string(280 + (120 - 40)/2, 668, "ENTER", 2, 255, 255, 255);
        draw_string(410, 667, "START GAME", 2, 200, 205, 210);

        // ガイド [ESC] 終了 (レッド)
        runtime_draw_rect(580, 660, 65, 30, 220, 60, 60);
        draw_string(580 + (65 - 24)/2, 668, "ESC", 2, 255, 255, 255);
        draw_string(660, 667, "QUIT", 2, 200, 205, 210);

        runtime_present();

        sleep(Duration::from_millis(16));
    }

    runtime_shutdown();
    println!("=== SaruOS ランチャー 終了 ===");
}

