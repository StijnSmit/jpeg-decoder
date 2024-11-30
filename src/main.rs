use std::collections::HashMap;
use std::fs;

mod entropy_coding;
mod huffman_tree;

pub use crate::huffman_tree::heap_test;

fn main() {
    heap_test();
    return;
    let marker_mapping = HashMap::from([
        (0xffd8,  "Start of Image"),
        (0xffe0, "Application Default Header"),
        (0xffdb, "Quantization Table"),
        (0xffc0, "Start of Frame"),
        (0xffc4, "Define Huffman Table"),
        (0xffda, "Start of Scan"),
        (0xffd9, "End of Image"),
    ]);

    let img_data: Vec<u8> = fs::read("tutorial-image.jpg").unwrap();

    let mut data = img_data.as_slice();

    loop {
        if data.len() < 2 {
            return;
        }
        let marker = u16::from_be_bytes([data[0], data[1]]) as usize;
        if let Some(description) = marker_mapping.get(&marker) {
            println!("{}", description);
        }

        match marker {
            0xffd8 => {
                // Start of Image: Skip 2 bytes
                data = &data[2..];
            }
            0xffd9 => {
                // End of Image: Return
                return;
            }
            0xffda => {
                // Start of Scan: Skip to the last 2 bytes
                if data.len() > 2 {
                    data = &data[data.len() - 2..];
                } else {
                    break;
                }
            }
            _ => {
                if data.len() >= 4 {
                    let lenchunk = u16::from_be_bytes([data[2], data[3]]) as usize;
                    let chunk = &data[4..=lenchunk + 2];

                    if marker == 0xffc4 {
                        println!("Huffman {}", lenchunk);
                        decode_huffman(chunk);
                    }

                    if data.len() >= 2 + lenchunk {
                        data = &data[2 + lenchunk..];
                    } else {
                        println!("BREAK");
                        break;
                    }


                } else {
                    break 
                }
            }
        }
    }
}

fn decode_huffman(data: &[u8]) {
    let header = data[0];
    let lengths  = &data[1..17];
    let total_symbols: usize = lengths.iter().map(|&x| x as usize).sum();
    let symbols = &data[17..17 + total_symbols];

    let mut elements_lengths = Vec::new();
    for (length, &count) in lengths.iter().enumerate() {
        elements_lengths.extend(std::iter::repeat((length + 1) as u8).take(count as usize));
    }


    println!("Header: {header}");
    println!("Lengths: {lengths:?}");
    println!("Total symbols: {total_symbols}");
    println!("symbols: {symbols:?}");
    println!("Elements lengths: {}", elements_lengths.len());
}
