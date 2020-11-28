//!
// #![warn(missing_debug_implementations, rust_2018_idioms, missing_docs)]
#[derive(Debug)]
pub struct StrSplit<'haystack, D> {
    remainder: Option<&'haystack str>,
    delimiter: D,
}

impl<'haystack, D> StrSplit<'haystack, D> {
    pub fn new(haystack: &'haystack str, delimiter: D) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

pub trait Delimiter {
    fn find_next(&self, s: &str) -> Option<(usize, usize)>;
    // accepts string and returns address of start and end
}

impl Delimiter for &str {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.find(self).map(|start| (start, start + self.len()))
    }
}

// https://youtu.be/rAl-9HwD858?t=4852
impl Delimiter for char {
    fn find_next(&self, s: &str) -> Option<(usize, usize)> {
        s.char_indices()
            .find(|(_, c)| c == self)
            .map(|(start, _)| (start, start + self.len_utf8()))
        //                          len_utf8 is needed since usize works as byte indices
    }
}

impl<'haystack, D> Iterator for StrSplit<'haystack, D>
where
    D: Delimiter,
{
    type Item = &'haystack str;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(ref mut remainder) = self.remainder {
            //      Option<&mut &'a str>    Option<&'a str>
            //      ref --> take a reference to ..
            if let Some((delim_start, delim_end)) = self.delimiter.find_next(remainder) {
                let until_delimiter = &remainder[..delim_start];
                *remainder = &remainder[delim_end..];
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
    StrSplit::new(s, c)
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
