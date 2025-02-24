use std::io::stdin;

use my_parser_project::parser::Expression;

fn main() {
    for line in stdin().lines() {
        match line {
            Ok(base) => match Expression::new(&base) {
                Ok((i, e)) => match e.calc() {
                    Ok(ans) => {
                        let i = i.trim_start();
                        if i.len() > 0 {
                            println!("={} ({})", ans, i)
                        } else {
                            println!("={}", ans)
                        }
                    },
                    Err(e) => eprintln!("{}", e),
                },
                Err(e) => eprintln!("{}", e),
            },
            Err(e) => eprintln!("{}", e),
        }
    }
}
