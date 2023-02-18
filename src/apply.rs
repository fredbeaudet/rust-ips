use std::fs::File;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::Seek;
use std::io::SeekFrom;
use std::io::Write;

use crate::parser;

pub fn apply(patch_file_path:&std::path::PathBuf, 
    file_to_patch_path:&std::path::PathBuf, 
    output_file_path:&std::path::PathBuf) {

    validate_paths(patch_file_path, file_to_patch_path, output_file_path);
    copy_source_to_output(file_to_patch_path, output_file_path);
    apply_patch(patch_file_path, output_file_path);
}


fn apply_patch(patch_file_path:&std::path::PathBuf, output_file_path:&std::path::PathBuf) {
    let patch = parser::PatchFile::new(patch_file_path);
    let mut output = open_output_file(output_file_path);

    for record in patch.records {
        output.seek(SeekFrom::Start((record.offset).into()));
        if (record.rle_encoded) {
            for n in 0..record.size {
                output.write(&record.data);
            }
        } else {
            output.write(&record.data);
        }
    }
}

fn open_output_file(output_file_path:&std::path::PathBuf) -> std::fs::File {
    return match OpenOptions::new().write(true).open(output_file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error: couldn't write to OUTPUT_FILE: {}", err);
            std::process::exit(1);
        }
    };
}

fn copy_source_to_output(file_to_patch_path:&std::path::PathBuf, 
    output_file_path:&std::path::PathBuf) {
    
    match std::fs::copy(file_to_patch_path, output_file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error: couldn't create or write to OUTPUT_FILE: {}", err);
            std::process::exit(1);
        }
    };
}

fn validate_paths(patch_file_path:&std::path::PathBuf, 
    file_to_patch_path:&std::path::PathBuf, 
    output_file_path:&std::path::PathBuf) {

    match OpenOptions::new().read(true).open(patch_file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error: couldn't open PATCH_FILE: {}", err);
            std::process::exit(1);
        }
    };

    match std::fs::File::create(output_file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error: couldn't create or write to OUTPUT_FILE: {}", err);
            std::process::exit(1);
        }
    };

    match OpenOptions::new().read(true).open(file_to_patch_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Error: couldn't open FILE_TO_PATCH: {}", err);
            std::process::exit(1);
        }
    };    
}