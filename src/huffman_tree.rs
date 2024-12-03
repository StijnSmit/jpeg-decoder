use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

pub fn huffman_frequency(input: &str) -> HashMap<char, i32> {
    let mut frequencies = HashMap::new();

    for c in input.chars() {
        let count = frequencies.entry(c).or_insert(0);
        *count += 1;
    }

    frequencies
        .iter()
        .map(|(key, value)| (*key, *value))
        .collect()
}

// Huffman will be made using:
// A custom node struct, containing:
//      Freq
//      Char for outer node
//      OR
//      Left, Right for inner node
//
//  BinaryHeap containing those nodes

#[derive(Eq, PartialEq, Debug)]
pub struct Node {
    frequency: i32,
    char: Option<char>,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new_outer(char: char, frequency: i32) -> Self {
        Node {
            frequency,
            char: Some(char),
            left: None,
            right: None,
        }
    }

    fn new_inner(frequency: i32, left: Node, right: Node) -> Self {
        Node {
            frequency,
            char: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.frequency.cmp(&self.frequency)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn huffman_tree(input: HashMap<char, i32>) -> Node {
    let mut heap = BinaryHeap::new();

    for (key, value) in input {
        heap.push(Node::new_outer(key, value));
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        heap.push(Node::new_inner(
            left.frequency + right.frequency,
            left,
            right,
        ));
    }
    heap.pop().unwrap()
}

pub fn huffman_tree_mapped(node: &Node) -> HashMap<char, String> {
    let mut map = HashMap::new();
    get_huffman_codes(node, "".to_string(), &mut map);
    map
}

pub fn get_huffman_codes(node: &Node, current_code: String, codes: &mut HashMap<char, String>) {
    if let Some(char) = node.char {
        codes.insert(char, current_code);
    } else {
        if let Some(ref left) = node.left {
            get_huffman_codes(left, format!("{current_code}0"), codes);
        }
        if let Some(ref right) = node.right {
            get_huffman_codes(right, format!("{current_code}1"), codes);
        }
    }
}

pub fn decode(node: &Node, input: String) -> String {
    let mut current_node = node;
    let mut decoded = "".to_string();

    for char in input.chars() {
        if char == '0' {
            if let Some(ref left) = current_node.left {
                current_node = left;
            }
        } else if char == '1' {
            if let Some(ref right) = current_node.right {
                current_node = right;
            }
        } else {
            panic!("There should be only a 0 OR 1");
        }

        if let Some(char) = current_node.char {
            current_node = node;
            decoded.push(char);
        }
    }

    decoded
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_huffman_frequency() {
        let input = "AAAABBBCCD";
        let expectation: HashMap<char, i32> =
            HashMap::from([('A', 4), ('B', 3), ('C', 2), ('D', 1)]);

        assert_eq!(huffman_frequency(input), expectation);
    }

    #[test]
    fn test_huffman_node() {
        let input: HashMap<char, i32> = HashMap::from([('A', 4), ('B', 3), ('C', 2), ('D', 1)]);
        let expectation = 10;

        assert_eq!(huffman_tree(input).frequency, expectation);
    }

    #[test]
    fn test_get_huffman_codes() {
        let input: HashMap<char, i32> = HashMap::from([('A', 4), ('B', 3), ('C', 2), ('D', 1)]);
        let tree = huffman_tree(input);
        let expected: HashMap<char, String> = HashMap::from([
            ('C', "111".to_string()), ('D', "110".to_string()), ('A', "0".to_string()), ('B', "10".to_string()),
        ]);
        assert_eq!(huffman_tree_mapped(&tree), expected);
    }

    #[test]
    fn test_decode_huffman() {
        let input: HashMap<char, i32> = HashMap::from([('A', 4), ('B', 3), ('C', 2), ('D', 1)]);
        let tree = huffman_tree(input);
        let coded_text = "0111010110001111".to_string();
        let decoded = decode(&tree, coded_text);
        let expected = "ACABDAAC".to_string();

        assert_eq!(decoded, expected);
    }
}
