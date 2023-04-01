use pose::*;

fn test_parse(testcase: &str, expect_result: Result<PoseType, PoseError>) {
    let mut parser = Pose::new(testcase.chars());
    let result = parser.read();
    assert_eq!(result, expect_result);
}

#[test]
fn it_works() {
    test_parse(";abc\n\"abc\"", Ok(PoseType::String("abc".into())));
    test_parse(" :abc\n\"abc\"", Ok(PoseType::Symbol(":abc".into())));
    test_parse(" :@", Err(PoseError::InvalidSymbol));
    test_parse("ABC", Err(PoseError::InvalidFirstLetter));
    test_parse("\"a\\\"bc\"", Ok(PoseType::String("a\"bc".into())));
    test_parse("\"a\\bc\"", Err(PoseError::InvalidString));
    test_parse("  def;ghi", Ok(PoseType::Symbol("def".into())));
    test_parse(
        " jkl-mno123;comment",
        Ok(PoseType::Symbol("jkl-mno123".into())),
    );
    test_parse("0", Ok(PoseType::Number(0.0)));
    test_parse("1", Ok(PoseType::Number(1.0)));
    test_parse("-150.05", Ok(PoseType::Number(-150.05)));
    test_parse("-2.34", Ok(PoseType::Number(-2.34)));
    test_parse("-2.abc", Err(PoseError::InvalidNumber));
    test_parse("-2.0E+12", Ok(PoseType::Number(-2000000000000.0)));
    test_parse("-2.0e12", Ok(PoseType::Number(-2000000000000.0)));
    test_parse("-0abc", Ok(PoseType::Number(-0.0)));
    test_parse("+abc", Ok(PoseType::Symbol("+abc".into())));
    test_parse("-abc", Ok(PoseType::Symbol("-abc".into())));
    test_parse("+12", Ok(PoseType::Symbol("+".into())));
    test_parse(
        "(\"jkl\" \"mnf\")",
        Ok(PoseType::List(vec![
            PoseType::String("jkl".into()),
            PoseType::String("mnf".into()),
        ])),
    );
    test_parse(
        "(\"jkl\" \"mnf\"",
        Err(PoseError::NothingClosingParenthesis),
    );
    test_parse(
        "(a (b c) d)",
        Ok(PoseType::List(vec![
            PoseType::Symbol("a".into()),
            PoseType::List(vec![
                PoseType::Symbol("b".into()),
                PoseType::Symbol("c".into()),
            ]),
            PoseType::Symbol("d".into()),
        ])),
    );
    test_parse(
        "(-0123)",
        Ok(PoseType::List(vec![
            PoseType::Number(-0.0),
            PoseType::Number(123.0),
        ])),
    );
    test_parse(
        "(+123)",
        Ok(PoseType::List(vec![
            PoseType::Symbol("+".into()),
            PoseType::Number(123.0),
        ])),
    );
    test_parse(";nothing object", Ok(PoseType::End));
    test_parse("@abc", Err(PoseError::InvalidFirstLetter));

    assert_eq!("abc".parse(), Ok(PoseType::Symbol("abc".into())));
    assert_eq!("abc n".parse::<PoseType>(), Err(PoseError::InvalidEnd));

    assert_eq!(
        format!("{}", " ;comment\nabc ".parse::<PoseType>().unwrap()),
        "abc"
    );
    assert_eq!(format!("{}", "1.543".parse::<PoseType>().unwrap()), "1.543");
    assert_eq!(
        format!("{}", "150.05".parse::<PoseType>().unwrap()),
        "150.05"
    );
    assert_eq!(
        format!("{}", "-150.05".parse::<PoseType>().unwrap()),
        "-150.05"
    );
    assert_eq!(
        format!("{}", "1.5e100".parse::<PoseType>().unwrap()),
        "1.5e100"
    );
    assert_eq!(
        format!("{}", "150000000000000".parse::<PoseType>().unwrap()),
        "150000000000000"
    );
    assert_eq!(
        format!("{}", "1500000000000000".parse::<PoseType>().unwrap()),
        "1.5e15"
    );
    assert_eq!(
        format!("{}", "-1500000000000000".parse::<PoseType>().unwrap()),
        "-1.5e15"
    );
    assert_eq!(
        format!("{}", "0.000000532".parse::<PoseType>().unwrap()),
        "5.32e-7"
    );
}
