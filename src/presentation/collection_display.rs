use ansi_term::Colour::Green;
use std::fmt::Display;

pub struct CollectionDisplay<T>
where
    T: Display,
{
    collection: Vec<T>,
}

impl<T> CollectionDisplay<T>
where
    T: Display,
{
    pub fn new(collection: Vec<T>) -> Self {
        CollectionDisplay {
            collection,
        }
    }
}

// Similar behavior to intersperse, but without using nightly
// https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.intersperse
impl<T> Display for CollectionDisplay<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, elem) in self.collection.iter().enumerate() {
            write!(f, "{} {}", Green.paint("•"), elem)?;

            if i != self.collection.len() - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}
