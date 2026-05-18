use runtime::{
    runtime_init, runtime_shutdown, runtime_get_delta_time,
    runtime_get_input, runtime_clear, runtime_present, runtime_draw_rect
};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!("サンプルゲーム: 起動中...");
    runtime_init();

    println!("サンプルゲーム: ゲームループ開始！");
    
    // プレイヤーの初期座標と速度
    let mut px = 640.0 - 25.0;
    let mut py = 360.0 - 25.0;
    let p_speed = 350.0; // 秒間350ピクセル

    // エネミーの初期座標と速度
    let mut ex = 100.0;
    let mut ey = 100.0;
    let mut evx = 250.0;
    let mut evy = 200.0;

    let mut is_running = true;

    while is_running {
        // 1. 経過時間を取得
        let dt = runtime_get_delta_time();
        
        // 2. 入力を取得
        let input = runtime_get_input();
        
        // 終了判定 (ウィンドウの閉じるボタンかEscapeキー)
        if input.quit || input.start {
            is_running = false;
            break;
        }

        // 3. プレイヤー移動処理
        if input.up {
            py -= p_speed * dt;
        }
        if input.down {
            py += p_speed * dt;
        }
        if input.left {
            px -= p_speed * dt;
        }
        if input.right {
            px += p_speed * dt;
        }

        // プレイヤーの画面外はみ出し制限
        if px < 0.0 { px = 0.0; }
        if px > 1280.0 - 50.0 { px = 1280.0 - 50.0; }
        if py < 0.0 { py = 0.0; }
        if py > 720.0 - 50.0 { py = 720.0 - 50.0; }

        // 4. エネミーの自律移動（バウンド）
        ex += evx * dt;
        ey += evy * dt;

        if ex < 0.0 {
            ex = 0.0;
            evx = -evx;
        }
        if ex > 1280.0 - 40.0 {
            ex = 1280.0 - 40.0;
            evx = -evx;
        }
        if ey < 0.0 {
            ey = 0.0;
            evy = -evy;
        }
        if ey > 720.0 - 40.0 {
            ey = 720.0 - 40.0;
            evy = -evy;
        }

        // 5. 描画処理
        runtime_clear();

        // エネミー（赤）を描画
        runtime_draw_rect(ex as i32, ey as i32, 40, 40, 255, 50, 50);

        // プレイヤー（緑）を描画
        runtime_draw_rect(px as i32, py as i32, 50, 50, 50, 255, 50);

        runtime_present();

        // フレームレート制限（約60FPS）
        sleep(Duration::from_millis(16));
    }

    println!("サンプルゲーム: シャットダウン中...");
    runtime_shutdown();
    println!("サンプルゲーム: 終了！");
}
