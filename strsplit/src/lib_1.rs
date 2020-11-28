//!
// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#[derive(Debug)]
pub struct StrSplit<'haystack, 'delimiter> {
    remainder: Option<&'haystack str>,
    delimiter: &'delimiter str, // not a String to support non-heap devices
}

// str -> [char]
// &str -> &[char]
// String -> Vec<char>
//
// String -> &str (cheap, uses AsRef)
// &str -> String <expensive, memcpy)

impl<'haystack, 'delimiter> StrSplit<'haystack, 'delimiter> {
    pub fn new(haystack: &'haystack str, delimiter: &'delimiter str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'haystack> Iterator for StrSplit<'haystack, '_>
// impl<'haystack, 'delimiter> Iterator for StrSplit<'haystack, 'delimiter>
// where
//     'delimiter: 'haystack,
// This can be used to give guarantee that the lifetimes are equivalent
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            //      Option<&mut &'a str>    Option<&'a str>
            //      ref --> take a reference to ..
            if let Some(next_delim) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delim];
                *remainder = &remainder[(next_delim + self.delimiter.len())..];
                // if we do --
                //          remainder = &remainder[(next_delim + self.delimiter.len())..];
                // Here,    &mut &'a str   &'a str
                // Hence, type mismatch. Need to dereference using *
                Some(until_delimiter)
            } else {
                self.remainder.take()
                // take -- takes value from var and sets original reference to None
                // impl<T> Option<T> { fn take(&mut self) -> Option<T>}
                // (got above syntax from JonHoo video - to verify)
            }
        } else {
            None
        }
    }
}

fn until_char(s: &str, c: char) -> &'_ str {
    //                              here, compiler can infer the lifetime from 's'
    StrSplit::new(s, &format!("{}", c))
        // Here, c is a temporary var with lifetime of until_char fn
        // Since we have multiple lifetimes for haystack + delimiter,
        // compiler does NOT have to SHORTEN the lifetime of haystack if delimiter is temporary
        .next()
        .expect("StrSplit always gives atleast one result")
}

#[test]
fn until_char_test() {
    assert_eq!(until_char("hello world!", 'o'), "hell");
}

#[test]
fn it_works() {
    let haystack = "a b c d e";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", "e"].into_iter()));
}

#[test]
fn tail_test() {
    let haystack = "a b c d ";
    let letters = StrSplit::new(haystack, " ");
    assert!(letters.eq(vec!["a", "b", "c", "d", ""].into_iter()));
}
