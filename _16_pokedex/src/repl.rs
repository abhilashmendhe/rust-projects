pub fn clean_input(text: &str) -> Vec<String> {
    text
        .split(" ")
        .filter(|word| word.len() > 0)
        .map(|word| word.to_lowercase())
        .collect::<Vec<String>>()   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_input() {
        let result = clean_input("  hello  world  ");
        assert_eq!(result, vec!["hello".to_string(),"world".to_string()]);
    }
}