extern crate sassruist;
use sassruist::convert_file;
use std::fs;

#[test]
fn test_1_sass() {
    let text = convert_file("./tests/demo_1.sass").unwrap();
    let expected = fs::read_to_string("./tests/demo_1_expected.sass").unwrap();
    assert_eq!(text, expected);
}

// #[test]
// fn test_1_scss() {
//     let text = convert_file("./tests/demo_1.scss").unwrap();
//     let expected = fs::read_to_string("./tests/demo_1_expected.scss").unwrap();
//     assert_eq!(text, expected);
// }
