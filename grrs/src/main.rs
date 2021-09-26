/* 
 * 概要:
 * grep するコマンド
 *
 * 補足:
 * 1. StructOptの機能により、構造体のドキュメントコメントからコマンドのhelpが生成される。
 * 2. anyhowの機能により、エラーハンドリング・メッセージを簡潔に。
 */
use structopt::StructOpt;
use anyhow::{Context, Result};

fn main() -> Result<()> {
    // コマンドライン引数の取得
    let args = CommandLineInterface::from_args();

    /*
     * ファイルの取得
     *
     * TODO BufReaderへの置き換え(https://doc.rust-lang.org/1.39.0/std/io/struct.BufReader.html)
     */
    // もっとも原始的なエラーハンドリングの実装。
    // let result = std::fs::read_to_string(&args.path);
    // let content = match result {
    //     Ok(content) => { content },
    //     Err(error) => { panic!("Can't deal with {}, just exit here.", error); }
    // };

    // anyhow を使った場合のエラーハンドリングの実装。
    let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("could not read file {:?}", &args.path))?;

    // 各行の取得
    for line in content.lines() {
        // パターン（検索文字列）が含まれるか
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    // 正常終了
    Ok(())
}

/// 指定したファイルから、特定の文字列を検索する。
#[derive(StructOpt)]
struct CommandLineInterface { 
    /// 探したい文字列
    pattern: String,
    /// 検索したいファイルのパス
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}
