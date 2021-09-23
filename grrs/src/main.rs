/* 
 * 概要:
 * grep するコマンド
 *
 * 補足:
 * 1. StructOptの機能により、構造体のドキュメントコメントからコマンドのhelpが生成される。
 */
use structopt::StructOpt;

fn main() {
    // コマンドライン引数の取得
    let args = CommandLineInterface::from_args();

    // ファイルの取得
    // TODO BufReaderへの置き換え(https://doc.rust-lang.org/1.39.0/std/io/struct.BufReader.html)
    let content = std::fs::read_to_string(&args.path)
    .expect("could not read file");

    // 各行の取得
    for line in content.lines() {
        // パターン（検索文字列）が含まれるか
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
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