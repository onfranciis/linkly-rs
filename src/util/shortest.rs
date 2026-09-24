// Get the first shortest value in a vector
pub fn get_shortest_value(data: Vec<&str>) -> &str {
    let mut value: &str = "";

    for i in data.into_iter() {
        if value.is_empty() {
            value = i;
        }

        if value.len() > i.len() {
            value = i;
        }
    }

    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_vector() {
        let input: Vec<&str> = vec![];
        assert_eq!(get_shortest_value(input), "");
    }

    #[test]
    fn test_single_element() {
        let input = vec!["single"];
        assert_eq!(get_shortest_value(input), "single");
    }

    #[test]
    fn test_varying_lengths() {
        let input = vec!["longest_word", "tiny", "medium"];
        assert_eq!(get_shortest_value(input), "tiny");
    }

    #[test]
    fn test_returns_first_shortest_on_tie() {
        let input = vec!["first", "second", "third", "one"];
        assert_eq!(get_shortest_value(input), "one");

        let ties = vec!["cat", "dog", "bat"];
        assert_eq!(get_shortest_value(ties), "cat");
    }

    #[test]
    fn test_uuid_split_segments() {
        // Simulates UUID v4 split on "-" as used in add_new_url.rs:
        // UUID structure: 8-4-4-4-12 hex digits
        let uuid_segments = vec!["c9a646d3", "9c61", "4cd9", "bf09", "24706be1ff9a"];
        assert_eq!(get_shortest_value(uuid_segments), "9c61");
    }
}
