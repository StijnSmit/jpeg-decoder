use std::collections::HashMap;
use std::fs;

mod entropy_coding;
mod huffman_tree;

#[repr(u16)]
enum Headers {
    StartOfImage = 0xffd8,
    Application = 0xffe0,
    QuantizationTable = 0xffdb,
    StartOfFrame = 0xffc0,
    HuffmanTable = 0xffc4,
    StartOfScan = 0xffda, 
    EndOfImage = 0xffd9,
}

struct jpeg {
    qtables: Vec<QuantizationTable>,
    sof: ImageData,
    htables: Vec<HuffmanHeader>,
    raw_data: Vec<u8>,
}

impl jpeg {
    fn new() -> Self {
        unimplemented!()
    }
}


fn main() {
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
                    let _chunk = &data[4..=lenchunk + 2];

                    if marker == 0xffc4 {
                        let chunk = &data[2..];
                        let table = decode_huffman_2(chunk);
                        println!("{:?}", table);
                    } else if marker == 0xffdb {
                        let chunk = &data[2..lenchunk + 2];
                        let table = QuantizationTable::new(chunk);
                       println!("QT: {:?}", table);
                    } else if marker == 0xffc0 {
                        let chunk = &data[2..lenchunk + 2];
                        let detail = decode_frame(chunk);
                        println!("Frame Detail: {:?}", detail);
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

#[derive(Debug)]
struct ImageData {
    data_precision: u8,
    height: u16,
    width: u16,
    noc: u8,
    components: Vec<(u8, u8, u8, u8)>,
}

fn decode_frame(input: &[u8]) -> ImageData {
    let mut components: Vec<(u8, u8, u8, u8)> = Vec::new();
    let mut index = 7;
    let noc = input[index] as u8;
    index += 1;
    for _ in 0..noc {
        let id = input[index];
        let sampling_factors = input[index + 1];
        let h = sampling_factors >> 4;
        let v = sampling_factors & 0x0f;
        let table_id = input[index + 2];
        components.push((id, h, v, table_id));
        index += 3;
    }

    ImageData {
        data_precision: input[2],
        height: u16::from_be_bytes([input[3], input[4]]),
        width: u16::from_be_bytes([input[5], input[6]]),
        noc,
        components,
    }
}

#[derive(Debug)]
struct QuantizationTable {
    class_destination: u8,
    bytes: Vec<u8>,
}

impl QuantizationTable {
    fn new(input: &[u8]) -> Self {
        QuantizationTable {
            class_destination: input[2],
            bytes: input[3..].to_vec()
        }
    }
}

#[derive(Debug)]
struct HuffmanHeader {
    length: u16,
    class_destination: u8,
    chunk: Vec<u8>,
    lengths: Vec<u8>,
    total_symbols: usize,
    symbols: Vec<u8>,
    elements_lengths: Vec<u8>,
}

fn decode_huffman_2(data: &[u8]) -> HuffmanHeader {
    let len_chunk = u16::from_be_bytes([data[0], data[1]]) as usize;
    let class_destination = data[2];
    let chunk = &data[..len_chunk];
    let mut data = &data[3..];

    let lengths = &data[..16];
    data = &data[16..];
    
    let total_symbols: usize = lengths.iter().map(|&x| x as usize).sum();
    let symbols = &data[..total_symbols];

    let mut elements_lengths = Vec::new();
    for (length, &count) in lengths.iter().enumerate() {
        elements_lengths.extend(std::iter::repeat((length + 1) as u8).take(count as usize));
    }

    HuffmanHeader {
        length: len_chunk as u16,
        class_destination,
        chunk: chunk.to_vec(),
        lengths: lengths.to_vec(),
        total_symbols,
        symbols: symbols.to_vec(),
        elements_lengths,
    }
}
