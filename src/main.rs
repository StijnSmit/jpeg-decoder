use std::collections::VecDeque;
use crate::huffman_tree::HuffmanTable;

//mod entropy_coding;
mod huffman_tree;

fn main() {
    let jpeg = jpeg_decoder::create_jpeg();
    let first = jpeg.htables.first().unwrap();
/*
    println!(" {:?}", first.code_lengths);
    println!("{:?}", jpeg.htables);
    println!("Code Lengths {:?}", first.code_lengths);
    println!("Symbols      {:?}", first.symbols);
    println!("Elements Len {:?}", first.elements_lengths);
    println!("Data ");
*/
    for i in jpeg.raw_data.iter().take(50) {
        print!("{:x} - ", i);
    }

    let dc_table = jpeg.htables.first().unwrap();
    let ac_table = jpeg.htables.get(1).unwrap();
    assert_eq!(dc_table.p, 0);
    assert_eq!(dc_table.t, 0);
    assert_eq!(ac_table.p, 1);
    assert_eq!(ac_table.t, 0);

    let dc = HuffmanTable::from_canonical_code(&dc_table.code_lengths, &dc_table.symbols);
    let ac = HuffmanTable::from_canonical_code(&ac_table.code_lengths, &ac_table.symbols);

    println!("\n\nNOW");
    println!("First DC Table!: {:?}", dc.get_huffman_codes());
    println!("First AC Table!: {:?}", ac.get_huffman_codes());

    let mut stream = jpeg.bitstream();
    let first_matrix = get_matrix(&mut stream, &dc, &ac);

    println!("Matrix: {:?}", first_matrix);
}

fn get_matrix(stream: &mut VecDeque<u8>, dc_table: &HuffmanTable, ac_table: &HuffmanTable) -> Vec<u8> {


    
    while let Some(bit) = stream.pop_front() {
//        print!("{:x} - ", bit);
    }

    vec![]
}

