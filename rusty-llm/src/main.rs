mod tokenizer;

use tokenizer::bpe;

fn main() {
    let piece = String::from("from the data we got from our love and life");

    let result = bpe::tokenizer(&piece);
    print!("{:?}", result);
}
