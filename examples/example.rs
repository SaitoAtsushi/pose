use pose::*;

fn main() -> Result<(), PoseError> {
    // parse a expression
    println!("{}", "(1 a \"b\" )".parse::<PoseType>()?);

    // parse multiple expressions.
    let testcase = "(1 a \"b\") ;commnet\n+ 1.0 1.2e+3";
    for item in Pose::new(testcase.chars()) {
        println!("{}", item?);
    }
    Ok(())
}
