use csv::{self,Reader};
use csv::{ReaderBuilder,WriterBuilder};
use std::fs::{self, File};
use std::env;

use std::path::Path;
use std::error::Error;

fn create_writer(parent:&str, base_name: &str,file_number: usize, headers: &csv::StringRecord) -> Result<csv::Writer<File>, Box<dyn Error>> {
    let filename = format!("{}/{}_{}.csv",parent,base_name, file_number);
    let path = Path::new(&filename);
    let file = File::create(path)?;
    let mut writer = WriterBuilder::new().from_writer(file);
    writer.write_record(headers)?;
    Ok(writer)
}

fn get_csv_file_readers(dir_path: &str) -> Vec<Reader<File>> {
    fs::read_dir(dir_path)
        .unwrap()
        .map(|direntry| direntry.unwrap().path())
        .filter_map(|path| {
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "csv" {
                        return Some(File::open(path));
                    }
                }
            }
            None
        })
        .filter_map(Result::ok)
        .map(|file| {
            ReaderBuilder::new()
                //.buffer_capacity(1000*1000*1000 *2) // 1GB
                .has_headers(true) // Assuming the file has a header row
                .quoting(true) // Enable quoting (handle fields with quotes and commas)
                .double_quote(true) // Handle double quotes correctly (escape quote with "")
                .from_reader(file)
        })
        .collect()
}

pub fn process_files(file_path: &str,max_records: usize, out_path: &str, out_name: &str){

    println!("FROM DIR {} TO OUT DIR {} with base name{}",file_path, out_path, out_name);

    let mut readers = get_csv_file_readers(file_path);

    // Assumming all have same headers
    let headers = readers[0]
        .headers()
        .expect("CSV file input had headers!")
        .clone();

    let mut file_count = 1;
    let mut record_count:usize = 0;
    let mut wtr = create_writer(out_path, out_name, file_count, &headers).expect("Could create sub csv in out dir");


    for record in readers.iter_mut()
            .flat_map(|reader| reader.records().filter_map(Result::ok)){

        if record_count >= max_records {
            file_count += 1;
            record_count = 0;
            wtr.flush().expect("Could flush content to writer"); // Make sure current file is flushed
            wtr = create_writer(out_path, out_name, file_count, &headers).expect("Could create sub csv in out dir");
        }

        wtr.write_record(&record).expect("Could write record to target csv");
        record_count += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        eprintln!(
            "Se debe ejecutar como: {} <src_dir> <max_records> <out_dir> <out_base_name>",
            args[0]
        );
        std::process::exit(1);
    }
    let dir_path = &args[1];
    let max_rec = &args[2];
    let out_dir = &args[3];
    let base_name = &args[4];



    let max_records: usize = max_rec
        .parse()
        .expect("No se pudo convertir a numero la cantidad de records per file");
    process_files(dir_path, max_records, out_dir, base_name);
}
