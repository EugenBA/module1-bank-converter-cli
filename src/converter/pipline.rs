use std::io::{Read, Write};
use bank_converter::errors::ConvertError;
use bank_converter::models::camt053::DocumentCamt053;
use bank_converter::models::csv::DocumentCsv;
use bank_converter::models::mt940::DocumentMt940;

#[derive(PartialEq)]
pub(crate) enum FormatType {
    None,
    Csv,
    Xml,
    Mt940,
    Camt053,
}

pub(crate) enum Document{
    DocumentCamt053(DocumentCamt053),
    DocumentMt940(DocumentMt940),
    DocumentCsv(DocumentCsv),
}

pub(crate) struct PipelineConverter{
    pub(crate) data_in: FormatType,
    pub(crate) data_out: FormatType
}


impl  PipelineConverter {
    pub(crate) fn get_format_type_from_string(format_str: &String) -> FormatType {
        match format_str.to_lowercase().as_str() {
            "csv" | "CSV" => FormatType::Csv,
            "xml" | "XML" => FormatType::Xml,
            "mt940" | "MT940" => FormatType::Mt940,
            "camt053" | "CAMT053" => FormatType::Camt053,
            _ => FormatType::None
        }
    }
    pub(crate) fn default() -> Self {
        Self {
            data_in: FormatType::None,
            data_out: FormatType::None
        }
    }
    pub(crate) fn read_document<T:Read>(&self, r: &mut T) -> Result<Document, ConvertError> {
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
    pub(crate) fn convert<T:Read, W:Write>(&self, r: &mut T, w: &mut W) -> Result<(), ConvertError> {
        let document = self.read_document(r)?;
        let mut camt = match document {
            Document::DocumentCamt053(doc) => doc,
            Document::DocumentMt940(doc) => { DocumentCamt053::try_from(doc)?},
            Document::DocumentCsv(doc) => { DocumentCamt053::try_from(doc)?},
        };
        match self.data_out {
            FormatType::None => { Err(ConvertError::WriteError("Bad output format".to_string())) }
            FormatType::Csv => {
                let mut csv = DocumentCsv::try_from(camt)?;
                return csv.write_to(w);
            }
            FormatType::Mt940 => {
                let mut mt940 = DocumentMt940::try_from(camt)?;
                return mt940.write_to(w);
            }
            FormatType::Camt053 | FormatType::Xml => {
                return camt.write_to(w);
            }
        }?;
        Ok(())
    }

}