#[warn(missing_docs, missing_debug_implementations)]

/// Strsplit struct.
/// Create an instance of Strsplit as below
/// ```rust
///  let haystack = "a,b,c,d,e,f";
/// let letters: Vec<_> = Strsplit::new(&haystack, ",").collect();
/// assert_eq!(letters, vec!["a", "b", "c", "d", "e", "f"]);
///
/// ```
///
#[derive(Debug)]
pub struct Strsplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str,
}

impl<'haystack, 'delimiter> Strsplit<'haystack, 'delimiter> {
    fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack, 'delimiter> Iterator for Strsplit<'haystack, 'delimiter> {
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        let remainder = self.remainder.as_mut()?;

        if let Some(next_delim) = remainder.find(self.delimiter) {
            let until_delimiter = &remainder[..next_delim];
            *remainder = &remainder[(next_delim + self.delimiter.len())..];
            Some(until_delimiter)
        } else {
            self.remainder.take()
        }
    }
}

/// Returns the string before the first instance of the delimiter is found
/// E.g
/// ```rust
///
///    let haystack = "hello";
/// let trimmed = until_char(&haystack, 'o');
/// assert_eq!(trimmed, "hell");
///
/// ```
///
pub fn until_char<'s>(s: &'s str, c: char) -> &'s str {
    Strsplit::new(s, c.to_string().as_str())
        .next()
        .expect("Gives atleast one result")
}
#[test]
fn it_works() {
    let haystack = "a,b,c,d,e,f";
    let letters: Vec<_> = Strsplit::new(&haystack, ",").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e", "f"]);
}
#[test]
fn until_char_works() {
    let haystack = "hello";

    let trimmed = until_char(&haystack, 'o');
    assert_eq!(trimmed, "hell");
}
