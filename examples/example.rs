use pose::*;

fn main() {
    let testcase = "(1 a \"bc\") ;commnet\n+ 1.0 1.2e+3";
    let mut parser = Pose::new(testcase.chars());
    while {
        match parser.read() {
            Ok(PoseType::End) => false,
            Ok(item) => {
                println!("{:?}", item);
                true
            }
            Err(e) => {
                println!("{:?}", e);
                false
            }
        }
    } {}
}
