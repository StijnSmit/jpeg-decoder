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

#[derive(Debug)]
struct DHTNode {
    value: Option<u8>,
    left: Option<Box<DHTNode>>,
    right: Option<Box<DHTNode>>,
}

impl DHTNode {
    fn new() -> Self {
        DHTNode {
            value: None,
            left: None,
            right: None,
        }
    }

    fn new_outer(value: u8) -> Self {
        DHTNode {
            value: Some(value),
            left: None,
            right: None,
        }
    }

    fn new_inner(left: DHTNode, right: DHTNode) -> Self {
        DHTNode {
            value: None,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        }
    }

    fn insert(&mut self, code: &str, value: u8) {
        let mut current = self;
        for bit in code.chars() {
            match bit {
                '0' => {
                    if current.left.is_none() {
                        current.left = Some(Box::new(DHTNode::new()));
                    }
                    current = current.left.as_mut().unwrap();
                }
                '1' => {
                    if current.right.is_none() {
                        current.right = Some(Box::new(DHTNode::new()));
                    }
                    current = current.right.as_mut().unwrap();
                }
                _ => panic!("Invalid bit in code: {}", bit),
            }
        }
        current.value = Some(value);
    }
}

#[derive(Debug)]
pub struct HuffmanTable {
    top_node: DHTNode,
}

impl HuffmanTable {
    pub fn from_canonical_code(length_table: &[u8], symbols: &[u8]) -> Self {
println!("{:?}", length_table);
        println!("{:?}", symbols);
        let mut codes = Vec::new();
        let mut code = 0;
        for (length, &count) in length_table.iter().enumerate() {
            for _ in 0..count {
                let binary_code = format!("{:0length$b}", code, length = length + 1); // Generate
                // binary code
                codes.push(binary_code);
                code += 1;
            }
            code <<= 1; // Shift left to prepare for the next length
        }

        let mut root = DHTNode::new();
        for (code, &symbol) in codes.iter().zip(symbols.iter()) {
            root.insert(code, symbol);
        }
        HuffmanTable { top_node: root }
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


    fn recursive_huffman_codes(node: &DHTNode, current_code: String, codes: &mut HashMap<u8, String>) {
        if let Some(value) = node.value {
            codes.insert(value, current_code);
        } else {
            if let Some(ref left) = node.left {
                Self::recursive_huffman_codes(left, format!("{current_code}0"), codes);
            }
            if let Some(ref right) = node.right {
                Self::recursive_huffman_codes(right, format!("{current_code}1"), codes);
            }
        }
    }
    pub fn get_huffman_codes(&self) -> HashMap<u8, String> {
        let mut codes = HashMap::new();
        Self::recursive_huffman_codes(&self.top_node, "".to_string(), &mut codes);
        codes
    }
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

    #[test]
    fn test_canonical_code_parsing() {
        let length_table = [0, 2, 2, 3, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let symbols = [5, 6, 3, 4, 2, 7, 8, 1, 0, 9];

        let expected: HashMap<u8, String> = HashMap::from([
            (0, "111".to_string()), (1, "110".to_string()), (8, "0".to_string()), (4, "10".to_string()),
        ]);


        let table = HuffmanTable::from_canonical_code(&length_table, &symbols);
        table.get_huffman_codes();

        println!("Table!: {:?}", table.get_huffman_codes());
        assert!(false);
    }
}
