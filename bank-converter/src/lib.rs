#![warn(missing_docs)]
//! Библиотека для работы с банковскими выписками форматов CAMT.053, MT940, CSV, XML.
//!
//! Предоставляет функциональность для чтения, парсинга и конвертации между форматами.
pub mod errors;
pub(crate) mod converter;
pub mod models;
mod macros;

