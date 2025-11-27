use std::io::{Read};
use crate::errors::ParserError;
use crate::models::camt053::{DocumentCamt053};
use crate::models::mt940::{DocumentMt940};
use crate::models::csv::{DocumentCsv, RowCsv};
use csv::{ReaderBuilder};
use regex::{Regex};


impl DocumentCamt053 {
    pub fn new() -> Self {
        DocumentCamt053::default()
    }
    /// Читает файл формата CAMT053 и возвращает его содержимое.
    ///
    /// # Аргументы
    ///
    /// * `r` - reader (любой тип реал изующий терейт Read)
    ///
    /// # Возвращает
    ///
    /// `Ok(DocumentCamt053)` с содержимым файла в случае успеха,
    /// `Err(ParseError)` в случае ошибки.
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если:
    /// * Ошибка чтения файла
    /// * Неверный формат файла
    /// * Неверный входной тип
    /// * Ошибка десерелизации xml
    ///
    pub fn from_read<R: Read>(r: &mut R) -> Result<Self, ParserError> {
        let mut xml_str = String::new();
        r.read_to_string(&mut xml_str)?;
        if let Some(xml_str) = DocumentCamt053::remove_name_space(&xml_str){
            Ok(serde_xml_rs::from_str(&xml_str)?)
        }
        else {
            Err(ParserError::BadInputFormatFile("Error parse CAMT053 document".to_string()))
        }
    }

    fn remove_name_space(xml: &str) -> Option<String>{
        if let Ok(regexp) = Regex::new(r#"xmlns[= \w/\d:".-]+"#){
            let result = regexp.replace_all(xml, "");
            return Some(result.to_string());
        }
        None

    }

}

impl DocumentMt940 {
    /// Читает файл формата MT940 и возвращает его содержимое.
    ///
    /// # Аргументы
    ///
    /// * `r` - reader (любой тип реал изующий терейт Read)
    ///
    /// # Возвращает
    ///
    /// `Ok(DocumentMT940)` с содержимым файла в случае успеха,
    /// `Err(ParseError)` в случае ошибки.
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если:
    /// * Ошибка чтения файла
    /// * Неверный формат файла
    /// * Неверный входной тип
    ///
    pub fn from_read<R: Read>(r: &mut R) -> Result<Self, ParserError> {
        let mut regex_pattern = String::new();
        r.read_to_string(&mut regex_pattern)?;
        if let Some(records) = DocumentMt940::find_record(&regex_pattern) {
            for record in records {
                DocumentMt940::parse_one_record(&regex_pattern[record.0..record.1]);
            }
        }
        Err(ParserError::BadInputFormatFile("No implement parse document".to_string()))
    }
}

impl DocumentCsv {
    /// Читает файл формата CSV и возвращает его содержимое.
    ///
    /// # Аргументы
    ///
    /// * `r` - reader (любой тип реал изующий терейт Read)
    ///
    /// # Возвращает
    ///
    /// `Ok(DocumentCsv)` с содержимым файла в случае успеха,
    /// `Err(ParseError)` в случае ошибки.
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если:
    /// * Ошибка чтения файла
    /// * Неверный формат файла
    /// * Неверный входной тип
    /// * Ошибка десерилизации Csv
    ///
    pub fn from_read<R: Read>(r: &mut R) -> Result<Self, ParserError> {
        let mut csv_document: DocumentCsv = DocumentCsv::new();
        let mut csv_rdr = ReaderBuilder::new().has_headers(false)
                                                            .from_reader(r);
        for row in csv_rdr.deserialize() {
            let row_data: RowCsv = row?;
            csv_document.rows.push(row_data);
        }
        Ok(csv_document)
    }
}
