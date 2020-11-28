pub fn flatten<I>(iter: I) -> Flatten<I::IntoIter>
where
    I: IntoIterator,
    I::Item: IntoIterator,
{
    Flatten::new(iter.into_iter())
}

pub struct Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    outer: O,
    front_iter: Option<<O::Item as IntoIterator>::IntoIter>,
    back_iter: Option<<O::Item as IntoIterator>::IntoIter>,
}

impl<O> Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    fn new(iter: O) -> Self {
        Flatten {
            outer: iter,
            front_iter: None,
            back_iter: None,
        }
    }
}

impl<O> Iterator for Flatten<O>
where
    O: Iterator,
    O::Item: IntoIterator,
{
    type Item = <O::Item as IntoIterator>::Item;
    fn next(&mut self) -> Option<Self::Item> {
        // self.outer.next().and_then(|inner| inner.into_iter().next())
        // ^ Initial implementation

        loop {
            // Option 1
            // if let Some(ref mut inner_iter) = self.front_iter {
            // Option 2
            // if let Some(inner_iter) = self.front_iter.as_mut() {
            // Option 3
            if let Some(inner_iter) = &mut self.front_iter {
                // In both 1 & 2 & 3, we convert from
                // self.front_iter: Option<T> to
                // inner_iter: Option<&mut T>
                if let Some(i) = inner_iter.next() {
                    return Some(i);
                }
                self.front_iter = None;
            }

            // This code to be executed when first time next() is called
            // let next_inner_item = self.outer.next()?.into_iter();
            // self.front_iter = Some(next_inner_item);

            if let Some(next_inner) = self.outer.next() {
                self.front_iter = Some(next_inner.into_iter());
            } else {
                return self.back_iter.as_mut()?.next();
            }
        }
    }
}

impl<O> DoubleEndedIterator for Flatten<O>
where
    O: DoubleEndedIterator,
    O::Item: IntoIterator,
    <O::Item as IntoIterator>::IntoIter: DoubleEndedIterator,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        //                     here, we don't need to specify <Self as Iterator> since only
        //                     1-level deep
        loop {
            if let Some(ref mut inner_iter) = self.back_iter {
                if let Some(i) = inner_iter.next_back() {
                    return Some(i);
                }
                self.back_iter = None;
            }
            // let back_inner_item = self.outer.next_back()?.into_iter();
            // self.back_iter = Some(back_inner_item);
            if let Some(back_inner) = self.outer.next_back() {
                self.back_iter = Some(back_inner.into_iter());
            } else {
                return self.front_iter.as_mut()?.next_back();
                // Here, we do as_mut since we don't wanna move out it, just use as ref
            }
        }
    }
}

mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(flatten(std::iter::empty::<Vec<()>>()).count(), 0);
    }

    #[test]
    fn empty_wide() {
        assert_eq!(flatten(vec![Vec::<()>::new(), vec![], vec![]]).count(), 0);
    }

    #[test]
    fn once() {
        assert_eq!(flatten(std::iter::once(vec!["a"])).count(), 1);
    }

    #[test]
    fn two() {
        assert_eq!(flatten(std::iter::once(vec!["a", "b"])).count(), 2);
    }

    #[test]
    fn two_wide() {
        assert_eq!(flatten(vec![vec!["a"], vec!["b"]]).count(), 2);
    }

    #[test]
    fn reverse() {
        assert_eq!(
            flatten(std::iter::once(vec!["a", "b"]))
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn reverse_wide() {
        assert_eq!(
            flatten(vec![vec!["a"], vec!["b"]])
                .rev()
                .collect::<Vec<_>>(),
            vec!["b", "a"]
        );
    }

    #[test]
    fn both_ends() {
        let mut iter = flatten(vec![vec!["a", "b"], vec!["c", "d"]]);
        assert_eq!(iter.next(), Some("a"));
        assert_eq!(iter.next_back(), Some("d"));
        assert_eq!(iter.next(), Some("b"));
        assert_eq!(iter.next_back(), Some("c"));
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn inf() {
        let mut iter = flatten((0..).map(|i| 0..i));
        //                  This inf iterator is not double ended
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(0));
        assert_eq!(iter.next(), Some(1));
    }

    #[test]
    fn deep() {
        assert_eq!(flatten(flatten(vec![vec![vec![0, 1]]])).count(), 2);
    }
}
