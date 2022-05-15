use std::fmt;
use std::fmt::{Display, Formatter};

pub struct Solution<T>
where
    T: Display,
{
    pub data: Vec<Vec<Option<T>>>,
}

impl<T> Display for Solution<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut formatted = String::from("");

        for row in &self.data {
            for elem in row {
                match elem {
                    Some(x) => formatted += &x.to_string(),
                    _ => formatted += "x",
                };
            }
            formatted += "\n";
        }

        write!(f, "{}", formatted)
    }
}
