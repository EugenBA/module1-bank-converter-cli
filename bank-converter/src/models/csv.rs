use regex::Regex;
use serde::Serialize;
use serde::Deserialize;
use crate::csv_data;
use crate::errors::ParserError;
use crate::models::camt053::{BalanceAttribute, BkToCstmrStmt, DocumentCamt053,
                             NtryAttribute, TxDtlsAttribute};
use chrono::{Local};

pub struct DocumentCsv {
    pub(crate) rows: Vec<RowCsv>
}
csv_data!(RowCsv, String, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u);


impl DocumentCsv {
    pub(crate) fn new() -> Self{
        Self{rows: Vec::new()}
    }

    fn extract_time(val: &str) -> Option<String>{
        let reg_pattern = Regex::new(r"(\d{2}:\d{2}:\d{2})");
        if let Ok(regexp) = reg_pattern {
            if let Some(capture) = regexp.captures(val)
            {
                let dt = capture[1].to_string();
                return Some(dt);
            }
        }
        None
    }
    fn extract_date(val: &str) -> Option<String> {
        let reg_pattern = Regex::new(r"(\d{2}.\d{2}.\d{4})");
        if let Ok(regexp) = reg_pattern {
            if let Some(capture) = regexp.captures(val)
            {
                let mut dt = String::new();
                dt.push_str(&capture[1][6..10]);
                dt.push_str("-");
                dt.push_str(&capture[1][3..5]);
                dt.push_str("-");
                dt.push_str(&capture[1][0..2]);
                return Some(dt);
            }
        }
        None
    }
    fn convert_ru_month_to_number(val: &str) -> Option<String> {
        let number_month = val.replace("января", "01")
            .replace("февраля", "02")
            .replace("марта", "03")
            .replace("апреля", "04")
            .replace("мая", "05")
            .replace("июня", "06")
            .replace("июля","07")
            .replace("августа", "08")
            .replace("сентября", "09")
            .replace("октября", "10")
            .replace("ноября", "11")
            .replace("декабря", "12");
        if number_month.len() == 2 {
            return Some(number_month);
        }
        None
    }

    fn extract_date_rus(val: &str) -> Option<String> {
        let reg_pattern = Regex::new(r"(\d{2}).(января|февраля|марта|апреля|мая|июня|июля|августа|сентября|октября|ноября|декабря).(\d{4})");
        if let Ok(regexp) = reg_pattern {
            if let Some(capture) = regexp.captures(val)
            {
                let mut dt = String::new();
                dt.push_str(&capture[3]);
                dt.push_str("-");
                if let Some(month) = Self::convert_ru_month_to_number(&capture[2]){
                    dt.push_str(&month);
                }
                dt.push_str("-");
                dt.push_str(&capture[1]);
                return Some(dt);
            }
        }
        None
    }

    fn extract_crd_agent(val: &str, ntry_det: &mut TxDtlsAttribute) {
        let reg_pattern = Regex::new(r"(\d+) ([\w ]+), (.+)");
        if let Ok(regexp) = reg_pattern {
            if let Some(capture) = regexp.captures(val)
            {
                ntry_det.rltd_agts.dbtr_agt.fin_instn_id.bic = capture[1].to_string();
                ntry_det.rltd_agts.dbtr_agt.fin_instn_id.nm = capture[2].to_string();
            }
        }
    }

    fn extract_ccy(val: &str) -> Option<String>{
        let ccy = val.replace("Российский рубль", "RUB")
            .replace("Доллар США", "USD")
            .replace("Евро", "EUR");
        if ccy.len() == 3 {
            return Some(ccy);
        }
        None
    }

    pub(crate) fn parse_to_camt(self) -> Result<DocumentCamt053, ParserError>{
        let mut camt = DocumentCamt053::new();
        let mut camt_bk_to_cstm = BkToCstmrStmt::default();
        if self.rows.len() < 8 {
            return Err(ParserError::BadInputFormatFile("Bad input csv file".to_string()))
        }
        if let Some(date_create) = DocumentCsv::extract_date(&self.rows[3].b){
            camt_bk_to_cstm.grp_hdr.cre_dt_tm = date_create;
            if let Some(time_create) = DocumentCsv::extract_time(&self.rows[3].b){
                camt_bk_to_cstm.grp_hdr.cre_dt_tm.push_str("T");
                camt_bk_to_cstm.grp_hdr.cre_dt_tm.push_str(&time_create);
            }
        }
        camt_bk_to_cstm.stmt.acct.id.othr.id = self.rows[4].m.to_string();
        camt_bk_to_cstm.stmt.acct.ownr.nm = self.rows[5].m.to_string();
        if let Some(dt) = DocumentCsv::extract_date_rus(&self.rows[6].c) {
            camt_bk_to_cstm.stmt.fr_to_dt.fr_dt_tm = dt + "T00:00:00";
        }
        if let Some(dt) = DocumentCsv::extract_date_rus(&self.rows[6].p){
            camt_bk_to_cstm.stmt.fr_to_dt.to_dt_tm= dt + "T23:59:59";
        }
        if let Some(ccy) = DocumentCsv::extract_ccy(&self.rows[7].c){
            camt_bk_to_cstm.stmt.acct.ccy =ccy;
        }
        for index_row in 11..self.rows.len() {
            let row = &self.rows[index_row];
            if row.b.is_empty() {
                break;
            }
            let mut ntry = NtryAttribute::default();
            let mut ntry_det = TxDtlsAttribute::default();
            ntry.amt.ccy = camt_bk_to_cstm.stmt.acct.ccy.clone();
            if let Some(date) = DocumentCsv::extract_date(&row.b){
                ntry.val_dt.dt = date;
                ntry.bookg_dt.dt = ntry.val_dt.dt.clone();
            }
            if row.j.is_empty(){
                ntry.cdt_dbt_ind = "CDIT".to_string();
                ntry.amt.amt = row.n.to_string();
            }
            else{
                ntry.cdt_dbt_ind =  "DBIT".to_string();
                ntry.amt.amt = row.j.to_string();
            }
            ntry.bk_tx_cd.prtry.cd = row.q.to_string();
            ntry.bk_tx_cd.prtry.issr = self.rows[2].b.to_string();
            ntry.acct_svcr_ref = row.o.to_string();
            ntry_det.refs.end_to_end_id ="1".to_string();
            let debit_detals: Vec<&str> = row.e.split("\n").collect();
            if debit_detals.len() == 3{
                ntry_det.rltd_pties.dbtr.nm = debit_detals[2].to_string();
                ntry_det.rltd_pties.dbtr.id.othr.id = debit_detals[1].to_string();
                ntry_det.rltd_pties.dbtr_acct.other.id = debit_detals[0].to_string();
            }
            let credit_detals: Vec<&str> = row.i.split("\n").collect();
            if credit_detals.len() == 3{
                ntry_det.rltd_pties.cdtr.nm = debit_detals[2].to_string();
                ntry_det.rltd_pties.cdtr.id.othr.id = debit_detals[1].to_string();
                ntry_det.rltd_pties.cdtr_acct.other.id = debit_detals[0].to_string();
            }
            DocumentCsv::extract_crd_agent(&row.r, & mut ntry_det);
            ntry_det.rmt_inf.ustrd.push(row.u.to_string());
            ntry.ntry_dtls.tx_dtls.push(ntry_det);
            camt_bk_to_cstm.stmt.ntry.push(ntry);
        }
        let next_row = self.rows.len() - 4; //offset balance data from end document
        let mut balance_opbd = BalanceAttribute::default();
        balance_opbd.amt.ccy = camt_bk_to_cstm.stmt.acct.ccy.clone();
        balance_opbd.tp.cd_or_prtry.cd = "OPDB".to_string();
        balance_opbd.amt.amt = self.rows[next_row+1].h.to_string();
        camt_bk_to_cstm.stmt.bal.push(balance_opbd);
        camt_bk_to_cstm.stmt.txs_summry.ttl_dbt_ntries.sum = self.rows[next_row+2].h.to_string();
        camt_bk_to_cstm.stmt.txs_summry.ttl_cdt_ntries.sum = self.rows[next_row+2].l.to_string();
        camt_bk_to_cstm.stmt.txs_summry.ttl_ntries.nb_of_ntries = self.rows[next_row].l.to_string();
        let mut balance_clbd = BalanceAttribute::default();
        balance_clbd.amt.ccy = camt_bk_to_cstm.stmt.acct.ccy.clone();
        balance_clbd.tp.cd_or_prtry.cd = "CLDB".to_string();
        balance_clbd.amt.amt = self.rows[next_row+3].l.to_string();
        camt_bk_to_cstm.stmt.bal.push(balance_clbd);
        camt.bk_to_cstmr_stmt.push(camt_bk_to_cstm);
        Ok(camt)
    }

    pub(crate)  fn parse_to_csv(camt: &DocumentCamt053) -> Result<Self, ParserError>{
        if let Some(doc) = camt.bk_to_cstmr_stmt.get(0) {
            let mut csv = DocumentCsv::new();
            let mut row_1 = RowCsv::new();
            row_1.b = format!("Дата формирования выписки: {}", Local::now().format("%d.%m.%Y %H:%M:%S"));
            csv.rows.push(row_1);
            let mut row_2 = RowCsv::new();
            row_2.b = "Выписка по лицевому счету".to_string();
            row_2.m = doc.stmt.acct.id.othr.id.clone();
            csv.rows.push(row_2);
            let mut row_3 = RowCsv::new();
            row_3.m = doc.stmt.acct.ownr.nm.clone();
            csv.rows.push(row_3);
            let mut row_4 = RowCsv::new();
            row_4.c = format!("За период с {}", doc.stmt.fr_to_dt.fr_dt_tm);
            row_4.o = "по".to_string();
            row_4.p = doc.stmt.fr_to_dt.to_dt_tm.clone();
            csv.rows.push(row_4);
            let mut row_5 = RowCsv::new();
            row_5.c = doc.stmt.acct.ccy.clone();
            csv.rows.push(row_5);
            let mut row_6 = RowCsv::new();
            row_6.b = "Дата проводки".to_string();
            row_6.e = "Счет".to_string();
            row_6.j = "Сумма по дебету".to_string();
            row_6.n = "Сумма по кредиту".to_string();
            row_6.o = "№ документа".to_string();
            row_6.q = "ВО".to_string();
            row_6.r = "Банк (БИК и наименование)".to_string();
            row_6.u = "Назначение платежа".to_string();
            csv.rows.push(row_6);
            let mut row_7 = RowCsv::new();
            row_7.e = "Дебет".to_string();
            row_7.i = "Кредит".to_string();
            csv.rows.push(row_7);
            for ntry in &doc.stmt.ntry{
                let mut row = RowCsv::new();
                row.b = ntry.bookg_dt.dt.clone();
                if ntry.cdt_dbt_ind == "CDIT"{
                    row.n = ntry.amt.amt.clone();
                }
                if ntry.cdt_dbt_ind == "DBIT"{
                    row.j = ntry.amt.amt.clone();
                }
                row.q = ntry.bk_tx_cd.prtry.cd.clone();
                row.o = ntry.acct_svcr_ref.clone();
                if let Some(ntry_det) = ntry.ntry_dtls.tx_dtls.get(0)
                {
                    row.e = format!("{}\n{}\n{}",
                                    ntry_det.rltd_pties.dbtr_acct.other.id,
                                    ntry_det.rltd_pties.dbtr.id.othr.id,
                                    ntry_det.rltd_pties.dbtr.nm);
                    row.i = format!("{}\n{}\n{}",
                                    ntry_det.rltd_pties.cdtr_acct.other.id,
                                    ntry_det.rltd_pties.cdtr.id.othr.id,
                                    ntry_det.rltd_pties.cdtr.nm);
                    row.r = format!("БИК {}, {}",
                                   ntry_det.rltd_agts.dbtr_agt.fin_instn_id.bic,
                                   ntry_det.rltd_agts.dbtr_agt.fin_instn_id.nm);
                    let mut ustrd_all = String::new();
                    for ustrd in ntry_det.rmt_inf.ustrd.clone(){
                        ustrd_all.push_str(&ustrd);
                        ustrd_all.push_str(",");
                    }
                    row.u = ustrd_all;
                }
                csv.rows.push(row);
            }
            let mut row_8 = RowCsv::new();
            row_8.b = "б/с".to_string();
            row_8.h = "Дебет".to_string();
            row_8.l = "Кредит".to_string();
            row_8.t = "Всего".to_string();
            csv.rows.push(row_8);
            let mut row_9 = RowCsv::new();
            row_9.b = "Количество операций".to_string();
            row_9.l = doc.stmt.txs_summry.ttl_ntries.nb_of_ntries.clone();
            csv.rows.push(row_9);
            for bal in &doc.stmt.bal{
                if bal.tp.cd_or_prtry.cd == "OPDB"{
                    let mut row = RowCsv::new();
                    row.b = "Входящий остаток".to_string();
                    row.h = bal.amt.amt.clone();
                    csv.rows.push(row);
                    let mut row_10 = RowCsv::new();
                    row_10.b = "Итого оборотов".to_string();
                    row_10.h = doc.stmt.txs_summry.ttl_dbt_ntries.sum.clone();
                    row_10.l = doc.stmt.txs_summry.ttl_cdt_ntries.sum.clone();
                    csv.rows.push(row_10);
                }
                if bal.tp.cd_or_prtry.cd == "CLDB" {
                    let mut row = RowCsv::new();
                    row.b = "Исходящий остаток".to_string();
                    row.l = bal.amt.amt.clone();
                    csv.rows.push(row);
                }
            }
            return Ok(csv);
        }
        Err(ParserError::BadCsvDeserializeError("No document to convert CSV format".to_string()))
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_extract_time(){
        let test = "time 12:34:56";
        assert_eq!("12:34:56", DocumentCsv::extract_time(test).unwrap());
    }

    #[test]
    fn test_extract_date(){
        let test = "date 12.01.2021";
        assert_eq!("2021-01-12", DocumentCsv::extract_date(test).unwrap());
    }

    #[test]
    fn test_convert_ru_month_to_number(){
        let test = "января";
        assert_eq!("01", DocumentCsv::convert_ru_month_to_number(test).unwrap());
    }

    #[test]
    fn test_extract_date_rus(){
        let test = "10 октября 2023";
        assert_eq!("2023-10-10", DocumentCsv::extract_date_rus(test).unwrap());
    }

    #[test]
    fn test_extract_crd_agent(){
        let mut test = TxDtlsAttribute::default();
        let data = "БИК 044525545 АО ЮниКредит Банк, г.Москва";
        test.rltd_agts.dbtr_agt.fin_instn_id.bic = "044525545".to_string();
        test.rltd_agts.dbtr_agt.fin_instn_id.nm = "АО ЮниКредит Банк".to_string();
        let mut result = TxDtlsAttribute::default();
        DocumentCsv::extract_crd_agent(&data, &mut result);
        assert_eq!(test, result);
    }

    #[test]
    fn test_extract_ccy(){
        let test = "Доллар США";
        assert_eq!("USD", DocumentCsv::extract_ccy(test).unwrap());
    }
}