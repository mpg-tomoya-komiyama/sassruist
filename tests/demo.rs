// extern crate sassruist;
// use std::fs;

// use sassruist::converter;

// #[test]
// fn test_1_sass() {
//     let text = fs::read_to_string("./tests/demo_1.sass").unwrap();
//     let expected = fs::read_to_string("./tests/demo_1_expected.sass").unwrap();
//     assert_eq!(converter::perform(&text), expected);
// }

// #[test]
// fn test_1_scss() {
//     let text = fs::read_to_string("./tests/demo_1.scss").unwrap();
//     let expected = fs::read_to_string("./tests/demo_1_expected.scss").unwrap();
//     assert_eq!(converter::perform(&text), expected);
// }

// #[test]
// fn test_2_scss() {
//     let text = fs::read_to_string("./tests/demo_2.scss").unwrap();
//     let expected = fs::read_to_string("./tests/demo_2_expected.scss").unwrap();
//     assert_eq!(converter::perform(&text), expected);
// }

// #[test]
// fn test_3_scss() {
//     let text = fs::read_to_string("./tests/demo_3.scss").unwrap();
//     let expected = fs::read_to_string("./tests/demo_3_expected.scss").unwrap();
//     assert_eq!(converter::perform(&text), expected);
// }
