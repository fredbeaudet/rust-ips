use std::io::Read;
use std::fs::OpenOptions;

pub struct PatchRecord {
    pub offset: u32,
    pub size: u16,
    pub data: Vec<u8>,
    pub rle_encoded: bool,
}

pub struct PatchFile {
    bytes: Vec<u8>,
    pub records: Vec<PatchRecord>,
}

impl PatchFile {
    pub fn new(patch_file_path:&std::path::PathBuf) -> PatchFile{
        let patch_file_bytes = get_patch_bytes(patch_file_path);
        let mut records = Vec::<PatchRecord>::new();
        let mut offset = 5 as usize;
    
        while (offset < patch_file_bytes.len() - 3) {
            records.push(parse_record(&patch_file_bytes, &mut offset));
        }
    
        return PatchFile {
            bytes: patch_file_bytes.to_vec(),
            records: records,
        };
    }
}

fn get_patch_bytes(patch_file_path:&std::path::PathBuf) -> Vec<u8>{
    let mut patch_file = match OpenOptions::new().read(true).open(patch_file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error: couldn't open PATCH_FILE: {}", err);
            std::process::exit(1);
        }
    };

    let metadata = std::fs::metadata(patch_file_path).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    patch_file.read(&mut buffer).expect("buffer overflow");
    
    return buffer
}

fn parse_record(bytes:&Vec<u8>, offset:&mut usize) -> PatchRecord {
    let record_offset = parse_record_offset(bytes, offset);
    let (mut record_size, rle_encoded) = parse_record_size_and_encoding(bytes, offset);
    let record_data = parse_record_data(bytes, offset, &mut record_size, &rle_encoded);
    
    return PatchRecord{
        offset: record_offset,
        size: record_size,
        rle_encoded: rle_encoded,
        data: record_data,
    };
}

fn parse_record_offset(bytes:&Vec<u8>, offset:&mut usize) -> u32 {
    let record_offset = (0 << 24) + 
        ((bytes[*offset] as u32) << 16) + 
        ((bytes[*offset+1] as u32) <<  8) + 
        ((bytes[*offset+2] as u32) <<  0) ;
        
    *offset += 3;
    return record_offset;
}

fn parse_record_size_and_encoding(bytes:&Vec<u8>, offset:&mut usize) -> (u16, bool) {
    let mut record_size = ((bytes[*offset] as u16) << 8) + ((bytes[*offset+1] as u16) << 0);
    let mut rle_encoded = false;
    *offset += 2;
    
    if(record_size == 0) {
        record_size = ((bytes[*offset] as u16) << 8) + ((bytes[*offset+1] as u16) << 0);
        rle_encoded = true;
        *offset += 2;
    }

    return (record_size, rle_encoded);
}

fn parse_record_data(bytes:&Vec<u8>, 
    offset:&mut usize, record_size:&mut u16, rle_encoded:&bool) -> Vec<u8> {
    
    if (*rle_encoded) {
        let data = (&bytes[*offset..(*offset+1)]).to_vec(); 
        *offset += 1;
        return data;
    }
    
    let data = (&bytes[*offset..(*offset+(*record_size as usize))]).to_vec(); 
    *offset += *record_size as usize;
    return data;
}