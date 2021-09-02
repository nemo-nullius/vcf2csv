mod vcf2csv;
use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("A little tool for conversion of *.vcf files to *.csv files.\nDeveloped by Nemo<nemoou@outlook.com>.\nUsage:\nvcf2csv.exe <filename for conversion>");
        return;
    }

    let infilename = &args[1];
    let outfilename = format!("{}{}", infilename, ".csv");

    let vcf_raw = match fs::read_to_string(&infilename) {
        Ok(x) => x,
        Err(err) => {
            println!("[ERROR] {}", err);
            return;
        }
    };
    let vcf_lines: Vec<&str> = vcf_raw.lines().collect();
    let vcf_headers = match vcf2csv::get_vcf_headers(&vcf_lines) {
        Ok(x) => x,
        Err(err) => {
            println!("[ERROR] {}", err);
            return;
        }
    };
    //DEBUG println!("{:?}", vcf_headers);
    let csv_body_vec = vcf2csv::get_csv_body_vec(&vcf_lines, &vcf_headers);
    //DEBUG println!("{:?}", &csv_body_vec[0..200]);
    let csv_body_string = vcf2csv::get_csv_body_string(&csv_body_vec);
    //println!("{:?}", csv_body_string);
    let result = vcf_headers.join(",") + "\n" + &csv_body_string;

    //let mut file = fs::File::create(&outfilename).expect("Save file error.");
    let mut file = match fs::File::create(&outfilename) {
        Ok(x) => x,
        Err(err) => {
            println!("[ERROR] {}", err);
            return;
        }
    };

    match file.write_all(result.as_bytes()) {
        Ok(_) => {}
        Err(err) => {
            println!("[ERROR] {}", err);
            return;
        }
    }

    println!("Conversion finished. Output located in {}", &outfilename);
}
