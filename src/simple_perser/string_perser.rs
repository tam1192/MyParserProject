type Output<'a> = (Option<&'a str>, &'a str);

/// 文字列を指定したパターンで分割する
fn split_string<'a>(input: &'a str, pattern: char) -> Output<'a> 
{
    assert!(input.len() > 0);
    match input.find(pattern) {
        Some(len) => (Some(&input[..len]), &input[len+1..]),
        None => (None, input),
    }
}

#[cfg(test)]
mod tests_ex {
    use super::split_string;
    #[test]
    fn test1() {
        // ' 'で区切る場合
        let input = "hello world !!!";
        let output =  (Some("hello"), "world !!!");
        let seperated = ' ';
        assert_eq!(split_string(input, seperated), output);
    }
    
    #[test]
    fn test2() {
        // 区切るものがない場合
        let input = "helloworld!!!";
        let output =  (None, input);
        let seperated = ' ';
        assert_eq!(split_string(input, seperated), output);
    }

    #[test]
    fn test3() {
        // 連続して区切る場合
        let data = "red,blue,gre en";
        let pattern = ',';

        let (output, data) = split_string(data, pattern);
        assert_eq!(output, Some("red"));

        let (output, data) = split_string(data, pattern);
        assert_eq!(output, Some("blue"));

        let (output, data) = split_string(data, pattern);
        assert_eq!(output, None);
        assert_eq!(data, "gre en");
    }

    #[test]
    #[should_panic]
    fn test4() {
        // 入力が空白の場合
        let input = "";
        let seperated = ' ';
        split_string(input, seperated);
    }
}