use anyhow::Result;
use clap::{Parser, ValueEnum};
use crossterm::{
    cursor::{MoveDown, MoveLeft, MoveTo, Show},
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode},
};
use inquire::Text;
use log::{info, warn};
use pretty_env_logger;
use ratatui::widgets::Gauge;
use ratatui::{
    Frame, Terminal,
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, List, ListItem},
};
use serde::Serialize;
use std::{error::Error, io, thread, time::Duration};
/*



/// コマンドライン引数の定義
#[derive(Parser, Debug, Serialize)]
#[command(name = "Rust CLI with TUI")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(version = "1.0")]
#[command(about = "A CLI tool with TUI for progress visualization", long_about = None)]
struct Args {
    /// プログレスバーのタイトル
    #[arg(short, long, default_value = "Progress")]
    title: String,

    /// 初期の進捗率 (0-100)
    #[arg(short, long, default_value_t = 0)]
    progress: u8,

    /// プログレスバーの色
    #[arg(short, long, value_enum, default_value = "green")]
    color: BarColor,
}

/// プログレスバーの色オプション
#[derive(Debug, Clone, ValueEnum, Serialize)]
enum BarColor {
    Red,
    Green,
    Blue,
    Cyan,
    Yellow,
}

impl BarColor {
    /// 色を TUI の `Color` 型に変換
    fn to_color(&self) -> Color {
        match self {
            BarColor::Red => Color::Red,
            BarColor::Green => Color::Green,
            BarColor::Blue => Color::Blue,
            BarColor::Cyan => Color::Cyan,
            BarColor::Yellow => Color::Yellow,
        }
    }
}

fn main() -> Result<()> {
    // 1. コマンドライン引数を解析して構造体にマッピング
    let args = Args::parse();

    // 2. JSON で構造体を確認（デバッグ用）
    println!("{}", serde_json::to_string_pretty(&args)?);

    // 3. ターミナルの初期化
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 初期進捗率
    let mut progress = args.progress as f64 / 100.0;

    loop {
        // 4. 描画処理
        terminal.draw(|f| {
            // レイアウト設定
            let size = f.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
                .split(size);

            // プログレスゲージの作成
            let gauge = Gauge::default()
                .block(Block::default().title(&*args.title).borders(Borders::ALL))
                .gauge_style(Style::default().fg(args.color.to_color()))
                .percent((progress * 100.0) as u16);
            f.render_widget(gauge, chunks[0]);
        })?;

        // キー入力の確認
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break; // "q"で終了
                }
            }
        }

        // 進捗の更新
        progress += 0.01;
        if progress > 1.0 {
            progress = 0.0;
        }

        // 更新間隔
        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
*/

fn main() -> Result<()> {
 
  
    pretty_env_logger::init();

    for i in 1..11 {
        info!("いんふぉー{}",i);
        warn!("けいこくー{}",i);
    }

    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let items = vec![
        ListItem::new("Option 1"),
        ListItem::new("Option 2"),
        ListItem::new("Option 3"),
    ];

    // ユーザーに名前を尋ねる
    let name = Text::new("What's your name?")
        .with_default("John Doe") // デフォルト値を設定
        .prompt(); // 入力を受け取る

    match name {
        Ok(name) => println!("Hello, {}!", name), // 成功した場合
        Err(e) => eprintln!("Error: {}", e),      // エラーの場合
    }
    loop {
        let mut items_ = List::new(items.clone());
        terminal.draw(|f| {
            let size = f.size();
            let block = Block::default().borders(Borders::ALL).title("Menu");
            f.render_widget(block, size);

            let list = items_.block(Block::default().borders(Borders::ALL));
            f.render_widget(list, size);
        })?;

        // キー入力の確認
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break; // "q"で終了
                }
            }
        }
    }

    Ok(())
}
