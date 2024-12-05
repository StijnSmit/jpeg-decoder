use std::fs;

//mod entropy_coding;
//mod huffman_tree;
mod jpeg;

fn main() {
    let img_data: Vec<u8> = fs::read("tutorial-image.jpg").unwrap();
    let jpeg = jpeg::JPEG::new(img_data);
    println!("{:?}", jpeg);
}

