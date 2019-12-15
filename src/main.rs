use std::collections::HashMap;

fn main() {
    let word = "abcdefg
efg";


    huffman_encode(word.to_string());
}

fn get_counts(text: String) -> (HashMap<char, f64>, f64) {
    let mut total: f64 = 0.0;
    let mut counts: HashMap<char, f64> = HashMap::new();
    for c in text.chars() {
        match counts.get(&c) {
            Some(review) => counts.insert(c, review + 1.0),
            None => counts.insert(c, 1.0),
        };
        total += 1.0
    }
    return (counts, total);
}

fn get_probabilities(counts: HashMap<char, f64>, total: f64) -> HashMap<char, f64> {
    let mut probs: HashMap<char, f64> = HashMap::new();
    for (k, v) in counts {
        probs.insert(k, v / total);
    }
    return probs;
}

fn huffman_encode(text: String) {

    let (counts, total) = get_counts(text);
    let probs = get_probabilities(counts, total);
    let prob_pairs: Vec<(&f64, String)> = probs.iter().map(|x| (x.1, x.0.to_string())).collect();

    println!("{:?} {}", prob_pairs, 1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    struct CountTest {
        text: String,
        results: Vec<(char, f64)>,
    }

    #[test]
    fn run_tests() {
        let tests = vec![
            CountTest{
                text: "a".to_string(),
                results: vec![
                    ('a', 1.0),
                ],
            },
            CountTest{
                text: "aaaabbbcccaaaa".to_string(),
                results: vec![
                    ('a', 8.0),
                    ('b', 3.0),
                    ('c', 3.0),
                ],
            },
            CountTest{
                text: "a
b W cd".to_string(),
                results: vec![
                    ('a', 1.0),
                    ('b', 1.0),
                    ('c', 1.0),
                    ('\n', 1.0),
                    (' ', 2.0),
                    ('W', 1.0),
                    ('d', 1.0)
                ],
            }
        ];

        for test in tests {
            test_counts(test.clone());
            let (counts, total) = get_counts(test.text.clone());
            test_prob_dist(counts, total, test);
        }

    }

    fn test_counts(test: CountTest) {
        let (counts, total) = get_counts(test.text);
        let mut count_total: f64 = 0.0;
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

    fn test_prob_dist(counts: HashMap<char, f64>, total: f64, test: CountTest) {
        let prob_dist = get_probabilities(counts, total);
        let mut tot_prob = 0.0;
        for (c, count) in test.results {
            match prob_dist.get(&c) {
                Some(&calc_prob) => assert_eq!(calc_prob, count / total),
                None => assert!(false, "Failed to calculate correct probability"),
            }
            tot_prob += count / total;
        }
        assert_eq!(tot_prob, 1.0);
    }
}