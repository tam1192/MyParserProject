use super::error::{Error, ErrorKind};
use super::Number::{Float, Int};

#[test]
fn test1() {
    // 整数同士の足し算
    let x = Int(1);
    let y = Int(1);
    assert_eq!(x + y, Int(2))
}

#[test]
fn test2() {
    // 整数同士の引き算
    let x = Int(2);
    let y = Int(1);
    assert_eq!(x - y, Int(1))
}

#[test]
fn test3() {
    // 整数同士の掛け算
    let x = Int(2);
    let y = Int(3);
    assert_eq!(x * y, Int(6))
}

#[test]
fn test4() {
    // 整数同士の割り算
    let x = Int(2);
    let y = Int(2);
    assert_eq!(x / y, Ok(Float(1.0)))
}

#[test]
fn test5() {
    // 整数と小数の足し算
    let x = Int(1);
    let y = Float(1.0);
    assert_eq!(x + y, Float(2.0))
}

#[test]
fn test6() {
    // 整数と小数の引き算
    let x = Float(2.0);
    let y = Int(1);
    assert_eq!(x - y, Float(1.0))
}

#[test]
fn test7() {
    // 整数と小数の掛け算
    let x = Float(2.0);
    let y = Float(3.0);
    assert_eq!(x * y, Float(6.0))
}

#[test]
fn test8() {
    // 整数と小数の割り算
    let x = Int(3);
    let y = Float(1.0);
    assert_eq!(x / y, Ok(Float(3.0)))
}

#[test]
fn test9() {
    // 累乗
    let x = Int(2);
    let y = Int(-1);
    assert_eq!(x.pow(y), Float(0.5))
}

#[test]
fn test10() {
    // ぜろ除算
    let x = vec![Int(4), Int(0), Float(3.0), Float(3.14), Float(0.0)];
    let i_zero = Int(0);
    let f_zero = Float(0.0);
    let error = Err(Error::from(ErrorKind::ZeroDiv));
    for x in x {
        assert_eq!(x / i_zero, error);
        assert_eq!(x / f_zero, error);
    }
}
