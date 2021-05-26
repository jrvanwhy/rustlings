// quiz4.rs
// This quiz covers the sections:
// - Modules
// - Macros

// Write a macro that passes the quiz! No hints this time, you can do it!

macro_rules! my_macro {
    ($end:expr) => {
        format!("Hello {}", $end)
        //{
        //	let mut phrase = "Hello ".to_string();
        //	phrase.push_str($end);
        //	phrase
        //}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_macro_world() {
        assert_eq!(my_macro!("world!"), "Hello world!");
    }

    #[test]
    fn test_my_macro_goodbye() {
        assert_eq!(my_macro!("goodbye!"), "Hello goodbye!");
    }
}
