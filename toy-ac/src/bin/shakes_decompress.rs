use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

use bitbit::{BitReader, MSB};
use toy_ac::decoder::Decoder;

use toy_ac::symbol_model::VectorCountSymbolModel;
use workspace_root::get_workspace_root;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sm_contexts: Vec<VectorCountSymbolModel<u8>> = Vec::new();
    for _ in 0..=255 {
        sm_contexts.push(VectorCountSymbolModel::new((0..=255).collect()));
    }

    let mut dec = Decoder::new();

    let mut data_folder_path = get_workspace_root();
    data_folder_path.push("data");

    let input_file = match File::open(data_folder_path.join("out.dat")) {
        Err(_) => panic!("Error opening file"),
        Ok(f) => f,
    };

    let output_file = match File::create(data_folder_path.join("reconstructed.txt")) {
        Err(_) => panic!("Error opening output file"),
        Ok(f) => f,
    };

    let mut buf_reader = BufReader::new(input_file);

    let mut size_bytes: [u8; 8] = [0; 8];
    buf_reader.read(&mut size_bytes)?;
    let output_size = u64::from_be_bytes(size_bytes);

    let mut br: BitReader<_, MSB> = BitReader::new(&mut buf_reader);

    let mut writer = BufWriter::new(output_file);

    let mut prior = 0;

    for _ in 0..output_size {
        let next_byte = dec.decode(&sm_contexts[prior], &mut br);
        let next_byte = next_byte.to_owned();

        sm_contexts[prior].incr_count(&next_byte);
        prior = next_byte as usize;

        writer.write(&[next_byte])?;
    }

    writer.flush()?;

    Ok(())
}
