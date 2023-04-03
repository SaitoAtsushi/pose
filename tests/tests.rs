#![cfg(test)]

use pose::{Pose, PoseError, PoseError::*, PoseType, PoseType::*};

fn pose_str(s: &str) -> Result<PoseType, PoseError> {
    Ok(String(s.to_string()))
}

fn pose_sym(s: &str) -> Result<PoseType, PoseError> {
    Ok(Symbol(s.to_string()))
}

fn pose_num(s: f64) -> Result<PoseType, PoseError> {
    Ok(Number(s))
}

macro_rules! pose_list {
    ($($e:expr), *) => {
        pose::PoseType::List(vec![$($e),*])
    };
}

#[test]
fn parse_one() {
    let testcase = [
        (";abc\n\"abc\"", pose_str("abc")),
        (" :abc\n\"abc\"", pose_sym(":abc")),
        (" :@", InvalidSymbol.into()),
        ("ABC", InvalidFirstLetter.into()),
        ("\"a\\\"bc\"", pose_str("a\"bc")),
        ("\"a\\bc\"", Err(InvalidString)),
        ("  def;ghi", pose_sym("def")),
        (" jkl-mno123;comment", pose_sym("jkl-mno123")),
        ("0", pose_num(0.0)),
        ("1", pose_num(1.0)),
        ("-150.05", pose_num(-150.05)),
        ("-2.34", pose_num(-2.34)),
        ("-2.abc", Err(InvalidNumber)),
        ("-2.0E+12", pose_num(-2.0E+12)),
        ("-2.0e12", pose_num(-2.0e12)),
        ("-0abc", pose_num(-0.0)),
        ("+abc", pose_sym("+abc")),
        ("-abc", pose_sym("-abc")),
        ("+12", pose_sym("+")),
        (
            "(\"jkl\" \"mnf\")",
            Ok(pose_list![String("jkl".into()), String("mnf".into())]),
        ),
        ("(\"jkl\" \"mnf\"", Err(NothingClosingParenthesis)),
        (
            "(a (b c) d)",
            Ok(pose_list![
                Symbol("a".into()),
                pose_list![Symbol("b".into()), Symbol("c".into())],
                Symbol("d".into())
            ]),
        ),
        ("(-0123)", Ok(pose_list![Number(-0.0), Number(123.0)])),
        ("(+123)", Ok(pose_list![Symbol("+".into()), Number(123.0)])),
        (";nothing object", Ok(End)),
        ("@abc", Err(InvalidFirstLetter)),
    ];

    for (&ref s, expect_result) in &testcase {
        let mut parser = Pose::new(s.chars());
        let result = parser.read();
        assert_eq!(&result, expect_result);
    }
}

#[test]
fn parse_and_write() {
    let testcase = [
        ("abc", "abc"),
        (" ;comment\nabc ", "abc"),
        ("1.543", "1.543"),
        ("150.05", "150.05"),
        ("-150.05", "-150.05"),
        ("1.5e100", "1.5e100"),
        ("150000000000000", "150000000000000"),
        ("1500000000000000", "1.5e15"),
        ("-1500000000000000", "-1.5e15"),
        ("0.000000532", "5.32e-7"),
        ("\"ab\\\"c\"", "\"ab\\\"c\"")
    ];

    for (&ref s, &ref expected_result) in &testcase {
        assert_eq!(
            format!("{}", s.parse::<PoseType>().unwrap()),
            expected_result
        );
    }
}
