use runtime::{
    runtime_init, runtime_shutdown, runtime_get_delta_time,
    runtime_get_input, runtime_clear, runtime_present, runtime_draw_rect
};
use std::thread::sleep;
use std::time::Duration;

struct Bullet {
    x: f32,
    y: f32,
}

struct Enemy {
    x: f32,
    y: f32,
    speed: f32,
}

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
        'S' => "###\
                #..\
                ###\
                ..#\
                ###",
        'C' => "###\
                #..\
                #..\
                #..\
                ###",
        'O' => "###\
                #.#\
                #.#\
                #.#\
                ###",
        'R' => "###\
                #.#\
                ###\
                #.#\
                #.#",
        'E' => "###\
                #..\
                ###\
                #..\
                ###",
        'P' => "###\
                #.#\
                ###\
                #..\
                #..",
        'L' => "#..\
                #..\
                #..\
                #..\
                ###",
        'A' => "###\
                #.#\
                ###\
                #.#\
                #.#",
        'Y' => "#.#\
                #.#\
                ###\
                ..#\
                ..#",
        'G' => "###\
                #..\
                #.#\
                #.#\
                ###",
        'I' => "###\
                .#.\
                .#.\
                .#.\
                ###",
        'N' => "#.#\
                ###\
                ###\
                #.#\
                #.#",
        'M' => "###\
                ###\
                #.#\
                #.#\
                #.#",
        'V' => "#.#\
                #.#\
                #.#\
                #.#\
                .#.",
        'H' => "#.#\
                #.#\
                ###\
                #.#\
                #.#",
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
        'F' => "###\
                #..\
                ###\
                #..\
                #..",
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
    for c in text.chars() {
        draw_char(x, y, c, pixel_size, r, g, b);
        x += 4 * pixel_size;
    }
}

fn load_highscore() -> u32 {
    if let Ok(content) = std::fs::read_to_string("highscore.txt") {
        if let Ok(score) = content.trim().parse::<u32>() {
            return score;
        }
    }
    0
}

fn save_highscore(score: u32) {
    let _ = std::fs::write("highscore.txt", score.to_string());
}

fn main() {
    println!("=== レトロアドベンチャー (Space Shooter) 起動 ===");
    runtime_init();

    let mut px = 100.0;
    let mut py = 360.0 - 20.0;
    let p_speed = 350.0;
    let mut hp: u32 = 5;
    let max_hp: u32 = 5;
    let mut score = 0;
    let mut highscore = load_highscore();
    let mut shoot_cooldown = 0.0f32;
    let mut damage_flash = 0.0f32;


    let mut bullets: Vec<Bullet> = Vec::new();
    let mut enemies: Vec<Enemy> = Vec::new();

    let mut stars = vec![
        (120.0, 100.0, 80.0),
        (450.0, 300.0, 120.0),
        (800.0, 200.0, 60.0),
        (1000.0, 600.0, 160.0),
        (600.0, 500.0, 200.0),
        (200.0, 650.0, 90.0),
        (950.0, 150.0, 110.0),
    ];

    let mut enemy_spawn_timer = 0.0f32;
    let mut animation_time = 0.0f32;

    let mut is_running = true;
    let mut is_gameover = false;

    while is_running {
        let dt = runtime_get_delta_time();
        animation_time += dt * 5.0;
        let input = runtime_get_input();
        
        if input.quit || input.start {
            is_running = false;
            break;
        }

        if !is_gameover {
            if input.up { py -= p_speed * dt; }
            if input.down { py += p_speed * dt; }
            if input.left { px -= p_speed * dt; }
            if input.right { px += p_speed * dt; }

            if px < 10.0 { px = 10.0; }
            if px > 600.0 { px = 600.0; }
            if py < 90.0 { py = 90.0; }
            if py > 630.0 - 40.0 { py = 630.0 - 40.0; }

            if shoot_cooldown > 0.0 {
                shoot_cooldown -= dt;
            }
            if input.action_a && shoot_cooldown <= 0.0 {
                bullets.push(Bullet {
                    x: px + 40.0,
                    y: py + 16.0,
                });
                shoot_cooldown = 0.15;
            }

            if damage_flash > 0.0 {
                damage_flash -= dt;
            }

            enemy_spawn_timer += dt;
            if enemy_spawn_timer >= 0.8 {
                use std::time::SystemTime;
                let nanos = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
                let spawn_y = 100.0 + (nanos % 500) as f32;
                let speed = 180.0 + (nanos % 180) as f32;
                enemies.push(Enemy {
                    x: 1280.0,
                    y: spawn_y,
                    speed,
                });
                enemy_spawn_timer = 0.0;
            }

            for star in stars.iter_mut() {
                star.0 -= star.2 * dt;
                if star.0 < 0.0 {
                    star.0 = 1280.0;
                }
            }

            for bullet in bullets.iter_mut() {
                bullet.x += 700.0 * dt;
            }
            bullets.retain(|b| b.x < 1280.0);

            for enemy in enemies.iter_mut() {
                enemy.x -= enemy.speed * dt;
            }
            
            let mut damaged_by_escape = 0;
            enemies.retain(|e| {
                if e.x < 0.0 {
                    damaged_by_escape += 1;
                    false
                } else {
                    true
                }
            });
            if damaged_by_escape > 0 {
                hp = hp.saturating_sub(damaged_by_escape);
                damage_flash = 0.2;
                if hp == 0 {
                    is_gameover = true;
                    save_highscore(highscore);
                }
            }

            let mut hit_bullets = std::collections::HashSet::new();
            let mut hit_enemies = std::collections::HashSet::new();

            for (b_idx, bullet) in bullets.iter().enumerate() {
                for (e_idx, enemy) in enemies.iter().enumerate() {
                    if bullet.x < enemy.x + 40.0 &&
                       bullet.x + 15.0 > enemy.x &&
                       bullet.y < enemy.y + 30.0 &&
                       bullet.y + 6.0 > enemy.y - 8.0 {
                        hit_bullets.insert(b_idx);
                        hit_enemies.insert(e_idx);
                        score += 100;
                        if score > highscore {
                            highscore = score;
                        }
                    }
                }
            }

            let mut b_count = 0;
            bullets.retain(|_| {
                let keep = !hit_bullets.contains(&b_count);
                b_count += 1;
                keep
            });

            let mut e_count = 0;
            enemies.retain(|_| {
                let keep = !hit_enemies.contains(&e_count);
                e_count += 1;
                keep
            });

            let mut hit_player_enemy_idx = None;
            for (e_idx, enemy) in enemies.iter().enumerate() {
                if px < enemy.x + 40.0 &&
                   px + 40.0 > enemy.x &&
                   py < enemy.y + 30.0 &&
                   py + 40.0 > enemy.y - 8.0 {
                    hit_player_enemy_idx = Some(e_idx);
                    break;
                }
            }

            if let Some(idx) = hit_player_enemy_idx {
                enemies.remove(idx);
                hp = hp.saturating_sub(1);
                damage_flash = 0.3;
                if hp == 0 {
                    is_gameover = true;
                    save_highscore(highscore);
                }
            }

        } else {
            if input.action_a {
                px = 100.0;
                py = 360.0 - 20.0;
                hp = max_hp;
                score = 0;
                bullets.clear();
                enemies.clear();
                is_gameover = false;
            }
        }

        runtime_clear();

        for star in &stars {
            runtime_draw_rect(star.0 as i32, star.1 as i32, 6, 6, 120, 130, 160);
        }

        for bullet in &bullets {
            runtime_draw_rect(bullet.x as i32, bullet.y as i32, 15, 6, 255, 255, 0);
        }

        for enemy in &enemies {
            runtime_draw_rect(enemy.x as i32, enemy.y as i32, 40, 24, 255, 50, 50);
            runtime_draw_rect(enemy.x as i32 + 8, enemy.y as i32 - 8, 24, 8, 255, 100, 100);
            runtime_draw_rect(enemy.x as i32 + 12, enemy.y as i32 + 24, 16, 6, 255, 120, 0);
        }

        if !is_gameover {
            let (r, g, b) = if damage_flash > 0.0 && ((damage_flash * 15.0) as i32 % 2 == 0) {
                (255, 50, 50)
            } else {
                (0, 200, 255)
            };
            runtime_draw_rect(px as i32, py as i32 + 12, 40, 16, r, g, b);
            runtime_draw_rect(px as i32 + 10, py as i32, 20, 40, r, g, b);
            runtime_draw_rect(px as i32 + 30, py as i32 + 16, 10, 8, 255, 255, 255);
            runtime_draw_rect(px as i32 - 10, py as i32 + 16, 10, 8, 255, 100, 0);
        }

        runtime_draw_rect(0, 0, 1280, 80, 15, 20, 30);
        runtime_draw_rect(0, 78, 1280, 2, 0, 220, 255);

        let score_bar_w = ((score / 100) * 10).min(600) as u32;
        runtime_draw_rect(40, 30, score_bar_w, 20, 0, 220, 255);
        runtime_draw_rect(40, 30, 600, 2, 50, 70, 90);
        runtime_draw_rect(40, 50, 600, 2, 50, 70, 90);

        // Score displays next to the score bar
        draw_string(660, 18, &format!("HI-SCORE:{:06}", highscore), 3, 255, 220, 0);
        draw_string(660, 45, &format!("SCORE:   {:06}", score), 3, 0, 220, 255);

        // HP Label before hearts
        draw_string(930, 30, "HP:", 4, 50, 255, 50);

        for h in 0..max_hp {
            let hx = 1000 + (h * 45) as i32;
            if h < hp {
                runtime_draw_rect(hx, 25, 30, 30, 50, 255, 50);
            } else {

                runtime_draw_rect(hx, 25, 30, 30, 80, 40, 40);
            }
        }

        if is_gameover {
            runtime_draw_rect(440, 200, 400, 320, 30, 10, 10);
            runtime_draw_rect(450, 210, 380, 300, 50, 20, 20);
            
            for d in 0..100 {
                runtime_draw_rect(540 + d, 280 + d, 12, 12, 255, 50, 50);
                runtime_draw_rect(640 - d, 280 + d, 12, 12, 255, 50, 50);
            }

            // Draw "GAME OVER" text inside the window
            draw_string(535, 230, "GAME OVER", 6, 255, 50, 50);

            // Draw final score and high score
            draw_string(480, 370, &format!("HI-SCORE:    {:06}", highscore), 3, 255, 220, 0);
            draw_string(480, 395, &format!("FINAL SCORE: {:06}", score), 3, 0, 220, 255);
            
            let pulse_color = if (animation_time.sin() * 5.0) as i32 % 2 == 0 { 200 } else { 50 };
            runtime_draw_rect(500, 440, 280, 20, 50, pulse_color, 50);

            // Draw pulsing restart instructions inside the pulsing rect
            draw_string(508, 442, "PRESS ENTER TO RESTART", 3, 255, 255, 255);
        }

        runtime_present();
        sleep(Duration::from_millis(16));
    }

    save_highscore(highscore);
    println!("=== レトロアドベンチャー シャットダウン ===");
    runtime_shutdown();
    println!("=== レトロアドベンチャー 終了 ===");
}
