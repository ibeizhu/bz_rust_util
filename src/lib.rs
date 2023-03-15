//! # BZ Rust Util
//!
//! `bz_rust_util` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// hahahahahahhahah

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = bz_rust_util::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub mod utils {
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = bz_rust_util::add_one2(arg);
    ///
    /// assert_eq!(7, answer);
    /// ```
    pub fn add_one2(x: i32) -> i32 {
        x + 2
    }
}

pub enum PrimaryColor {
    Red,
    Yellow,
    Blue,
}

pub mod kinds {
    /// The primary colors according to the RYB color model.

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(5);
        assert_eq!(result, 6);
    }
    #[test]
    fn it_works1() {
        let result = utils::add_one2(5);
        assert_eq!(result, 7);
    }
}
