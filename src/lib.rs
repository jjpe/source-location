//!

use std::fmt;
use serde::{Deserialize, Serialize};

#[macro_export]
macro_rules! location {
    () => {
        $crate::Location {
            file:   file!().to_string(),
            line:   line!(),
            column: column!(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[derive(Deserialize, Serialize)]
pub struct Location {
    pub file: String,
    pub line: u32,
    pub column: u32,
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.line, self.column)
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_location() {
        let Location { file, line, column } = Location::default();
        assert_eq!(  file, "");
        assert_eq!(  line,  0);
        assert_eq!(column,  0);
    }

    #[test]
    fn location_macro() {
        let Location { file, line, column } = location!();
        assert_eq!(  file, "src/lib.rs");
        assert_eq!(  line,  47);
        assert_eq!(column,  47);
    }
}
