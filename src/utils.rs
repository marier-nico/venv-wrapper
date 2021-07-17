use std::fmt::Display;

pub fn intersperse<T, U>(displayables: T, separator: char) -> String
where
    T: Iterator<Item = U>,
    U: Display,
{
    let s = displayables.fold(String::new(), |mut acc, elm| {
        let elm = elm.to_string();
        acc.reserve(elm.len() + 1);
        acc.push_str(elm.as_str());
        acc.push(separator);
        acc
    });
    let s = s.trim_end();

    s.to_string()
}
