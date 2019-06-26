// extern crate sassruist;

#[macro_use] // macroを使うのでmacro_useを追記
extern crate clap;
use clap::{Arg, SubCommand};

fn main() {
    let app = app_from_crate!()
        .arg(
            Arg::with_name("pa")
                .help("sample positional argument")
                .required(true),
        )
        .arg(
            Arg::with_name("flg") // フラグを定義
                .help("sample flag") // ヘルプメッセージ
                .short("f") // ショートコマンド
                .long("flag"), // ロングコマンド
        )
        .arg(
            Arg::with_name("opt") // オプションを定義
                .help("sample option") // ヘルプメッセージ
                .short("o") // ショートコマンド
                .long("opt") // ロングコマンド
                .takes_value(true), // 値を持つことを定義
        )
        .subcommand(
            SubCommand::with_name("sub") // サブコマンドを定義
                .about("sample subcommand") // このサブコマンドについて
                .arg(
                    Arg::with_name("subflg") // フラグを定義
                        .help("sample flag by sub") // ヘルプメッセージ
                        .short("f") // ショートコマンド
                        .long("flag"), // ロングコマンド
                ),
        );

    // 引数を解析
    let matches = app.get_matches();

    // paが指定されていれば値を表示
    if let Some(o) = matches.value_of("pa") {
        println!("Value for pa: {}", o);
    }

    // optが指定されていれば値を表示
    if let Some(o) = matches.value_of("opt") {
        println!("Value for opt: {}", o);
    }

    // flgのON/OFFで表示するメッセージを切り替え
    println!(
        "flg is {}",
        if matches.is_present("flg") {
            "ON"
        } else {
            "OFF"
        }
    );

    // subサブコマンドの解析結果を取得
    if let Some(ref matches) = matches.subcommand_matches("sub") {
        println!("used sub"); // subが指定されていればメッセージを表示
                              // subflgのON/OFFで表示するメッセージを切り替え
        println!(
            "subflg is {}",
            if matches.is_present("subflg") {
                "ON"
            } else {
                "OFF"
            }
        );
    }
}
