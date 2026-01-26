use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

use bitbit::BitWriter;
use toy_ac::encoder::Encoder;

use toy_ac::symbol_model::VectorCountSymbolModel;
use workspace_root::get_workspace_root;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut sm_contexts: Vec<VectorCountSymbolModel<u8>> = Vec::new();
    for _ in 0..=255 {
        sm_contexts.push(VectorCountSymbolModel::new((0..=255).collect()));
    }

    let mut enc = Encoder::new();

    let mut data_folder_path = get_workspace_root();
    data_folder_path.push("data");

    let input_file = match File::open(data_folder_path.join("shakespeare.txt")) {
        Err(_) => panic!("Error opening file"),
        Ok(f) => f,
    };
    let metadata = input_file.metadata()?;
    let input_length = metadata.len();

    let output_file = match File::create(data_folder_path.join("out.dat")) {
        Err(_) => panic!("Error opening output file"),
        Ok(f) => f,
    };

    let mut buf_writer = BufWriter::new(output_file);
    // First write out the input length as a u64
    buf_writer.write(&input_length.to_be_bytes())?;

    let mut bw = BitWriter::new(&mut buf_writer);

    let reader = BufReader::new(input_file);

    let mut prior = 0;

    for next_byte in reader.bytes() {
        match next_byte {
            Ok(b) => {
                enc.encode(&b, &sm_contexts[prior], &mut bw);
                sm_contexts[prior].incr_count(&b);
                prior = b as usize;
            }
            Err(_) => panic!("Error reading byte from file"),
        }
    }

    enc.finish(&mut bw)?;

    bw.pad_to_byte()?;
    buf_writer.flush()?;

    Ok(())
}
