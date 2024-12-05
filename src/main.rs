use std::fs;
use std::fmt;

//mod entropy_coding;
//mod huffman_tree;

fn main() {
    let img_data: Vec<u8> = fs::read("tutorial-image.jpg").unwrap();
    let jpeg = JPEG::new(img_data);
    println!("{:?}", jpeg);
}

#[derive(PartialEq, Debug)]
enum Markers {
    StartOfImage,
    ApplicationDefaultHeader,
    QuantizationTable,
    StartOfFrame,
    DefineHuffmanTable,
    StartOfScan,
    EndOfImage,
}

impl Markers {
    fn from_u16(marker: u16) -> Self {
        match marker {
            0xffd8 => Markers::StartOfImage,
            0xffe0 => Markers::ApplicationDefaultHeader,
            0xffdb => Markers::QuantizationTable,
            0xffc0 => Markers::StartOfFrame,
            0xffc4 => Markers::DefineHuffmanTable,
            0xffda => Markers::StartOfScan,
            0xffd9 => Markers::EndOfImage,
            _ => {
                panic!("There can't be an unknown header");
            }
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Markers::StartOfImage => "Start of Image",
            Markers::ApplicationDefaultHeader => "Application Default Header",
            Markers::QuantizationTable => "Quantization Table",
            Markers::StartOfFrame => "Start of Frame",
            Markers::DefineHuffmanTable => "Define Huffman Table",
            Markers::StartOfScan => "Start of Scan",
            Markers::EndOfImage => "End of Image",
        }
    }
}

struct JPEG {
    qtables: Vec<QuantizationTable>,
    sof: ImageData,
    htables: Vec<HuffmanHeader>,
    raw_data: Vec<u8>,
}

impl fmt::Debug for JPEG {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("Image details: ", &self.sof)
            .field("Quantized tables: ", &self.qtables.len())
            .field("Huffman tables: ", &self.htables.len())
            .field("Raw data of len: ", &self.raw_data.len())
            .finish()
    } 
}

impl JPEG {
    fn new(img_data: Vec<u8>) -> Self {
        let mut data = img_data.as_slice();
        let mut qtables: Vec<QuantizationTable> = vec![];
        let mut sof: ImageData = ImageData::empty();
        let mut htables: Vec<HuffmanHeader> = vec![];
        let mut raw_data: Vec<u8> = vec![];

        loop {
            if data.len() < 2 {
                break;
            }
            let marker = Markers::from_u16(u16::from_be_bytes([data[0], data[1]]));



            match marker {
                Markers::StartOfImage => {
                    // Start of Image: Skip 2 bytes
                    data = &data[2..];
                }
                Markers::EndOfImage => {
                    // End of Image: Return
                    break;
                }
                Markers::StartOfScan => {
                    // Start of Scan: Skip to the last 2 bytes
                    if data.len() > 2 {
                        // Start of Scan: Skip to the last 2 bytes
                        let lenchunk = u16::from_be_bytes([data[2], data[3]]) as usize;
                        println!("Data length: {}, len: {}", data.len(), lenchunk);
                        for i in 0..lenchunk + 2 {
                            println!("{:x}", data[i]);
                        }
                        raw_data = data[lenchunk + 2..].to_vec();
                        data = &data[data.len() - 2..];
                    } else {
                        break;
                    }
                }
                _ => {
                    if data.len() >= 4 {
                        let lenchunk = u16::from_be_bytes([data[2], data[3]]) as usize;
                        let _chunk = &data[4..=lenchunk + 2];

                        match marker {
                            Markers::DefineHuffmanTable => {
                                let second_table = HuffmanHeader::new(_chunk);
                                htables.push(second_table);
                            }
                            Markers::QuantizationTable => {
                                let chunk = &data[2..lenchunk + 2];
                                let table = QuantizationTable::new(chunk);
                                qtables.push(table);
                            }
                            Markers::StartOfFrame => {
                                let chunk = &data[2..lenchunk + 2];
                                sof = ImageData::new(chunk);
                            }
                            _ => {}
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
        JPEG { qtables , sof, htables, raw_data }
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

impl ImageData {

    fn empty() -> Self { ImageData { data_precision: 0, height: 0, width: 0, noc: 0, components: vec![] } }

    fn new(input: &[u8]) -> Self {
        let mut components: Vec<(u8, u8, u8, u8)> = Vec::new();
        let mut index = 7;
        let noc = input[index];
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
    class: u8,
    destination: u8,
    lengths: Vec<u8>,
    total_symbols: usize,
    symbols: Vec<u8>,
    elements_lengths: Vec<u8>,
}

impl HuffmanHeader {
    fn new(input: &[u8]) -> Self {
        
        let cd = input[0];
        let class = cd >> 4;
        let destination = cd & 0x0f;
        let lengths = &input[1..17];
        let total_symbols: usize = lengths.iter().map(|&x| x as usize).sum();
        let symbols = &input[17..17 + total_symbols];

        let mut elements_lengths = Vec::new();
        for (length, &count) in lengths.iter().enumerate() {
            elements_lengths.extend(std::iter::repeat((length + 1) as u8).take(count as usize));
        }

        HuffmanHeader { class, destination, lengths: lengths.to_vec(), total_symbols, symbols: symbols.to_vec(), elements_lengths }
    }
}
