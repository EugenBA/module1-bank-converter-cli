mod converter;

use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use crate::converter::pipline::*;

fn main() {
    // Получаем аргументы командной строки
    let mut args: Vec<String> = env::args().collect();
    // Если аргументов недостаточно, показываем справку
    if args.len() < 2 {
        eprintln!("Использование:");
        eprintln!("  -i <file name>");
        eprintln!("  -o <file name>");
        eprintln!("  --in_format CSV|XML|MT940|CAMT053");
        eprintln!("  --out_format CSV|XML|MT940|CAMT053");
        return;
    }
    let mut converter = PipelineConverter::default();
    let mut in_file = String::new();
    let mut out_file = String::new();
    while args.len() > 2
    {
        match args.remove(1).as_str(){
            "-i" => {
                in_file = args.remove(1);
            }
            "-o" => {
                out_file = args.remove(1);
            }
            "--in_format" => {
                let format = args.remove(1);
                converter.data_in = PipelineConverter::get_format_type_from_string(&format);
            }
            "--out_format" => {
                let format = args.remove(1);
                converter.data_out = PipelineConverter::get_format_type_from_string(&format);
            }
            arg => {
                eprintln!("Неизвестная команда: {}", arg);
                return;
            }
        }
    }
    if in_file.is_empty() || out_file.is_empty()  {
        eprintln!("Не указаны входной или выходной файл");
        return;
    }
    if converter.data_in == FormatType::None ||
        converter.data_out == FormatType::None {
        eprintln!("Не указаны форматы входного или выходного файла");
        return;
    }
    if converter.data_in == converter.data_out{
        eprintln!("Выбран один и тот же формат для входного и выходного файлов");
        return;
    }
    if !Path::new(&in_file).exists() {
        eprintln!("Файл {} не существует", in_file);
        return;
    }
    let mut reader = BufReader::new(File::open(in_file).unwrap());
    let mut writer = BufWriter::new(File::create(out_file).unwrap());
    if let Err(e) = converter.convert(&mut reader, &mut writer){
        eprintln!("{}", e);
        return;
    }
    print!("Конвертация успешна!")
}
