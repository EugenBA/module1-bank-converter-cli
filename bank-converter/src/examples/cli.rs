use std::{env};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;
use bank_converter::errors::{ConvertError};
use bank_converter::models::camt053::{DocumentCamt053};
use bank_converter::models::mt940::{DocumentMt940};
use bank_converter::models::csv::{DocumentCsv};

#[derive(PartialEq)]
enum FormatType {
    None,
    Csv,
    Xml,
    Mt940,
    Camt053,
}

enum Document{
    DocumentCamt053(DocumentCamt053),
    DocumentMt940(DocumentMt940),
    DocumentCsv(DocumentCsv),
}

struct PipelineConverter{
     data_in: FormatType,
     data_out: FormatType
}


impl  PipelineConverter {
    pub fn get_format_type_from_string(format_str: &String) -> FormatType {
        match format_str.to_lowercase().as_str() {
            "csv" | "CSV" => FormatType::Csv,
            "xml" | "XML" => FormatType::Xml,
            "mt940" | "MT940" => FormatType::Mt940,
            "camt053" | "CAMT053" => FormatType::Camt053,
            _ => FormatType::None
        }
    }
    pub fn default() -> Self {
        Self {
            data_in: FormatType::None,
            data_out: FormatType::None
        }
    }
    fn read_document<T:Read>(&self, r: &mut T) -> Result<Document, ConvertError> {
        match self.data_in {
            FormatType::None => {
                Err(ConvertError::BadArgument("Not support input format".to_string()))
            }
            FormatType::Csv => {
                Ok(Document::DocumentCsv(DocumentCsv::from_read(r)?))
            }
            FormatType::Mt940 => {
                Ok(Document::DocumentMt940(DocumentMt940::from_read(r)?))
            }
            FormatType::Camt053 | FormatType::Xml => {
                Ok(Document::DocumentCamt053(DocumentCamt053::from_read(r)?))
            }
        }
    }
    fn convert<T:Read, W:Write>(&self, r: &mut T, w: &mut W) -> Result<(), ConvertError> {
        let document = self.read_document(r)?;
        let mut camt = match document {
            Document::DocumentCamt053(doc) => doc,
            Document::DocumentMt940(doc) => { DocumentCamt053::from(doc)},
            Document::DocumentCsv(doc) => { DocumentCamt053::from(doc)},
        };
        match self.data_out {
            FormatType::None => { Err(ConvertError::WriteError("Bad output format".to_string())) }
            FormatType::Csv => {
                let mut csv = DocumentCsv::from(camt);
                return csv.write_to(w);
            }
            FormatType::Mt940 => {
                let mut mt940 = DocumentMt940::from(camt);
                return mt940.write_to(w);
            }
            FormatType::Camt053 | FormatType::Xml => {
                return camt.write_to(w);
            }
        }?;
        Ok(())
    }

}


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