#[allow(unused_imports)]
use ellipse::Ellipse;

pub fn get_column_string(text: &str, width: usize) -> String {
    match text.len().cmp(&width) {
        // If text length is exactly the specified width, return text as is.
        std::cmp::Ordering::Equal => text.to_owned(),

        // If text length is shorter than the specified width, need to right-pad text with whitespaces.
        std::cmp::Ordering::Less => {
            format!("{:<width$}", text)
        }

        // If text length is longer than the specified with, then the result depends on width.
        // Note: I don't think truncate_ellipse helped much.  Here's an equivalent solution without using it.
        std::cmp::Ordering::Greater => match width {
            0 => "".to_owned(),
            1 => ".".to_owned(),
            2 => "..".to_owned(),
            3 => "...".to_owned(),
            // _ => text.truncate_ellipse(width - 3).to_string(),
            _ => {
                let left_substr = &text[..(width - 3)];
                format!("{}...", left_substr)
            }
        },
    }
}

#[test]
fn test_print_get_column_string() {
    let text = "0123456789"; // text.len() is 10

    for width in 0..=20 {
        println!(
            "width: {:2}, text: |{}|",
            width,
            get_column_string(text, width)
        );
    }
}

/// From trial and error, it looks like text.truncate_ellipse(n) does the following:
///   n = 0             =>  Return empty string
///   n < text.len()    =>  Keep first n characters, followed by "..." (This can create a longer string than text!)
///   n >= text.len()   =>  Return text as is
#[test]
fn test_print_truncate_ellipse() {
    let text = "0123456789"; // text.len() is 10

    for n in 0..=20 {
        println!("n: {:2}, text: |{}|", n, text.truncate_ellipse(n),);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_column_string() {
        let text1 = "";
        let text2 = "test";
        let text3 = "testme";
        let text4 = "testmetest";

        let width = 0;

        assert_eq!(get_column_string(text4, width), "".to_owned());

        let width = 1;

        assert_eq!(get_column_string(text4, width), ".".to_owned());

        let width = 2;

        assert_eq!(get_column_string(text4, width), "..".to_owned());

        let width = 3;

        assert_eq!(get_column_string(text4, width), "...".to_owned());

        let width = 4;

        assert_eq!(get_column_string(text4, width), "t...".to_owned());

        let width = 6;

        assert_eq!(get_column_string(text1, width), "      ".to_owned());
        assert_eq!(get_column_string(text2, width), "test  ".to_owned());
        assert_eq!(get_column_string(text3, width), "testme".to_owned());
        assert_eq!(get_column_string(text4, width), "tes...".to_owned());
    }
}
