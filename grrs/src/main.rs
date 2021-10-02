/* 
 * 概要:
 * grep するコマンド
 *
 * 補足:
 * 1. StructOptの機能により、構造体のドキュメントコメントからコマンドのhelpが生成される。
 * 2. anyhowの機能により、エラーハンドリング・メッセージを簡潔に。
 * 3. 時間のかかる処理があり、インジケータを使いたい場合は下記サンプル参照。
 *    https://github.com/mitsuhiko/indicatif/tree/main/examples
 */
use structopt::StructOpt;
use anyhow::{Context, Result};
use std::io;
use log::{info, warn};
use std::env;

fn main() -> Result<()> {
    // コマンドライン引数の取得
    let args = CommandLineInterface::from_args();

    // ロギングの設定
    if args.verbose {
        env::set_var("RUST_LOG", "info");
    }
    env_logger::init();
    info!("starting up");

    /*
     * ファイルの取得
     *
     * TODO BufReaderへの置き換え(https://doc.rust-lang.org/1.39.0/std/io/struct.BufReader.html)
     *
     * 補足:
     * 1. anyhow を使ってエラー出力が実装されている
     * 下記は原始的（？）なエラー出力の実装。
     * let result = std::fs::read_to_string(&args.path);
     * let content = match result {
     *     Ok(content) => { content },
     *     Err(error) => { panic!("Can't deal with {}, just exit here.", error); }
     * };
     *
     * 2. fmtについて
     * {}は、Deisplayを出力するが、
     * {:?}は、Debug（[derive(Debug)]）を出力する。
     * fmtモジュール参照 https://doc.rust-lang.org/1.39.0/std/fmt/index.html
     */
    let content = std::fs::read_to_string(&args.path)
    .with_context(|| format!("could not read file {:?}", &args.path))?;

    // ファイルの中身が空だったら、検索処理をせず正常終了
    if content.is_empty() {
        warn!("target file is empty");
        println!("no match");
    } else {
        let stdout = io::stdout();
        let mut writer = io::BufWriter::new(stdout.lock());
        find_matches(&content, &args.pattern, &mut writer)?;
    }

    Ok(())
}

/**
contentのlineごとに、一致するpatternを標準出力に表示する。

一致するものがない場合はその旨を表示する。
*/
fn find_matches(content: &str, pattern: &str, mut writer: impl io::Write) -> Result<()> {
    /*
     * 各行の取得・比較・マッチするものを表示。
     *
     * 補足：
     * 1. printlnはループなどで呼び出すと遅い。
     * ・毎回ロックされるから→直接stdoutを取得してロックの回数を１度だけに減らす
     * ・毎回flushされるから→BufWriterを利用してバッファリングし回数を減らす
     */
    let mut is_matched = false;
    for line in content.lines() {
        // パターン（検索文字列）が含まれるか
        if line.contains(pattern) {
            writeln!(writer, "{}", line)
            .with_context(|| "Couldn't write to stdout.")?;
            is_matched = true;
        }
    }

    // 検索で一致するものがなかった
    if !is_matched {
        writeln!(writer, "no match")
        .with_context(|| "Couldn't write to stdout.")?;
    }

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
    /// switch on verbosity
    #[structopt(short)]
    verbose: bool,
}

#[test]
fn case_matched() -> Result<()> {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result)?;
    assert_eq!(result, b"lorem ipsum\n");
    Ok(())
}

#[test]
fn case_no_match() -> Result<()> {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "test", &mut result)?;
    assert_eq!(result, b"no match\n");
    Ok(())
}