// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a hint.

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    // let mut result: Vec<String> = vec![];
    // let words_iter = words.iter();
    // for word in words_iter {
    //     result.push(capitalize_first(word));
    // }
    // result

    // Better Solution :-

    let capitalize_first_words: Vec<String> =
        words.into_iter().map(|w| capitalize_first(w)).collect();
    capitalize_first_words
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    // let mut single_word = String::new();
    // let sep_cap_words = capitalize_words_vector(words);
    // let sep_cap_words_iter = sep_cap_words.iter();
    // for word in sep_cap_words_iter {
    //     single_word.push_str(word);
    // }
    // single_word

    // Better Solution :-
    let single_word: String = words
        .into_iter()
        .map(|w| capitalize_first(w))
        .collect::<String>();
    single_word
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
