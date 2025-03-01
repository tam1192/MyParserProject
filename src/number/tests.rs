use super::*;

#[test]
fn test1() {
    let x = Number::Int(1);
    let y = Number::Int(1);
    assert_eq!(x + y, Number::Int(2))
}

#[test]
fn test2() {
    let x = Number::Int(2);
    let y = Number::Int(1);
    assert_eq!(x - y, Number::Int(1))
}

#[test]
fn test3() {
    let x = Number::Int(2);
    let y = Number::Int(3);
    assert_eq!(x * y, Number::Int(6))
}

#[test]
fn test4() {
    let x = Number::Int(2);
    let y = Number::Int(2);
    assert_eq!(x / y, Ok(Number::Float(1.0)))
}

#[test]
fn test5() {
    let x = Number::Int(1);
    let y = Number::Float(1.0);
    assert_eq!(x + y, Number::Float(2.0))
}

#[test]
fn test6() {
    let x = Number::Float(2.0);
    let y = Number::Int(1);
    assert_eq!(x - y, Number::Float(1.0))
}

#[test]
fn test7() {
    let x = Number::Float(2.0);
    let y = Number::Float(3.0);
    assert_eq!(x * y, Number::Float(6.0))
}

#[test]
fn test8() {
    let x = Number::Int(2);
    let y = Number::Int(-1);
    assert_eq!(x.pow(y), Number::Float(0.5))
}
