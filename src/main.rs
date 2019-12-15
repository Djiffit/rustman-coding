use std::collections::HashMap;

fn main() {
    let word = "abcdefg
efg";


    huffman_encode(word.to_string());
}

fn get_counts(text: String) -> (HashMap<char, u32>, u32) {
    let mut total: u32 = 0;
    let mut counts: HashMap<char, u32> = HashMap::new();
    for c in text.chars() {
        match counts.get(&c) {
            Some(review) => counts.insert(c, review + 1),
            None => counts.insert(c, 1),
        };
        total += 1
    }
    return (counts, total);
}

fn huffman_encode(text: String) {

    let (counts, total) = get_counts(text);

    println!("{:?} {}", counts, total);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CountTest {
        text: String,
        results: Vec<(char, u32)>,
    }

    #[test]
    fn test_get_counts() {

        let tests = vec![
            CountTest{
                text: "a".to_string(),
                results: vec![
                    ('a', 1),
                ],
            },
            CountTest{
                text: "aaaabbbcccaaaa".to_string(),
                results: vec![
                    ('a', 8),
                    ('b', 3),
                    ('c', 3),
                ],
            },
            CountTest{
                text: "a
b W cd".to_string(),
                results: vec![
                    ('a', 1),
                    ('b', 1),
                    ('c', 1),
                    ('\n', 1),
                    (' ', 2),
                    ('W', 1),
                    ('d', 1)
                ],
            }
        ];

        for test in tests {
            let (counts, total) = get_counts(test.text);
            let mut count_total: u32 = 0;
            assert_eq!(counts.keys().len(), test.results.len());
            for (character, count) in test.results {
                match counts.get(&character) {
                    Some(&calc_count) => assert_eq!(count, calc_count),
                    None => assert_eq!(false, true),
                };
                count_total += count;
            }
            assert_eq!(count_total, total);

        }

    }
}