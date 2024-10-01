use std::fmt::Debug;
use std::fs;

pub trait AssertSameAsInFile {
    fn assert_same_as_in_file(&self, expected_output_file_name: &str);
}

impl<T: Debug> AssertSameAsInFile for T {
    fn assert_same_as_in_file(&self, expected_output_file_name: &str) {
        let content =
            fs::read_to_string(format!("tests/expected_output/{expected_output_file_name}"))
                .unwrap();

        assert_eq!(format!("{self:#?}"), content);
    }
}
