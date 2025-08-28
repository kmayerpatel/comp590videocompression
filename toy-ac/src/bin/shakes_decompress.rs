use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

use toy_ac::decoder::Decoder;
use bitbit::{BitReader, MSB};

use toy_ac::symbol_model::VectorCountSymbolModel;
use workspace_root::get_workspace_root;

fn main() {
    let mut sm = VectorCountSymbolModel::new((0..=255).collect());
    let mut dec = Decoder::new();
    
    let mut data_folder_path = get_workspace_root();
    data_folder_path.push("data");

    let input_file = match File::open(data_folder_path.join("out.dat")) {
        Err(_) => panic!("Error opening file"),
        Ok(f) => f
    };

    let output_file = match File::create(data_folder_path.join("reconstructed.txt")) {
        Err(_) => panic!("Error opening output file"),
        Ok(f) => f
    };

    let mut buf_reader = BufReader::new(input_file);
    let mut br: BitReader<_, MSB> = BitReader::new(&mut buf_reader);

    let mut writer = BufWriter::new(output_file);

    while true {
        let next_byte = dec.decode(&sm, &mut br);
        sm.incr_count(next_byte);
        writer.write(&[*next_byte]);
    }

    writer.flush();
    
//    enc.encode(&(5 as u8), &sm, &mut bw);

    println!("Hello, world");
}
