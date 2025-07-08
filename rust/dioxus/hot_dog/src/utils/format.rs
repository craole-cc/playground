/// Capitalizes the first character of a string slice.
///
/// This function is UTF-8 safe as it operates on `char`s, not bytes,
/// preventing panics with multi-byte characters.
pub fn capitalize_first_letter(s: &str) -> String {
  let mut c = s.chars();
  match c.next() {
    None => String::new(),
    Some(f) => f.to_uppercase().collect::<String>() + c.as_str()
  }
}
