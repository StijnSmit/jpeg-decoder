use std::{fmt, collections::VecDeque};

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
                panic!("There can't be an unknown header: {}", marker);
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

pub struct JPEG {
    qtables: Vec<QuantizationTable>,
    sof: ImageData,
    sos: SOS,
    pub htables: Vec<DHT>,
    pub raw_data: Vec<u8>,
}

impl fmt::Debug for JPEG {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("Image details", &self.sof)
            .field("Scan", &self.sos)
            .field("Quantized tables", &self.qtables.len())
            .field("Huffman tables", &self.htables.len())
            .field("\ncontents", &self.htables)
            .field("Raw data of len", &self.raw_data.len())
            .finish()
    } 
}

impl JPEG {
    pub fn new(img_data: Vec<u8>) -> Self {
        let mut data = img_data.as_slice();
        let mut qtables: Vec<QuantizationTable> = vec![];
        let mut sof: ImageData = ImageData::empty();
        let mut sos: SOS = SOS::empty();
        let mut htables: Vec<DHT> = vec![];
        let mut raw_data: Vec<u8> = vec![];

        loop {
            if data.len() < 2 {
                break;
            }
            let marker = Markers::from_u16(u16::from_be_bytes([data[0], data[1]]));

            // Start of Image: Skip 2 bytes
            if marker == Markers::StartOfImage {
                 data = &data[2..];
                continue;
            }

            // End of Image: Return
            if marker == Markers::EndOfImage {
                break;
            }

            let lenchunk = u16::from_be_bytes([data[2], data[3]]) as usize;
            let _chunk = &data[4..=lenchunk + 2];

            if data.len() < 4 {
                println!("Data not long enough");
                break;
            }

            match marker {
                Markers::StartOfScan => {
                    // Start of Scan: Skip to the last 2 bytes
                    let lenchunk = u16::from_be_bytes([data[2], data[3]]) as usize;
                    sos = SOS::from_data(&data[4..lenchunk + 2]);
                    raw_data = data[lenchunk + 2..].to_vec();
                    data = &data[data.len() - 2..];
                    continue;
                }
                Markers::DefineHuffmanTable => {
                    let second_table = DHT::new(_chunk);
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
                break;
            }
        }

        JPEG { qtables , sof, sos, htables, raw_data }
    }

    pub fn bitstream(&self) -> VecDeque<u8> {
        let mut bitstream = VecDeque::new();
        for byte in &self.raw_data {
            for bit_pos in (0..8).rev() {
                let bit = (byte >> bit_pos) & 1;
                bitstream.push_back(bit);
            }
        }
        bitstream
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
pub struct DHT {
    pub p: u8,
    pub t: u8,
    pub code_lengths: Vec<u8>,
    total_symbols: usize,
    pub symbols: Vec<u8>,
    pub elements_lengths: Vec<u8>,
}

impl DHT {
    fn new(input: &[u8]) -> Self {
        
        let cd = input[0];
        let p = cd >> 4;
        let t = cd & 0x0f;
        let code_lengths = &input[1..17];
        let total_symbols: usize = code_lengths.iter().map(|&x| x as usize).sum();
        let symbols = &input[17..17 + total_symbols];

        let mut elements_lengths = Vec::new();
        for (length, &count) in code_lengths.iter().enumerate() {
            elements_lengths.extend(std::iter::repeat((length + 1) as u8).take(count as usize));
        }

        DHT { p, t, code_lengths: code_lengths.to_vec(), total_symbols, symbols: symbols.to_vec(), elements_lengths }
    }
}

#[derive(Debug)]
struct SOS {
    comp: Vec<(u8, u8, u8)>,
    spectral_lower: u8,
    spectral_upper: u8,
    successive_approx: u8,
}

impl SOS {
    fn empty() -> Self {
        SOS { comp: vec![], spectral_lower: 0, spectral_upper: 0, successive_approx: 0 }
    }

    fn from_data(data:  &[u8]) -> Self {
        let mut cursor = 0;
        let comp_count = data[cursor];
        cursor += 1;
        let mut comp = vec![];

        for _ in 0..comp_count {
            let dc = data[cursor+1] >> 4;
            let ac = data[cursor+1] & 0x0f;
            comp.push((data[cursor], dc, ac));
            cursor += 2;
        };

        let spectral_lower = data[cursor];
        let spectral_upper = data[cursor+1];

        cursor += 2;
        let successive_approx = data[cursor];

        SOS { comp, spectral_lower, spectral_upper, successive_approx }
    }
}
