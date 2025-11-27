use std::io::{Write};
use csv::{WriterBuilder};
use serde_xml_rs::to_string;
use crate::errors::{ConvertError};
use crate::models::camt053::{DocumentCamt053};
use crate ::models::mt940::{DocumentMt940};
use crate::models::csv::{DocumentCsv};


impl DocumentCamt053 {
    /// Сохраняет файл формата CAMT053.
    ///
    /// # Аргументы
    ///
    /// * `w` - writer (любой тип реал изующий терейт Write)
    ///
    /// # Возвращает
    ///
    /// `Ok(())` с содержимым файла в случае успеха,
    /// `Err(ConvertError)` в случае ошибки.
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если:
    /// * Ошибка записи файла
    ///
    pub fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<(), ConvertError> {
        //let mut record_write = String::new();
        let mut record_write = to_string(&self)?;
        record_write = record_write.replace("<?xml version=\"1.0\" encoding=\"UTF-8\"?><Document>",
                                            "<Document xmlns=\"urn:iso:std:iso:20022:tech:xsd:camt.053.001.02\" \
                                            xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" \
                                            xsi:schemaLocation=\"urn:iso:std:iso:20022:tech:xsd:camt.053.001.02 camt.053.001.02.xsd\">");
        writer.write_all(record_write.as_bytes())?;
        writer.flush()?;
        Ok(())
    }
}

impl DocumentMt940 {
    /// Сохраняет файл формата MT940.
    ///
    /// # Аргументы
    ///
    /// * `w` - writer (любой тип реал изующий терейт Write)
    ///
    /// # Возвращает
    ///
    /// `Ok(())` с содержимым файла в случае успеха,
    /// `Err(ConvertError)` в случае ошибки.
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если:
    /// * Ошибка записи файла
    ///
    pub fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<(), ConvertError> {
        let mut record_write = String::new();
        for record in &self.document {
            record_write.push_str("{1:F01");
            record_write.push_str(&record.stmt.acct.svcr.fin_instn_id.bic);
            record_write.push_str("}\n");
            record_write.push_str("{2:");
            record_write.push_str(&record.grp_hdr.msg_id);
            record_write.push_str("}\n{3:}\n{4:\n");
            record_write.push_str(":20:");
            record_write.push_str(&record.grp_hdr.msg_id);
            record_write.push_str("\n");
            record_write.push_str(":25:");
            record_write.push_str(&record.stmt.acct.ownr.id.org_id.othr.id);
            record_write.push_str("\n");
            record_write.push_str(":28C:");
            record_write.push_str(&record.stmt.elctrnc_seq_nb);
            record_write.push_str("/");
            record_write.push_str(&record.stmt.lgl_seq_nb);
            record_write.push_str("\n");
            DocumentMt940::extract_field_6x_mt940(&record, &mut record_write);
            DocumentMt940::extract_field_61_86_mt940(&record.stmt.ntry, &mut record_write);
            record_write.push_str("}\n{5:-}\n");
            writer.write_all(record_write.as_bytes())?;
            writer.flush()?;
            record_write.clear();
        }
        Ok(())
    }
}

impl DocumentCsv {
    /// Сохраняет файл формата CSV.
    ///
    /// # Аргументы
    ///
    /// * `w` - writer (любой тип реал изующий терейт Write)
    ///
    /// # Возвращает
    ///
    /// `Ok(())` с содержимым файла в случае успеха,
    /// `Err(ConvertError)` в случае ошибки.
    ///
    /// # Ошибки
    ///
    /// Возвращает ошибку, если:
    /// * Ошибка записи файла
    ///
    pub fn write_to<W: Write>(&mut self, writer: &mut W) -> Result<(), ConvertError> {
        let mut csv_wrt = WriterBuilder::new().has_headers(false).from_writer(writer);
        for row in &self.rows {
            csv_wrt.serialize(row)?;
            csv_wrt.flush()?;
        }
        Ok(())
    }
}
