use crate::errors::ParserError;
use crate::models::camt053::DocumentCamt053;
use crate::models::csv::DocumentCsv;
use crate::models::mt940::DocumentMt940;


impl TryFrom<DocumentCamt053> for DocumentMt940 {
    type Error = ParserError;
    fn try_from(camt053: DocumentCamt053) -> Result<Self, Self::Error> {
        Ok(Self {
            document: camt053.bk_to_cstmr_stmt
        })
    }
}

impl TryFrom<DocumentCsv> for DocumentMt940 {
    type Error = ParserError;
    fn try_from(csv: DocumentCsv) -> Result<Self, Self::Error> {
        let camt = DocumentCsv::parse_to_camt(csv)?;
        Ok(DocumentMt940::try_from(camt)?)
    }
}
impl TryFrom<DocumentMt940> for DocumentCamt053 {
    type Error = ParserError;
    fn try_from(mt940: DocumentMt940) -> Result<Self, Self::Error> {
        let mut camt = Self::default();
        camt.bk_to_cstmr_stmt = mt940.document;
        Ok(camt)
    }
}

impl TryFrom<DocumentCsv> for DocumentCamt053 {
    type Error = ParserError;
    fn try_from(csv: DocumentCsv) -> Result<Self, Self::Error> {
        Ok(DocumentCsv::parse_to_camt(csv)?)
    }
}

impl TryFrom<DocumentCamt053> for DocumentCsv {
    type Error = ParserError;
    fn try_from(camt053: DocumentCamt053) -> Result<Self, Self::Error> {
        Ok(DocumentCsv::parse_to_csv(&camt053)?)
    }
}

impl TryFrom<DocumentMt940> for DocumentCsv {
    type Error = ParserError;
    fn try_from(mt940: DocumentMt940) -> Result<Self, Self::Error> {
        let camt = DocumentCamt053::try_from(mt940)?;
        Ok(DocumentCsv::parse_to_csv(&camt)?)
    }
}
