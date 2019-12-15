use std::collections::HashMap;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct Node {
    prob: f64,
    val: String,
    left: Vec<Node>,
    right: Vec<Node>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.prob > other.prob {
            return Ordering::Less;
        }
        if self.prob < other.prob {
            return Ordering::Greater;
        }
        return Ordering::Equal;
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl Eq for Node {

}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool { 
        return self.prob == other.prob;
    }
}

fn main() {
    let word = "       aaaaeeeefffhhiimmnnssttloprux";

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

fn parse_codes(char_map: HashMap<char, f64>, curr_node: &Node, mut bin_maps: HashMap<char, String>, curr_word: String) -> HashMap<char, String> {
    let curr_char = curr_node.val.clone().into_bytes()[0] as char;
    if curr_node.val.clone().into_bytes().len() == 1 && char_map.contains_key(&curr_char) {
        bin_maps.insert(curr_char, curr_word);
    } else {
        bin_maps = parse_codes(char_map.clone(), &curr_node.left[0], bin_maps.clone(), curr_word.clone() + "0");
        bin_maps = parse_codes(char_map, &curr_node.right[0], bin_maps, curr_word.clone() + "1");
    }
    return bin_maps;
}

fn huffman_encode(text: String) -> HashMap<char, String> {
    let (counts, total) = get_counts(text);
    let probs = get_probabilities(counts, total);
    let prob_pairs: Vec<Node> = probs.iter().map(|x| Node {prob: *x.1, val: x.0.to_string(), left: vec![], right: vec![] }).collect();
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();

    for node in prob_pairs {
        heap.push(node);
    }

    while heap.len() > 1 {
        let first_node = match heap.pop() {
            Some(node) => node,
            None => panic!("No node found!"),
        }; 
        let second_node = match heap.pop() {
            Some(node) => node,
            None => panic!("No node found!"),
        };
        let new_node = Node {
            val: first_node.val.clone() + &second_node.val,
            prob: first_node.prob + second_node.prob,
            left: vec![first_node],
            right: vec![second_node],
        };
        heap.push(new_node);
    }

    let root = match heap.pop() {
        Some(node) => node,
        None => panic!("Expected a node"),
    };
    
    println!("{:?} {}", parse_codes(probs.clone(), &root, HashMap::new(), "".to_string()), 1);
    return parse_codes(probs.clone(), &root, HashMap::new(), "".to_string())
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
    fn run_counter_tests() {
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

    struct EncodeTest {
        word: String,
        lengths: Vec<(char, u32)>,
    }

    #[test]
    fn test_huffman_encoding() {

        let tests = vec![
            EncodeTest {
                word: "       aaaaeeeefffhhiimmnnssttloprux".to_string(),
                lengths: vec![
                    (' ', 3),
                    ('a', 3),
                    ('e', 3),
                    ('f', 4),
                    ('h', 4),
                    ('i', 4),
                    ('m', 4),
                    ('n', 4),
                    ('s', 4),
                    ('t', 4),
                    ('l', 5),
                    ('o', 5),
                    ('p', 5),
                    ('r', 5),
                    ('u', 5),
                    ('x', 5)
                ]
            },
            EncodeTest {
                word: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaabbbbbbbbbbbbbccccccccccccddddddddddddddddeeeeeeeeefffff".to_string(),
                lengths: vec![
                    ('a', 1),
                    ('b', 3),
                    ('c', 3),
                    ('d', 3),
                    ('e', 4),
                    ('f', 4)
                ]
            },
        ];

        for test in tests {
            let code_map = huffman_encode(test.word);
            for (c, len) in test.lengths {
                let char_len = match code_map.get(&c) {
                    Some(word) => word.len() as u32,
                    _ => panic!("Expected value"),
                };
                assert_eq!(char_len, len);
            }
        }

    }
}