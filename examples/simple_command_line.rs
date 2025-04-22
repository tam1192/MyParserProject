use my_parser_project::parser::{
    combinator::{Concat, Map, SubResult, Substitute},
    str_parser::{char, string, trimer},
};
use std::io::stdin;
fn main() {
    // 標準入力を受け付ける
    let stdin = stdin();

    loop {
        let mut input = String::new();
        _ = stdin.read_line(&mut input);

        // 入力から命令を取り出す
        let (input, opcode) = trimer.cat_b(
            // say <msg> : msgをユーザーに投げる
            string("say".to_string())
                // exit : プログラムを終了させる
                .sub(string("exit".to_string()))
                // ping ; pong!とユーザーに投げる
                .sub(string("ping".to_string()))
                .map(|r| match r {
                    Ok(SubResult::A(SubResult::A(_))) => 1,
                    Ok(SubResult::A(SubResult::B(_))) => 2,
                    Ok(SubResult::B(_)) => 3,
                    Err(_) => 0,
                }),
        )(&input);

        match opcode {
            1 => {
                let (msg, _) = char(' ')(input);
                println!("{}", msg);
            }
            2 => {
                println!("bye");
                break;
            }
            3 => {
                println!("pong!")
            }
            _ => println!(""),
        }
    }
}
