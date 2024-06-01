use std::fmt::Display;

use crate::vector::Vector;

impl Display for Vector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{} {} {}]",
            self.coords[0], self.coords[1], self.coords[2]
        )
    }
}
