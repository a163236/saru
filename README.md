# Saru OS / Runtime (Phase 1) 開発・実行ガイド

Saru OS は、Linux ベースのゲーム特化プラットフォームです。
現在（Phase 1）は、Ubuntu 等の Linux ディストリビューション上で動作する**ゲームランタイム（Runtime）**、**ゲームランチャー（Launcher）**、および **SDK** の開発を行っています。

本リポジトリには、ゲームランチャー、共通ランタイム、および 2 つのサンプルゲーム（`sample_game` と `retro_adventure`）が含まれています。

---

## 🎮 動作イメージ・システム構成

```text
Linux デスクトップ環境 (Ubuntu等)
        ↓
   Saru ランチャー (launcher)  ←← [ゲーム選択・起動管理]
        ↓ (プロセス起動)
   ゲームランタイム (runtime)   ←← [描画・入力・時間等の共通抽象化API]
        ↓
 実際のゲーム (retro_adventure / sample_game)
```

---

## 🛠 1. 事前準備 (Prerequisites)

本リポジトリでは SDL3 をソースコードからビルドする設定（`build-from-source`）を有効にしています。そのため、ビルドを行う前に SDL3 のコンパイルに必要なシステムライブラリ（CMake、Ninja、各種開発用パッケージ）をインストールする必要があります。

### Ubuntu / Debian の場合
以下のコマンドを実行して、必要なパッケージをインストールしてください。

```bash
sudo apt update
sudo apt install -y \
  build-essential \
  cmake \
  ninja-build \
  pkg-config \
  libasound2-dev \
  libpulse-dev \
  libx11-dev \
  libxext-dev \
  libxrandr-dev \
  libxcursor-dev \
  libxi-dev \
  libxinerama-dev \
  libxkbcommon-dev \
  libwayland-dev \
  libegl1-mesa-dev \
  libgl1-mesa-dev \
  libgles2-mesa-dev \
  libdbus-1-dev
```

---

## 📦 2. ビルド方法

ワークスペース全体のビルドを行います。ランチャーはデバッグビルド成果物（`target/debug/`）内のバイナリを参照してゲームを起動するため、**通常のデバッグビルド**を行ってください。

リポジトリのルートディレクトリで以下のコマンドを実行します。

```bash
cargo build
```

これにより、以下のコンポーネントがビルドされ、`target/debug/` 配下に実行ファイルが生成されます：
*   `launcher` (ゲームランチャー本体)
*   `sample_game` (シンプルなバウンドボールデモ)
*   `retro_adventure` (レトロな宇宙船シューティングゲーム)

---

## 🚀 3. 実行方法

ゲームランチャーを起動することで、登録されているゲームをメニューから選択して遊ぶことができます。

```bash
# ランチャーの起動
cargo run --bin launcher

# または、ビルドされたバイナリを直接実行
./target/debug/launcher
```

---

## 🕹 4. 操作方法 (Controls)

ランチャーおよび各ゲームは、以下のキーアサインで共通して操作可能です。キーボードのほか、SDL3が認識する一般的なゲームコントローラ（Xbox, DualShock 4, Nintendo Switch Pro Controller等）にも対応しています。

| 操作内容 | キーボード入力 | コントローラ入力 (目安) |
| :--- | :--- | :--- |
| **移動 / カーソル選択** | `↑` `↓` `←` `→` または `W` `S` `A` `D` | 方向キー / Lスティック |
| **決定 / アクションA (ショット等)** | `Enter` キー または `J` キー | Aボタン / ✕ボタン |
| **アクションB (キャンセル等)** | `Space` キー または `K` キー | Bボタン / ◯ボタン |
| **ゲーム終了 / ランチャー復帰** | `Escape` (ESC) キー | START / OPTION ボタン |

> [!NOTE]
> *   **ランチャー画面で `Escape` または `ESC` キーを押す**と、ランチャーが終了します。
> *   **ゲーム中に `Escape` または `ESC` キーを押す**と、ゲームプロセスが終了し、自動的にランチャー画面に戻ります。

---

## ➕ 5. 自作ゲームの追加方法

新しいゲームをこのプラットフォーム上に追加する場合は、以下の手順に従ってください。

1.  **ゲーム用ディレクトリの作成**  
    `games/` ディレクトリ配下に新しいゲームフォルダを作成します。
    ```bash
    mkdir -p games/my_new_game/src
    ```
2.  **`meta.json` の作成**  
    `games/my_new_game/meta.json` を作成し、ゲームのメタ情報を記述します。`exec` にはビルド後に生成されるバイナリ名を指定します。
    ```json
    {
      "name": "My New Game",
      "exec": "my_new_game",
      "resolution": "1280x720",
      "controller_required": true,
      "version": "1.0.0"
    }
    ```
3.  **`Cargo.toml` の作成**  
    `games/my_new_game/Cargo.toml` を作成し、共通の `runtime` クレートへの依存を記述します。
    ```toml
    [package]
    name = "my_new_game"
    version = "0.1.0"
    edition = "2021"

    [dependencies]
    runtime = { path = "../../runtime" }
    ```
4.  **ゲームコードの実装**  
    `games/my_new_game/src/main.rs` を作成し、`runtime` のAPIを利用してゲームループを記述します。
    ```rust
    use runtime::{runtime_init, runtime_shutdown, runtime_get_delta_time, runtime_get_input, runtime_clear, runtime_present};
    use std::thread::sleep;
    use std::time::Duration;

    fn main() {
        runtime_init();
        let mut is_running = true;
        while is_running {
            let dt = runtime_get_delta_time();
            let input = runtime_get_input();
            if input.quit || input.start {
                is_running = false;
                break;
            }
            runtime_clear();
            // ここにゲームの描画やロジックを実装
            runtime_present();
            sleep(Duration::from_millis(16));
        }
        runtime_shutdown();
    }
    ```
5.  **ワークスペースへの追加**  
    ルートディレクトリの [Cargo.toml](file:///home/keita/saru/Cargo.toml) の `members` 配列に `"games/my_new_game"` を追記します。
6.  **ビルドと実行**  
    再度ルートで `cargo build` を実行すると、ランチャーのリストに自動で新しいゲームが追加され、起動できるようになります。
