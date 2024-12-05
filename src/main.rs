
//mod entropy_coding;
//mod huffman_tree;
use jpeg_decoder::create_jpeg;

fn main() {
    let jpeg = create_jpeg();
    println!("{:?}", jpeg);
}

