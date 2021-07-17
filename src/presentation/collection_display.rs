use ansi_term::Colour::Green;
use std::fmt::Display;

use crate::utils::intersperse;

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

impl<T> Display for CollectionDisplay<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let with_bullet = self.collection.iter().map(|e| format!("{} {}", Green.paint("•"), e));
        write!(f, "{}", intersperse(with_bullet, '\n'))?;

        Ok(())
    }
}
