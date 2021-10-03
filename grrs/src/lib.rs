use std::io;
use anyhow::{Context, Result};

/// contentのlineごとに、一致するpatternを標準出力に表示する。
pub fn find_matches(content: &str, pattern: &str, mut writer: impl io::Write) -> Result<bool> {
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

    Ok(is_matched)
}

/// 一致する文字列がある
#[test]
fn case_matched() -> Result<()> {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result)?;
    assert_eq!(result, b"lorem ipsum\n");
    Ok(())
}

/// 一致する文字列がない
#[test]
fn case_no_match() -> Result<()> {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "test", &mut result)?;
    assert_eq!(result, b"");
    Ok(())
}