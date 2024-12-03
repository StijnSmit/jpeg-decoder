const ZIGZAG_INDICES: [(i32, i32); 64] = [
    (0, 0), (0, 1), (1, 0), (2, 0), (1, 1), (0, 2), (0, 3), (1, 2),
    (2, 1), (3, 0), (4, 0), (3, 1), (2, 2), (1, 3), (0, 4), (0, 5),
    (1, 4), (2, 3), (3, 2), (4, 1), (5, 0), (6, 0), (5, 1), (4, 2),
    (3, 3), (2, 4), (1, 5), (0, 6), (0, 7), (1, 6), (2, 5), (3, 4),
    (4, 3), (5, 2), (6, 1), (7, 0), (7, 1), (6, 2), (5, 3), (4, 4),
    (3, 5), (2, 6), (1, 7), (2, 7), (3, 6), (4, 5), (5, 4), (6, 3),
    (7, 2), (7, 3), (6, 4), (5, 5), (4, 6), (3, 7), (4, 7), (5, 6),
    (6, 5), (7, 4), (7, 5), (6, 6), (5, 7), (6, 7), (7, 6), (7, 7),
];

pub fn decode(input: &[i32]) -> [[i32; 8]; 8] {
    assert_eq!(input.len(), 64);
    let mut zigzag: [[i32; 8];8] = [[0; 8]; 8];


    for (index, &(row, col)) in ZIGZAG_INDICES.iter().enumerate() {
        zigzag[row as usize][col as usize] = input[index];
    }

    zigzag
}

pub fn encode(input: &[[i32; 8]; 8]) -> Vec<i32> {
    let mut zigzag: Vec<i32> = vec![];

    for &(row, col) in &ZIGZAG_INDICES {
        zigzag.push(input[row as usize][col as usize]);
    }

    zigzag
}

pub fn run_length_encoded(input: Vec<i32>) -> Vec<(i32, i32)> {
    let mut zero_count = 0;
    let mut v: Vec<(i32, i32)> = vec![];

    for i in input {
        if i == 0 {
            zero_count += 1;
        } else {
            v.push((zero_count, i));
            zero_count = 0;
        }
    }

    if zero_count > 0 {
        v.push((0, 0));
    }
        
    v
}

pub fn run_length_decoded(coded: Vec<(i32, i32)>) -> Vec<i32> {
    let required_length = 8 * 8;
    let mut v: Vec<i32> = vec![0; required_length];

    let mut index: usize = 0;
    for (i, value) in coded.iter().enumerate() {
        index += value.0 as usize;
        v[index + i] = value.1;
    }

    v
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zigzag_encode_order() {
        let quantized_block: [[i32; 8]; 8] = [
            [ 16, 11, 10, 16,  24,  40,   51,  61 ], 
            [ 12, 12, 14, 19,  26,  58,   60,  55 ], 
            [ 14, 13, 16, 24,  40,  57,   69,  56 ], 
            [ 14, 17, 22, 29,  51,  87,   80,  62 ], 
            [ 18, 22, 37, 56,  68, 109,  103,  77 ], 
            [ 24, 35, 55, 64,  81, 104,  113,  92 ], 
            [ 49, 64, 78, 87, 103, 121,  120, 101 ], 
            [ 72, 92, 95, 98, 112, 100,  103,  99 ], 
        ];

        let expected_output: Vec<i32> = vec![
            16, 11, 12, 14, 12, 10, 16, 14,
            13, 14, 18, 17, 16, 19, 24, 40,
            26, 24, 22, 22, 24, 49, 35, 37,
            29, 40, 58, 51, 61, 60, 57, 51,
            56, 55, 64, 72, 92, 78, 64, 68,
            87, 69, 55, 56, 80, 109, 81, 87,
            95, 98, 103, 104, 103, 62, 77, 113,
            121, 112, 100, 120, 92, 101, 103, 99
        ];

        assert_eq!(encode(&quantized_block), expected_output);
    }

    #[test]
    fn zigzag_decode_order() {

        let expected_output: [[i32; 8]; 8] = [
            [ 16, 11, 10, 16,  24,  40,   51,  61 ], 
            [ 12, 12, 14, 19,  26,  58,   60,  55 ], 
            [ 14, 13, 16, 24,  40,  57,   69,  56 ], 
            [ 14, 17, 22, 29,  51,  87,   80,  62 ], 
            [ 18, 22, 37, 56,  68, 109,  103,  77 ], 
            [ 24, 35, 55, 64,  81, 104,  113,  92 ], 
            [ 49, 64, 78, 87, 103, 121,  120, 101 ], 
            [ 72, 92, 95, 98, 112, 100,  103,  99 ], 
        ];

        let quantized_block: Vec<i32> = vec![
            16, 11, 12, 14, 12, 10, 16, 14,
            13, 14, 18, 17, 16, 19, 24, 40,
            26, 24, 22, 22, 24, 49, 35, 37,
            29, 40, 58, 51, 61, 60, 57, 51,
            56, 55, 64, 72, 92, 78, 64, 68,
            87, 69, 55, 56, 80, 109, 81, 87,
            95, 98, 103, 104, 103, 62, 77, 113,
            121, 112, 100, 120, 92, 101, 103, 99
        ];

        assert_eq!(decode(&quantized_block), expected_output);
    }

    #[test]
    fn run_length_encoding() {
        let input = vec![
            16, 11, 12, 12, 0, 10, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        let expected_output = [(0, 16), (0, 11), (0, 12), (0, 12), (1, 10), (0, 14), (0, 0)];
        assert_eq!(run_length_encoded(input), expected_output);
    }

    #[test]
    fn run_length_decoding() {
        let input = [(0, 16), (0, 11), (0, 12), (0, 12), (1, 10), (0, 14), (0, 0)].to_vec();
        let expected_output = vec![
            16, 11, 12, 12, 0, 10, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        assert_eq!(run_length_decoded(input), expected_output);
    }
}
