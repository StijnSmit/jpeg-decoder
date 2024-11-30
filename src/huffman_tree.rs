use std::collections::{HashMap, BinaryHeap};

fn huffman_frequency(input: &str) -> HashMap<char, i32> {
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




#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_huffman() {
        let input = "AAAABBBCCD";
        let expectation: HashMap<char, i32> =
            HashMap::from([('A', 4), ('B', 3), ('C', 2), ('D', 1)]);

        assert_eq!(huffman_frequency(input), expectation);
    }
}
