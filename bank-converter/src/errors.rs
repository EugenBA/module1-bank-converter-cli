//! Модуль для реализации обработки ошибок 
//!
//! Предоставляет функциональность по обработке ошибок чтения, конвертации и записи.

use std::fmt::{Display, Formatter};
use std::io::Error;
use serde::Deserialize;
use thiserror::Error;
/// Перчсисление для ошибок конвертации
/// Ошибки входных аргументов парсера
/// Ошибки конвертации
/// Ошибки запис сконвертировнных данных

#[derive(Error, Debug)]
pub enum ConvertError
{
    /// Ошибки входных аргументов
    BadArgument(String),
    /// Ошибки конвертации
    ParseError(String),
    /// Ошибки записи сконвертировнных данных
    WriteError(String)
}

/// Перчисление ошибо чтеняи и парсинга входныхх данных
/// Ошибки чтения файлов
/// Ошибка формата входных файлов
/// Ошибка xml десерилизации
/// Ошибка CSV десерелизации
///
#[derive(Error, Debug, Deserialize)]
pub enum ParserError
{
    /// Ошибки чтения файлов
    FileReadError(String),
    /// Ошибка формата входных файлов
    BadInputFormatFile(String),
    /// Ошибка xml десерилизации
    BadXmlDeserializeError(String),
    /// Ошибка CSV десерелизации
    BadCsvDeserializeError(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::FileReadError(s) => write!(f, "File read error: {}", s),
            ParserError::BadInputFormatFile(s) => write!(f, "Bad input format file: {}", s),
            ParserError::BadCsvDeserializeError(s) => write!(f, "Csv format deserialize error: {}", s),
            ParserError::BadXmlDeserializeError(s) => write!(f, "Xml format deserialize error: {}", s),
        }
    }
}

impl  From<serde_xml_rs::Error> for ParserError{
    fn from(err: serde_xml_rs::Error) -> Self{
        ParserError::BadXmlDeserializeError(err.to_string())
    }
}

impl  From<csv::Error> for ParserError {
    fn from(err: csv::Error) -> Self {
        ParserError::BadCsvDeserializeError(err.to_string())
    }
}

impl From<std::io::Error> for ParserError {
    fn from(value: Error) -> Self {
        ParserError::FileReadError(value.to_string())
    }
}

impl From<ParserError> for ConvertError{
    fn from(err: ParserError) -> Self {
        ConvertError::ParseError(err.to_string())
    }
}
impl Display for ConvertError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ConvertError::BadArgument(s) => write!(f, "Bad argument: {}", s),
            ConvertError::ParseError(s) => write!(f, "Parse error: {}", s),
            ConvertError::WriteError(s) => write!(f, "Write error: {}", s),
        }
    }
}

impl  From<serde_xml_rs::Error> for ConvertError{
    fn from(err: serde_xml_rs::Error) -> Self {
        ConvertError::WriteError(err.to_string())
    }
}

impl From<std::io::Error>  for ConvertError{
    fn from(value: Error) -> Self {
        ConvertError::WriteError(value.to_string())
    }

}

impl From<csv::Error> for ConvertError {
    fn from(err: csv::Error) -> Self {
        ConvertError::WriteError(err.to_string())
    }

}


