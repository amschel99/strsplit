#[warn(missing_docs, missing_debug_implementations)]

/// Generally, when you are creating a struct that has string slices, you have to assign lifetimes.
/// A lifetime just says how long something's going to live before it's dropped.
///
/// ```rust
/// pub struct Strsplit<'a> {
///     remainder: Option<&'a str>,
///     delimiter: &'a str,
/// }
/// ```
///
///
/// When users pass the argurments to new, we want to ensure that atleast what they are passing have live atleast as long as Strsplit because we are returning Strsplit and if
/// what we are passing does not live longer, then Strsplit will have remainder and delimiter pointing to dropped values.
/// If we wrote the new method like below, it wouldn't work because we are returning Strsplit which has it's own lifetime and the argurments to new have their own lifetime which the compiler cannot guess and there is a possibility
/// that the lifetime of Strsplit might outlive the lifetimes of the 2 parameters. Hence the returned Strsplit type might have pointers to dropped values.
///
/// ```rust
///  impl Strsplit <_>{
/// fn new (haystack:&str, delimiter:&str)->Self{
/// Self {
/// remainder:Some(haystack),
/// delimiter
/// }
/// }
/// }
/// ```
/// A good implementation looks like below:
/// ```rust
/// impl<'a> Strsplit<'a> {
/// fn new(haystack: &'a str, delimiter: &'a str) -> Self {
///   Self {
///      remainder: Some(haystack),
///     delimiter,
/// }
///}
/// }
/// ```
/// That's how the impl block of the struct has been done.
///
///
///
///

pub struct Strsplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

impl<'a> Strsplit<'a> {
    fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for Strsplit<'a> {
    type Item = &'a str;
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

#[test]
fn it_works() {
    let haystack = "a,b,c,d,e,f";
    let letters: Vec<_> = Strsplit::new(&haystack, ",").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e", "f"]);
}
