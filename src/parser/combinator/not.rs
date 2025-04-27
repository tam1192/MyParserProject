use super::*;

/// [Parser]を成功させるまでに使った入力を取る
///
/// 戻り値が[Result]の[Parser]を、成功するまで入力をずらしながら試し、ずらした分の入力を結果として返します。
///
/// # Example
/// ```
/// use my_parser_project::parser::{combinator::*, str_parser::*};
///  
/// let base = "abc123";
/// let parser = not(num);
/// let (remain, result) = parser(base);
///
/// assert_eq!(result, "abc");
/// assert_eq!(remain, "123");
/// ```
///
pub fn not<'a, T, E, P: Parser<&'a str, Result<T, E>>>(p: P) -> impl Parser<&'a str, &'a str> {
    move |i| {
        for first in 0..i.len() {
            if let (_, Ok(_)) = p(&i[first..]) {
                return (&i[first..], &i[..first]);
            }
        }
        ("", &i)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::str_parser::*;

    #[test]
    fn char_parser() {
        let patterns = [
            ("abc+", ("+", "abc")),
            ("+abc", ("+abc", "")),
            ("+", ("+", "")),
            ("abc", ("", "abc")),
        ];
        let parser = not(char('+'));
        for (input, result) in patterns {
            assert_eq!(parser(input), result);
        }
    }

    #[test]
    fn num_parser() {
        let patterns = [
            ("abc123", ("123", "abc")),
            ("123abc", ("123abc", "")),
            ("123", ("123", "")),
            ("abc", ("", "abc")),
        ];
        let parser = not(num);
        for (input, result) in patterns {
            assert_eq!(parser(input), result);
        }
    }

    #[test]
    fn string_parser() {
        let patterns = [
            ("123abc", ("abc", "123")),
            ("abc123", ("abc123", "")),
            ("abc", ("abc", "")),
            ("123", ("", "123")),
        ];
        let parser = not(string(String::from("abc")));
        for (input, result) in patterns {
            assert_eq!(parser(input), result);
        }
    }

    #[test]
    fn simple() {
        let input = "abc123";
        let parser = not(num);
        let (remain, result) = parser(input);
        assert_eq!(result, "abc");
        assert_eq!(remain, "123");
    }

    // 入力パーサーがすぐに成功しちゃう例
    #[test]
    fn zero_length_remain() {
        let input = "123";
        let parser = not(num);
        let (remain, result) = parser(input);
        assert_eq!(result, "");
        assert_eq!(remain, "123");
    }

    // 入力パーサーがすべて失敗する例
    #[test]
    fn input_parser_always_fail() {
        let input = "abc";
        let parser = not(num);
        let (remain, result) = parser(input);
        println!("{}", result);
        assert_eq!(result, "abc");
        assert_eq!(remain, "");
    }
}
