use unicode_segmentation::UnicodeSegmentation;

pub fn check_length(value: &String, max_length: usize) -> Result<(), String> {
    if value.trim().is_empty() {
        return Err("must not be empty".into());
    }

    // Graphemes  perceived as a single character but it is combined of
    // two characters. For example a french letter with an accent + an english
    // letter
    if value.graphemes(true).count() > max_length {
        return Err(format!("must be less than {} characters long", max_length));
    }

    // Forbidden characters?

    Ok(())
}
