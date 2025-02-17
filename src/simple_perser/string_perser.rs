type Output<'a> = (Option<&'a str>, &'a str);

fn ex<'a>(input: &'a str, pattern: &str) -> Output<'a> 
{
    assert!(pattern.len() > 0);
    assert!(input.len() > 0);
    match input.find(pattern) {
        Some(len) => (Some(&input[..len]), &input[len+1..]),
        None => (None, input),
    }
}

#[cfg(test)]
mod tests_ex {
    use super::ex;
    #[test]
    fn test1() {
        // ' 'で区切る場合
        let input = "hello world !!!";
        let output =  (Some("hello"), "world !!!");
        let seperated = " ";
        assert_eq!(ex(input, seperated), output);
    }
    
    #[test]
    fn test2() {
        // 区切るものがない場合
        let input = "helloworld!!!";
        let output =  (None, input);
        let seperated = " ";
        assert_eq!(ex(input, seperated), output);
    }

    #[test]
    fn test3() {
        // 連続して区切る場合
        let data = "red,blue,gre en";
        let pattern = ",";

        let (output, data) = ex(data, pattern);
        assert_eq!(output, Some("red"));

        let (output, data) = ex(data, pattern);
        assert_eq!(output, Some("blue"));

        let (output, data) = ex(data, pattern);
        assert_eq!(output, None);
        assert_eq!(data, "gre en");
    }

    #[test]
    #[should_panic]
    fn test4() {
        // 区切り文字が空白の場合
        let input = "helloworld!!!";
        let seperated = "";
        ex(input, seperated);
    }

    #[test]
    #[should_panic]
    fn test5() {
        // 入力が空白の場合
        let input = "";
        let seperated = " ";
        ex(input, seperated);
    }

    #[test]
    #[should_panic]
    fn test6() {
        // なにもかも空白の場合
        ex("", "");
    }
}