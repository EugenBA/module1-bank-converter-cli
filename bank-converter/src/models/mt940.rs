use regex::Regex;
use crate::models::camt053::{BalanceAttribute, BkToCstmrStmt, DtAttribute, NtryAttribute,
                             NtryDtlsAttribute, TxDtlsAttribute};

pub struct DocumentMt940 {
    pub(crate) document: Vec<BkToCstmrStmt>
}

impl DocumentMt940 {

    pub(crate) fn find_record(document: &str) -> Option<Vec<(usize, usize)>> {
        let mut vec_start_pattern: Vec<usize> = Vec::new();
        let mut vec_end_pattern: Vec<usize> = Vec::new();
        let mut records: Vec<(usize, usize)> = Vec::new();
        for (index, _found_pattern) in document.match_indices("{1"){
            vec_start_pattern.push(index);
        }
        for (index, _found_pattern) in document.match_indices("{5:"){
            vec_end_pattern.push(index);
        }
        for i in vec_start_pattern.iter().enumerate(){
            records.push((vec_start_pattern[i.0], vec_end_pattern[i.0]));
        }
        Some(records)
    }

    fn parse_field_one(header: &str) -> String{
        let regex = Regex::new(r"F\d{2}([A-Z]*\d*[A-Z]*)\d").unwrap();
        if let Some(capture) = regex.captures(header) {
            capture[1].to_string()
        }
        else {
            "UNKNOW_BIC".to_string()
        }
    }

    fn parse_field_two(header: &str, document: &mut BkToCstmrStmt) {
        let regex = Regex::new(r"([IO])(\d{3})([\w\d]+)");
        if let Ok(regex) = regex {
            if let Some(capture) = regex.captures(header) {
                document.grp_hdr.msg_id = capture[3].to_string();
                document.stmt.id = capture[3].to_string() + "-940";
            }
        }
    }

    fn parse_field_balance(header: &str) -> Option<BalanceAttribute>{
        let regex = Regex::new(r"([CD])(\d{6})([A-Z]+)(\d+,\d+)");
        if let Ok(regex) = regex {
            if let Some(capture) = regex.captures(header) {
                let mut balance = BalanceAttribute::default();
                balance.dt= DtAttribute::format_dt(&capture[2]);
                balance.amt.ccy = capture[3].to_string();
                balance.amt.amt = capture[4].replace(",", ".").to_string();
                balance.cd = capture[1].to_string();
                return Some(balance);
            }
        }
        None
    }
    fn parse_field_61(field: (&str, &str), vault: &str, ntry: &mut NtryAttribute){
        let regex = Regex::new(r"(\d{6})(\d{4})([CD])(\d+,\d+)([A-Z]{4})(\w+)");
        if let Ok(regex) = regex {
            if let Some(capture) = regex.captures(field.0) {
                ntry.val_dt = DtAttribute::format_dt(&capture[1]);
                let dt =  capture[1][0 .. 2].to_string() + &capture[2][0..4].to_string();
                ntry.bookg_dt = DtAttribute::format_dt(&dt);
                ntry.bk_tx_cd.prtry.cd = capture[5].to_string();
                ntry.amt.amt = capture[4].replace(",", ".").to_string();
                ntry.amt.ccy = vault.to_string();
                ntry.cdt_dbt_ind  = if capture[3].to_string() == "C".to_string(){
                    "CRDT".to_string()
                } else { "DBIT".to_string()};
                let mut nxdet: NtryDtlsAttribute = NtryDtlsAttribute::default();
                DocumentMt940::parse_field_86(field.1, capture[6].to_string(), &mut nxdet);
                ntry.ntry_dtls = nxdet;
            }
        }
    }

    fn parse_field_86(field: &str, refs: String, ntrydet: &mut NtryDtlsAttribute) {
        let reg_pattern = Regex::new(r"/([A-Z]{4})/([\w]*)");
        if let Ok(regexp) = reg_pattern {
            let mut tlds: TxDtlsAttribute = TxDtlsAttribute::default();
            for capture in regexp.captures_iter(field){
                match &capture[1] {
                    "EREF" =>{
                        tlds.refs.end_to_end_id = capture[2].to_string();
                    },
                    "CRNM" =>{
                        tlds.rltd_pties.cdtr.nm = capture[2].to_string();
                    },
                    "CACT" => {
                        tlds.rltd_pties.cdtr_acct.other.id = capture[2].to_string();
                    },
                    "CBIC" => {
                        tlds.rltd_agts.cdtr_agt.fin_instn_id.bic = capture[2].to_string();
                    },
                    "REMI" =>{
                        tlds.rmt_inf.ustrd.push(capture[2].to_string());
                    },
                    "OPRP" =>{
                        tlds.addtl_tx_inf = capture[2].to_string();
                    },
                    "DACT" =>{
                        tlds.rltd_pties.dbtr_acct.other.id = capture[2].to_string();
                    },
                    "DBIC" =>{
                        tlds.rltd_pties.dbtr_acct.other.id = capture[2].to_string();
                    },
                    "OAMT" =>{
                        tlds.amt_dtls.amt = capture[2].to_string();
                    },
                    "DCID" =>{
                        tlds.rltd_pties.dbtr.id.othr.id = capture[2].to_string();
                    },
                    "NREF" => {
                        tlds.refs.prtry.refdt = capture[2].to_string();
                        tlds.refs.prtry.tp = "NREF".to_string();
                    },
                    _ => {}
                }
            }
            tlds.refs.end_to_end_id = refs;
            ntrydet.tx_dtls.push(tlds);
        }
    }

    fn parse_field_ntry(header: &str, vault: &str) -> Option<Vec<NtryAttribute>>{
        let mut reg_pattern = Regex::new(r":61:([\n\w\d ,/-]+):");
        let mut field_61: Vec<String> = Vec::new();
        let mut field_86: Vec<String> = Vec::new();
        let mut nxtry : Vec<NtryAttribute> = Vec::new();
        if let Ok(regexp) = reg_pattern {
            for capture in regexp.captures_iter(header){
                field_61.push(capture[1].to_string());
            }
        }
        reg_pattern = Regex::new(r":86:([\n\w\d ,/-]+)");
        if let Ok(regexp) = reg_pattern {
            for capture in regexp.captures_iter(header){
                field_86.push(capture[1].to_string());
            }
        }
        let unions: Vec<(String, String)> = field_61.into_iter().zip(field_86.into_iter()).collect();
        for union in unions.iter(){
            let mut ntry = NtryAttribute::default();
            DocumentMt940::parse_field_61((&union.0, &union.1), vault, &mut ntry);
            nxtry.push(ntry);
        }
        Some(nxtry)
    }

    fn parse_field_foo(header: &str, document: &mut BkToCstmrStmt) {
        let reg_codes = ["26", "25", "28C", "60F", "60M", "62F", "62M", "64", "65"];
        for reg_code in reg_codes.iter() {
            let reg_pattern = Regex::new(&format!(r":{}:([\n\w\d ,/-]+)",
                                                  reg_code));
            if let Ok(regexp) = reg_pattern{
                if let Some(capture) = regexp.captures(header){
                    let capture = capture[1].replace("\n", "")
                                                  .replace(" ", "");
                    match reg_code.as_ref() {
                        "26" => {
                            document.grp_hdr.msg_id = capture;
                            document.stmt.id = document.grp_hdr.msg_id.clone();
                        },
                        "25" => {
                            document.stmt.acct.ownr.id.org_id.othr.id = capture;
                        },
                        "28C" => {
                            let fields: Vec<&str> = capture.split('/').collect();
                            if fields.len() > 1{
                                document.stmt.elctrnc_seq_nb = fields[0].to_string();
                                document.stmt.lgl_seq_nb = fields[1].to_string();
                            }
                        },
                        "60F" | "60M" | "62F" | "62M" | "64"  | "65"=> {
                            if  let Some(mut balance) = DocumentMt940::parse_field_balance(&capture){
                                if *reg_code == "60F" {
                                    balance.tp.cd_or_prtry.cd = "OPBD".to_string();
                                }
                                if *reg_code == "60M" {
                                    balance.tp.cd_or_prtry.cd = "OPAV".to_string();
                                }
                                if *reg_code == "62F" {
                                    balance.tp.cd_or_prtry.cd = "CLBD".to_string();
                                }
                                if *reg_code == "62M" {
                                    balance.tp.cd_or_prtry.cd = "CLAV".to_string();
                                }
                                if *reg_code == "64" {
                                    balance.tp.cd_or_prtry.cd = "ITAV".to_string();
                                }
                                if *reg_code == "65" {
                                    balance.tp.cd_or_prtry.cd = "FPAV".to_string();
                                }
                                document.stmt.bal.push(balance);
                            }
                        },
                        _=>{}
                    }
                }
            }
        }
        let mut acc = "";
        if let Some(bal) = &document.stmt.bal.get(0){
            acc = &bal.amt.ccy;
        }
        if let Some(ntry) = DocumentMt940::parse_field_ntry(&header, acc){
            document.stmt.ntry = ntry;
        }
    }
    pub(crate) fn parse_one_record(document: &str) -> Option<BkToCstmrStmt> {
        let mut record: BkToCstmrStmt = BkToCstmrStmt::default();
        for field in 1..6 {
            let reg_pattern = Regex::new(&format!(r"\{{{}:([\n\w\d ,/:-]*)\}}",
                                                  field));
            if let Ok(regexp) = reg_pattern {
                match regexp.captures(document) {
                    Some(capture) => {
                        match field {
                            1 => { record.stmt.acct.svcr.fin_instn_id.bic =
                                DocumentMt940::parse_field_one(&capture[1].to_string());},
                            2 => {
                                DocumentMt940::parse_field_two(&capture[1].to_string(),
                                                               &mut record);
                            },
                            4 => { DocumentMt940::parse_field_foo(&capture[1].to_string(),
                                                                  &mut record); },
                            _ => {}
                        }
                    }
                    None => {}
                }
            }
        }
        Some(record)
    }
    pub(crate) fn extract_field_6x_mt940(record_camt: &BkToCstmrStmt, record_write: &mut String) {
        for balance in &record_camt.stmt.bal {
            match balance.tp.cd_or_prtry.cd.as_ref() {
                "OPBD" => { record_write.push_str(":60F:") },
                "OPAV" => { record_write.push_str(":60M:") },
                "CLBD" => { record_write.push_str(":62F:") },
                "CLAV" => { record_write.push_str(":62M:") },
                "ITAV" => { record_write.push_str(":64:") },
                "FPAV" => { record_write.push_str(":65:") },
                _ => { continue}
            }
            record_write.push_str(balance.cd.as_ref());
            let dt = balance.dt.dt.replace("-", "");
            if dt.len() >= 8 {
                record_write.push_str(&dt[2..8]);
            }
            record_write.push_str(balance.amt.ccy.as_ref());
            record_write.push_str(balance.amt.amt.replace(".", ",").as_ref());
            record_write.push_str("\n");
        }
    }
    pub(crate) fn extract_field_61_86_mt940(record_camt: &Vec<NtryAttribute>, record_write: &mut String) {
        for ntry in record_camt {
            record_write.push_str(":61:");
            let mut dt = ntry.val_dt.dt.replace("-", "");
            if dt.len() >= 8 {
                record_write.push_str(&dt[2..8]);
            }
            dt = ntry.bookg_dt.dt.replace("-", "");
            if dt.len() >= 8 {
                record_write.push_str(&dt[4..8]);
            }
            if ntry.cdt_dbt_ind == "CRDT" {
                record_write.push_str("C")
            } else { record_write.push_str("D") };
            record_write.push_str(ntry.amt.amt.replace(".", ",").as_ref());
            record_write.push_str(ntry.bk_tx_cd.prtry.cd.as_ref());
            if !ntry.ntry_dtls.tx_dtls.is_empty() {
                record_write.push_str(ntry.ntry_dtls.tx_dtls[0].refs.end_to_end_id.as_ref());
                record_write.push_str(" ");
            }
            record_write.push_str("\n");
            for tx_dtls in &ntry.ntry_dtls.tx_dtls {
                record_write.push_str(":86:/NREF/");
                record_write.push_str(tx_dtls.refs.end_to_end_id.as_ref());
                record_write.push_str("\n");
                if !tx_dtls.rltd_pties.cdtr.nm.is_empty() {
                    record_write.push_str("/CRNM/");
                    record_write.push_str(tx_dtls.rltd_pties.cdtr.nm.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rltd_pties.cdtr_acct.other.id.is_empty() {
                    record_write.push_str("/CACT/");
                    record_write.push_str(tx_dtls.rltd_pties.cdtr_acct.other.id.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rltd_agts.cdtr_agt.fin_instn_id.bic.is_empty() {
                    record_write.push_str("/CBIC/");
                    record_write.push_str(tx_dtls.rltd_agts.cdtr_agt.fin_instn_id.bic.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rmt_inf.ustrd.is_empty() {
                    record_write.push_str("/REMI/");
                    let mut ustrd_all = String::new();
                    for ustrd in tx_dtls.rmt_inf.ustrd.clone(){
                        ustrd_all.push_str(&ustrd);
                        ustrd_all.push_str("/");
                    }
                    record_write.push_str(ustrd_all.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.addtl_tx_inf.is_empty() {
                    record_write.push_str("/OPRP/");
                    record_write.push_str(tx_dtls.addtl_tx_inf.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rltd_pties.dbtr_acct.other.id.is_empty() {
                    record_write.push_str("/DACT/");
                    record_write.push_str(tx_dtls.rltd_pties.dbtr_acct.other.id.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.amt_dtls.amt.is_empty() {
                    record_write.push_str("/OAMT/");
                    record_write.push_str(tx_dtls.amt_dtls.amt.as_ref());
                    record_write.push_str("\n");
                }
                if !tx_dtls.rltd_pties.dbtr.id.othr.id.is_empty() {
                    record_write.push_str("/DCID/");
                    record_write.push_str(tx_dtls.rltd_pties.dbtr.id.othr.id.as_ref());
                    record_write.push_str("\n");
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::camt053::*;
    use super::*;

    #[test]
    fn test_find_record(){
        let doc ="{1:}{5:-}{1:   }{2:}{3:}{4:}{5:-}".to_string();
        let result: Vec<(usize, usize)> = vec![(0, 4), (9, 28)];
        assert_eq!(DocumentMt940::find_record(&doc).unwrap(), result);
    }
    #[test]
    fn test_parse_field_one() {
        let doc = "F01ASNBNL21XXXX0000000000".to_string();
        assert_eq!(DocumentMt940::parse_field_one(&doc), "ASNBNL21XXXX".to_string());
    }
    #[test]
    fn test_parse_field_two() {
        let doc = "{2:O940ASNBNL21XXXXN}".to_string();
        let mut result = BkToCstmrStmt::default();
        DocumentMt940::parse_field_two(&doc, &mut result);
        assert_eq!(BkToCstmrStmt {
            grp_hdr: HeaderAttribute {
                msg_id: "ASNBNL21XXXXN".to_string(),
                cre_dt_tm: "".to_string(),
            },
            stmt: StatementAttribute {
                id: "ASNBNL21XXXXN-940".to_string(),
                elctrnc_seq_nb: "".to_string(),
                lgl_seq_nb: "".to_string(),
                cre_dt_tm: "".to_string(),
                fr_to_dt: Default::default(),
                acct: Default::default(),
                bal: vec![],
                txs_summry: Default::default(),
                ntry: vec![],
            },
        }, result);
    }
    #[test]
    fn test_parse_field_86() {
        let doc = ":86:/NREF/NIOBNL56ASNB9999999999\n".to_string();
        let mut ntry_det_result: NtryDtlsAttribute = NtryDtlsAttribute::default();
        let mut ntry_det_test: NtryDtlsAttribute = NtryDtlsAttribute::default();
        let mut ntry_det_tlds: TxDtlsAttribute = TxDtlsAttribute::default();
        ntry_det_tlds.refs.prtry.tp = "NREF".to_string();
        ntry_det_tlds.refs.prtry.refdt = "NIOBNL56ASNB9999999999".to_string();
        ntry_det_test.tx_dtls.push(ntry_det_tlds);
        DocumentMt940::parse_field_86(&doc, "".to_string(), &mut ntry_det_result);
        assert_eq!(ntry_det_test, ntry_det_result);
    }
    #[test]
    fn test_parse_field_61() {
        let field_61 = ":61:2001050105C1000,00NIOBNL56ASNB9999999999\n".to_string();
        let mut ntry_result = NtryAttribute::default();
        let mut ntry_test = NtryAttribute::default();
        ntry_test.amt.ccy = "EUR".to_string();
        ntry_test.val_dt.dt = "2020-01-05".to_string();
        ntry_test.bookg_dt.dt = "2020-01-05".to_string();
        ntry_test.bk_tx_cd.prtry.cd = "NIOB".to_string();
        ntry_test.amt.amt = "1000.00".to_string();
        ntry_test.cdt_dbt_ind = "CRDT".to_string();
        let mut nxdet: NtryDtlsAttribute = NtryDtlsAttribute::default();
        let field_86 = ":86:/NREF/NIOBNL56ASNB9999999999\n";
        DocumentMt940::parse_field_86(field_86, "NL56ASNB9999999999".to_string(), &mut nxdet);
        ntry_test.ntry_dtls = nxdet;
        DocumentMt940::parse_field_61((&field_61, &field_86), "EUR", &mut ntry_result);
        assert_eq!(ntry_test, ntry_result);
    }
    #[test]
    fn test_parse_field_ntry(){
        let doc = "{1:F01GSCRUS30XXXX3614000002}{2:I940GSCRUS30XXXXN}{4:
                           :20:15486025400
                           :25:107048825
                           :28C:49/2
                           :60M:C250218USD2732398848,02
                           :61:2502180218D12,01NTRFGSLNVSHSUTKWDR//GI2504900007841
                           :86:/EREF/GSLNVSHSUTKWDR
                                /CRNM/GOLDMAN SACHS BANK USA
                                /CACT/107045863/CBIC/GSCRUS30XXX
                                /REMI/USD Payment to Vendor
                                /OPRP/Tag Payment
                           :61:2502180218D12,01NTRFGSOXWBAQYTF4VH//GI2504900005623
                           :86:/EREF/GSOXWBAQYTF4VH
                                /CRNM/GOLDMAN SACHS BANK USA
                                /CACT/107045863/CBIC/GSCRUS30XXX
                                /REMI/The maximum length of the block is 65 characters
                                /OPRP/Tag Payment}{5:-}".to_string();
        let result = DocumentMt940::parse_field_ntry(&doc, "USD").unwrap();
        let ntry_test: Vec<NtryAttribute> = vec![NtryAttribute { ntry_ref: 0,
            amt: AmtAttribute { ccy: "USD".to_string(), amt: "12.01".to_string() },
            cdt_dbt_ind: "DBIT".to_string(),
            sts: "".to_string(), bookg_dt: DtAttribute { dt: "2025-02-18".to_string() },
            val_dt: DtAttribute { dt: "2025-02-18".to_string() }, acct_svcr_ref: "".to_string(),
            bk_tx_cd: BxTxCdAttribute { domn: DomnAttribute { cd: "".to_string(),
                fmly: FmlyAttribute { cd: "".to_string(), sub_fmly_cd: "".to_string() } },
                prtry: PrtryAttribute { cd: "NTRF".to_string(), issr: "".to_string() } },
            addtl_inf_ind: AddtlTxInfAtttribute { msg_nm_id: "".to_string() },
            ntry_dtls: NtryDtlsAttribute { btch: BtchAttribute {
                nb_of_txs: 0 }, tx_dtls: vec![TxDtlsAttribute {
                refs: EndToEndIdAttribute { pmt_inf_id: "".to_string(),
                    instr_id: "".to_string(), end_to_end_id: "GSLNVSHSUTKWDR".to_string(),
                    tx_id: "".to_string(), prtry: PrtryDetAttribute {
                        tp: "".to_string(), refdt: "".to_string() } }, amt_dtls: TxAmtAttribute {
                    end_to_end_id: "".to_string(), instd_amt: PrtryAmtAttribute {
                        tp: "".to_string(), amt: AmtAttribute { ccy: "".to_string(), amt: "".to_string() },
                        ccy_xchg: CcyXchgAttribute { src_ccy: "".to_string(),
                            trgt_ccy: "".to_string(), unit_ccy: "".to_string(), xchg_rate: "".to_string() } },
                    tx_amt: PrtryAmtAttribute { tp: "".to_string(), amt: AmtAttribute {
                        ccy: "".to_string(), amt: "".to_string() }, ccy_xchg: CcyXchgAttribute {
                        src_ccy: "".to_string(), trgt_ccy: "".to_string(), unit_ccy: "".to_string(),
                        xchg_rate: "".to_string() } }, prtry_amt: PrtryAmtAttribute {
                        tp: "".to_string(), amt: AmtAttribute { ccy: "".to_string(), amt: "".to_string() },
                        ccy_xchg: CcyXchgAttribute { src_ccy: "".to_string(), trgt_ccy: "".to_string(),
                            unit_ccy: "".to_string(), xchg_rate: "".to_string() } }, amt: "".to_string() },
                bk_tx_cd: BxTxCdAttribute { domn: DomnAttribute { cd: "".to_string(),
                    fmly: FmlyAttribute { cd: "".to_string(), sub_fmly_cd: "".to_string() } },
                    prtry: PrtryAttribute { cd: "".to_string(), issr: "".to_string() } },
                rltd_pties: RltdPtiesAttribute { dbtr: DbtrAttribute {
                    id: PrvtIdAttribute { othr: IdDtldAttribute { id: "".to_string() } },
                    nm: "".to_string(), pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                        bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(), ctry: "".to_string(),
                        adr_line: Vec::new() } }, dbtr_acct: IdTxDtlsAttribute {
                    id: IdIbanAttribute { iban: "".to_string(), othr: OtherAttribute {
                        id: "".to_string(), schme_nm: ShemeNumberAttribute { cd: "".to_string() } } },
                    other: IdDtldAttribute { id: "".to_string() } }, cdtr: CdtrAttribue {
                    id: PrvtIdAttribute { othr: IdDtldAttribute { id: "".to_string() } },
                    nm: "GOLDMAN".to_string(), pstl_adr: PostalAddressAttribute {
                        strt_nm: "".to_string(), bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(),
                        ctry: "".to_string(), adr_line: Vec::new() } }, cdtr_acct: IdTxDtlsAttribute {
                    id: IdIbanAttribute { iban: "".to_string(), othr: OtherAttribute {
                        id: "".to_string(), schme_nm: ShemeNumberAttribute { cd: "".to_string() } } },
                    other: IdDtldAttribute { id: "107045863".to_string() } } },
                rltd_agts: CdtrAgtAttribute { cdtr_agt: SvcrAttribute {
                    fin_instn_id: FinInstIdAttribute { bic: "GSCRUS30XXX".to_string(),
                        nm: "".to_string(), pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                            bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(), ctry: "".to_string(),
                            adr_line: Vec::new() } } }, dbtr_agt: SvcrAttribute {
                    fin_instn_id: FinInstIdAttribute { bic: "".to_string(), nm: "".to_string(),
                        pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                            bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(), ctry: "".to_string(),
                            adr_line: Vec::new() } } } }, rmt_inf: RmtInfAttribute {
                    ustrd: vec!["USD".to_string()], strd: StrdAttribute { cdtr_ref_inf: CdtrRefInfAttribute {
                        tp: CdOrPrtryAttribute { cd_or_prtry: CdAttribute { cd: "".to_string() } },
                        ref_cdtr: "".to_string() } } }, rltd_dts: RltdDtsAttribute { accptnc_dt_tm: "".to_string() },
                addtl_tx_inf: "Tag".to_string() }] } }, NtryAttribute { ntry_ref: 0,
            amt: AmtAttribute { ccy: "USD".to_string(), amt: "12.01".to_string() }, cdt_dbt_ind: "DBIT".to_string(),
            sts: "".to_string(), bookg_dt: DtAttribute { dt: "2025-02-18".to_string() },
            val_dt: DtAttribute { dt: "2025-02-18".to_string() }, acct_svcr_ref: "".to_string(),
            bk_tx_cd: BxTxCdAttribute { domn: DomnAttribute { cd: "".to_string(),
                fmly: FmlyAttribute { cd: "".to_string(), sub_fmly_cd: "".to_string() } },
                prtry: PrtryAttribute { cd: "NTRF".to_string(), issr: "".to_string() } },
            addtl_inf_ind: AddtlTxInfAtttribute { msg_nm_id: "".to_string() },
            ntry_dtls: NtryDtlsAttribute { btch: BtchAttribute { nb_of_txs: 0 },
                tx_dtls: vec![TxDtlsAttribute { refs: EndToEndIdAttribute {
                    pmt_inf_id: "".to_string(), instr_id: "".to_string(), end_to_end_id: "GSOXWBAQYTF4VH".to_string(),
                    tx_id: "".to_string(), prtry: PrtryDetAttribute { tp: "".to_string(), refdt: "".to_string() } },
                    amt_dtls: TxAmtAttribute { end_to_end_id: "".to_string(),
                        instd_amt: PrtryAmtAttribute {
                        tp: "".to_string(), amt: AmtAttribute { ccy: "".to_string(), amt: "".to_string() },
                        ccy_xchg: CcyXchgAttribute { src_ccy: "".to_string(), trgt_ccy: "".to_string(),
                            unit_ccy: "".to_string(), xchg_rate: "".to_string() } }, tx_amt: PrtryAmtAttribute {
                        tp: "".to_string(), amt: AmtAttribute { ccy: "".to_string(), amt: "".to_string() },
                        ccy_xchg: CcyXchgAttribute { src_ccy: "".to_string(), trgt_ccy: "".to_string(),
                            unit_ccy: "".to_string(), xchg_rate: "".to_string() } }, prtry_amt: PrtryAmtAttribute {
                        tp: "".to_string(), amt: AmtAttribute { ccy: "".to_string(), amt: "".to_string() },
                        ccy_xchg: CcyXchgAttribute { src_ccy: "".to_string(), trgt_ccy: "".to_string(),
                            unit_ccy: "".to_string(), xchg_rate: "".to_string() } }, amt: "".to_string() },
                    bk_tx_cd: BxTxCdAttribute { domn: DomnAttribute { cd: "".to_string(),
                        fmly: FmlyAttribute { cd: "".to_string(), sub_fmly_cd: "".to_string() } }, prtry:
                    PrtryAttribute { cd: "".to_string(), issr: "".to_string() } }, rltd_pties: RltdPtiesAttribute {
                        dbtr: DbtrAttribute { id: PrvtIdAttribute { othr: IdDtldAttribute {
                            id: "".to_string() } }, nm: "".to_string(), pstl_adr: PostalAddressAttribute {
                            strt_nm: "".to_string(), bldg_nb: "".to_string(), pst_cd: "".to_string(),
                            twn_nm: "".to_string(), ctry: "".to_string(), adr_line: Vec::new() } }, dbtr_acct: IdTxDtlsAttribute {
                            id: IdIbanAttribute { iban: "".to_string(), othr: OtherAttribute {
                                id: "".to_string(), schme_nm: ShemeNumberAttribute { cd: "".to_string() } } },
                            other: IdDtldAttribute { id: "".to_string() } }, cdtr: CdtrAttribue {
                            id: PrvtIdAttribute { othr: IdDtldAttribute { id: "".to_string() } },
                            nm: "GOLDMAN".to_string(), pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                                bldg_nb: "".to_string(), pst_cd: "".to_string(),
                                twn_nm: "".to_string(), ctry: "".to_string(), adr_line: Vec::new() } },
                        cdtr_acct: IdTxDtlsAttribute { id: IdIbanAttribute { iban: "".to_string(),
                            othr: OtherAttribute { id: "".to_string(), schme_nm: ShemeNumberAttribute {
                                cd: "".to_string() } } }, other: IdDtldAttribute { id: "107045863".to_string() } } },
                    rltd_agts: CdtrAgtAttribute { cdtr_agt: SvcrAttribute {
                        fin_instn_id: FinInstIdAttribute { bic: "GSCRUS30XXX".to_string(),
                            nm: "".to_string(), pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                                bldg_nb: "".to_string(), pst_cd: "".to_string(),
                                twn_nm: "".to_string(), ctry: "".to_string(),
                                adr_line: Vec::new() } } }, dbtr_agt: SvcrAttribute {
                        fin_instn_id: FinInstIdAttribute { bic: "".to_string(), nm: "".to_string(),
                            pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                                bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(), ctry: "".to_string(),
                                adr_line: Vec::new() } } } }, rmt_inf: RmtInfAttribute {
                        ustrd: vec!["The".to_string()], strd: StrdAttribute { cdtr_ref_inf: CdtrRefInfAttribute {
                            tp: CdOrPrtryAttribute { cd_or_prtry: CdAttribute { cd: "".to_string() } },
                            ref_cdtr: "".to_string() } } }, rltd_dts: RltdDtsAttribute { accptnc_dt_tm: "".to_string() },
                    addtl_tx_inf: "Tag".to_string() }] } }];
        assert_eq!(ntry_test, result);
    }
    #[test]
    fn test_parse_field_foo(){
        let mut result = BkToCstmrStmt::default();
        let document = ":20:15486025400
                               :25:107048825
                               :28C:49/2
                               :60M:C250218USD2732398848,02".to_string();
        DocumentMt940::parse_field_foo(&document, &mut result);
        let test = BkToCstmrStmt { grp_hdr: HeaderAttribute {
            msg_id: "".to_string(), cre_dt_tm: "".to_string() }, stmt: StatementAttribute {
            id: "".to_string(), elctrnc_seq_nb: "49".to_string(), lgl_seq_nb: "2".to_string(),
            cre_dt_tm: "".to_string(), fr_to_dt: FromToDtAttribute {
                fr_dt_tm: "".to_string(), to_dt_tm: "".to_string() },
            acct: AcctAttribute { id: IdIbanAttribute {
                iban: "".to_string(), othr: OtherAttribute {
                    id: "".to_string(), schme_nm: ShemeNumberAttribute {
                        cd: "".to_string() } } }, ccy: "".to_string(),
                nm: "".to_string(), ownr: OwnerAttribute { nm: "".to_string(),
                    pstl_adr: PostalAddressAttribute {
                        strt_nm: "".to_string(), bldg_nb: "".to_string(),
                        pst_cd: "".to_string(), twn_nm: "".to_string(),
                        ctry: "".to_string(), adr_line: Vec::new() },
                    bldg_nb: 0, pst_cd: 0,
                    twn_nm: "".to_string(), ctry: "".to_string(),
                    id: IdAttribute {
                        org_id: OrgIdAttribute {
                            othr: OtherAttribute { id: "107048825".to_string(),
                                schme_nm: ShemeNumberAttribute {
                                    cd: "".to_string() } } } } },
                svcr: SvcrAttribute { fin_instn_id: FinInstIdAttribute {
                    bic: "".to_string(), nm: "".to_string(), pstl_adr: PostalAddressAttribute {
                        strt_nm: "".to_string(), bldg_nb: "".to_string(), pst_cd: "".to_string(),
                        twn_nm: "".to_string(), ctry: "".to_string(), adr_line: Vec::new() },  } } },
            bal: vec![BalanceAttribute { tp: TpBalanceAttribute {
                cd_or_prtry: CdAttribute { cd: "OPAV".to_string() } },
                amt: AmtAttribute { ccy: "USD".to_string(), amt: "2732398848.02".to_string() },
                cdt_dbt_ind: "".to_string(), dt: DtAttribute { dt: "2025-02-18".to_string() }, cd: "C".to_string() }],
            txs_summry: TxsSummryAttribute {
                ttl_ntries: TtlNtriesAttribute {
                    nb_of_ntries: "".to_string(), ttl_net_ntry_amt: 0.0,
                    cdt_dbt_ind: "" .to_string()}, ttl_cdt_ntries: TtlCdtDbtNtriesAttribute {
                    nb_of_ntries: 0, sum: "".to_string() },
                ttl_dbt_ntries: TtlCdtDbtNtriesAttribute {
                    nb_of_ntries: 0, sum: "".to_string() } }, ntry: Vec::new() } };

           assert_eq!(test, result);
    }
    #[test]
    fn test_parse_one_record(){
        let doc = "{1:F01GSCRUS30XXXX3614000002}{2:I940GSCRUS30XXXXN}{4:
                           :20:15486025400
                           :25:107048825
                           :28C:49/2
                           :60M:C250218USD2732398848,02
                           :61:2502180218D12,01NTRFGSLNVSHSUTKWDR//GI2504900007841
                           :86:/EREF/GSLNVSHSUTKWDR
                                /CRNM/GOLDMAN SACHS BANK USA
                                /CACT/107045863/CBIC/GSCRUS30XXX
                                /REMI/USD Payment to Vendor
                                /OPRP/Tag Payment
                           :61:2502180218D12,01NTRFGSOXWBAQYTF4VH//GI2504900005623
                           :86:/EREF/GSOXWBAQYTF4VH
                                /CRNM/GOLDMAN SACHS BANK USA
                                /CACT/107045863/CBIC/GSCRUS30XXX
                                /REMI/The maximum length of the block is 65 characters
                                /OPRP/Tag Payment}{5:-}".to_string();
        let result = DocumentMt940::parse_one_record(&doc).unwrap();
        let test = BkToCstmrStmt { grp_hdr: HeaderAttribute {
            msg_id: "GSCRUS30XXXXN".to_string(), cre_dt_tm: "".to_string() },
            stmt: StatementAttribute { id: "GSCRUS30XXXXN-940".to_string(),
                elctrnc_seq_nb: "49".to_string(), lgl_seq_nb: "2".to_string(), cre_dt_tm: "".to_string(),
                fr_to_dt: FromToDtAttribute { fr_dt_tm: "".to_string(), to_dt_tm: "".to_string() },
                acct: AcctAttribute { id: IdIbanAttribute { iban: "".to_string(),
                    othr: OtherAttribute { id: "".to_string(), schme_nm: ShemeNumberAttribute {
                        cd: "".to_string() } } }, ccy: "".to_string(), nm: "".to_string(),
                    ownr: OwnerAttribute { nm: "".to_string(),
                        pstl_adr: PostalAddressAttribute {
                            strt_nm: "".to_string(), bldg_nb: "".to_string(),
                            pst_cd: "".to_string(), twn_nm: "".to_string(), ctry: "".to_string(),
                            adr_line: Vec::new() }, bldg_nb: 0,
                        pst_cd: 0, twn_nm: "".to_string(), ctry: "".to_string(),
                        id: IdAttribute { org_id: OrgIdAttribute {
                            othr: OtherAttribute { id: "107048825".to_string(),
                                schme_nm: ShemeNumberAttribute {
                                    cd: "".to_string() } } } } },
                    svcr: SvcrAttribute { fin_instn_id: FinInstIdAttribute {
                        bic: "GSCRUS30XXXX".to_string(), nm: "".to_string(),
                        pstl_adr: PostalAddressAttribute {
                            strt_nm: "".to_string(), bldg_nb: "".to_string(),
                            pst_cd: "".to_string(), twn_nm: "".to_string(),
                            ctry: "".to_string(), adr_line: Vec::new() } } } },
                bal: vec![BalanceAttribute { tp: TpBalanceAttribute {
                    cd_or_prtry: CdAttribute { cd: "OPAV".to_string() } },
                    amt: AmtAttribute { ccy: "USD".to_string(), amt: "2732398848.02".to_string() },
                    cdt_dbt_ind: "".to_string(), dt: DtAttribute { dt: "2025-02-18".to_string() },
                    cd: "C".to_string() }], txs_summry: TxsSummryAttribute {
                    ttl_ntries: TtlNtriesAttribute { nb_of_ntries: "".to_string(),
                        ttl_net_ntry_amt: 0.0, cdt_dbt_ind: "".to_string() },
                    ttl_cdt_ntries: TtlCdtDbtNtriesAttribute {
                        nb_of_ntries: 0, sum: "".to_string() },
                    ttl_dbt_ntries: TtlCdtDbtNtriesAttribute {
                        nb_of_ntries: 0, sum: "".to_string() } },
                ntry: vec![NtryAttribute { ntry_ref: 0,
                    amt: AmtAttribute { ccy: "USD".to_string(), amt: "12.01".to_string() },
                    cdt_dbt_ind: "DBIT".to_string(), sts: "".to_string(), bookg_dt: DtAttribute {
                        dt: "2025-02-18".to_string() }, val_dt: DtAttribute { dt: "2025-02-18".to_string() },
                    acct_svcr_ref: "".to_string(), bk_tx_cd: BxTxCdAttribute {
                        domn: DomnAttribute { cd: "".to_string(), fmly: FmlyAttribute {
                            cd: "".to_string(), sub_fmly_cd: "".to_string() } },
                        prtry: PrtryAttribute { cd: "NTRF".to_string(), issr: "".to_string() } },
                    addtl_inf_ind: AddtlTxInfAtttribute { msg_nm_id: "".to_string() },
                    ntry_dtls: NtryDtlsAttribute { btch: BtchAttribute {
                        nb_of_txs: 0 }, tx_dtls: vec![TxDtlsAttribute {
                        refs: EndToEndIdAttribute { pmt_inf_id: "".to_string(),
                            instr_id: "".to_string(), end_to_end_id: "GSLNVSHSUTKWDR".to_string(),
                            tx_id: "".to_string(), prtry: PrtryDetAttribute { tp: "".to_string(),
                                refdt: "".to_string() } }, amt_dtls: TxAmtAttribute {
                            end_to_end_id: "".to_string(), instd_amt: PrtryAmtAttribute {
                                tp: "".to_string(), amt: AmtAttribute { ccy: "".to_string(), amt: "".to_string() },
                                ccy_xchg: CcyXchgAttribute { src_ccy: "".to_string(),
                                    trgt_ccy: "".to_string(), unit_ccy: "".to_string(), xchg_rate: "".to_string() } },
                            tx_amt: PrtryAmtAttribute { tp: "".to_string(), amt: AmtAttribute {
                                ccy: "".to_string(), amt: "".to_string() }, ccy_xchg: CcyXchgAttribute {
                                src_ccy: "".to_string(), trgt_ccy: "".to_string(), unit_ccy: "".to_string(),
                                xchg_rate: "".to_string() } }, prtry_amt: PrtryAmtAttribute {
                                tp: "".to_string(), amt: AmtAttribute { ccy: "".to_string(), amt: "".to_string() },
                                ccy_xchg: CcyXchgAttribute { src_ccy: "".to_string(),
                                    trgt_ccy: "".to_string(), unit_ccy: "".to_string(), xchg_rate: "".to_string() } },
                            amt: "".to_string() }, bk_tx_cd: BxTxCdAttribute { domn: DomnAttribute {
                            cd: "".to_string(), fmly: FmlyAttribute { cd: "".to_string(), sub_fmly_cd: "".to_string() } },
                            prtry: PrtryAttribute { cd: "".to_string(), issr: "".to_string() } },
                        rltd_pties: RltdPtiesAttribute { dbtr: DbtrAttribute {
                            id: PrvtIdAttribute { othr: IdDtldAttribute {
                                id: "".to_string() } }, nm: "".to_string(), pstl_adr: PostalAddressAttribute {
                                strt_nm: "".to_string(), bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(),
                                ctry: "".to_string(), adr_line: Vec::new() } }, dbtr_acct: IdTxDtlsAttribute {
                            id: IdIbanAttribute { iban: "".to_string(), othr: OtherAttribute { id: "".to_string(),
                                schme_nm: ShemeNumberAttribute { cd: "".to_string() } } },
                            other: IdDtldAttribute { id: "".to_string() } }, cdtr: CdtrAttribue {
                            id: PrvtIdAttribute { othr: IdDtldAttribute { id: "".to_string() } },
                            nm: "GOLDMAN".to_string(), pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                                bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(),
                                ctry: "".to_string(), adr_line: Vec::new()} },
                            cdtr_acct: IdTxDtlsAttribute { id: IdIbanAttribute { iban: "".to_string(),
                                othr: OtherAttribute { id: "".to_string(), schme_nm: ShemeNumberAttribute {
                                    cd: "".to_string() } } }, other: IdDtldAttribute { id: "107045863".to_string() } } },
                        rltd_agts: CdtrAgtAttribute { cdtr_agt: SvcrAttribute { fin_instn_id:
                        FinInstIdAttribute { bic: "GSCRUS30XXX".to_string(), nm: "".to_string(),
                            pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                                bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(), ctry: "".to_string(),
                                adr_line: Vec::new() } } }, dbtr_agt: SvcrAttribute {
                            fin_instn_id: FinInstIdAttribute { bic: "".to_string(), nm: "".to_string(),
                                pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                                    bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(),
                                    ctry: "".to_string(),
                                    adr_line: Vec::new() } } } }, rmt_inf: RmtInfAttribute {
                            ustrd: vec!["USD".to_string()], strd: StrdAttribute { cdtr_ref_inf:
                            CdtrRefInfAttribute { tp: CdOrPrtryAttribute { cd_or_prtry:
                            CdAttribute { cd: "".to_string() } }, ref_cdtr: "".to_string() } } },
                        rltd_dts: RltdDtsAttribute { accptnc_dt_tm: "".to_string() },
                        addtl_tx_inf: "Tag".to_string() }] } }, NtryAttribute { ntry_ref: 0,
                    amt: AmtAttribute { ccy: "USD".to_string(), amt: "12.01".to_string() }, cdt_dbt_ind:
                    "DBIT".to_string(), sts: "".to_string(), bookg_dt: DtAttribute { dt: "2025-02-18".to_string() },
                    val_dt: DtAttribute { dt: "2025-02-18".to_string() }, acct_svcr_ref: "".to_string(),
                    bk_tx_cd: BxTxCdAttribute { domn: DomnAttribute { cd: "".to_string(),
                        fmly: FmlyAttribute { cd: "".to_string(), sub_fmly_cd: "".to_string() } },
                        prtry: PrtryAttribute { cd: "NTRF".to_string(), issr: "".to_string() } },
                    addtl_inf_ind: AddtlTxInfAtttribute { msg_nm_id: "".to_string() },
                    ntry_dtls: NtryDtlsAttribute { btch: BtchAttribute {
                        nb_of_txs: 0 }, tx_dtls: vec![TxDtlsAttribute {
                        refs: EndToEndIdAttribute { pmt_inf_id: "".to_string(),
                            instr_id: "".to_string(), end_to_end_id: "GSOXWBAQYTF4VH".to_string(),
                            tx_id: "".to_string(), prtry: PrtryDetAttribute { tp: "".to_string(),
                                refdt: "".to_string() } }, amt_dtls: TxAmtAttribute {
                            end_to_end_id: "".to_string(), instd_amt: PrtryAmtAttribute {
                                tp: "".to_string(), amt: AmtAttribute { ccy: "".to_string(), amt: "".to_string() },
                                ccy_xchg: CcyXchgAttribute { src_ccy: "".to_string(),
                                    trgt_ccy: "".to_string(), unit_ccy: "".to_string(),
                                    xchg_rate: "".to_string() } },
                            tx_amt: PrtryAmtAttribute { tp: "".to_string(), amt: AmtAttribute {
                                ccy: "".to_string(), amt: "".to_string() }, ccy_xchg: CcyXchgAttribute {
                                src_ccy: "".to_string(), trgt_ccy: "".to_string(),
                                unit_ccy: "".to_string(), xchg_rate: "".to_string() } },
                            prtry_amt: PrtryAmtAttribute { tp: "".to_string(), amt: AmtAttribute {
                                ccy: "".to_string(), amt: "".to_string() }, ccy_xchg: CcyXchgAttribute {
                                src_ccy: "".to_string(), trgt_ccy: "".to_string(),
                                unit_ccy: "".to_string(), xchg_rate: "".to_string() } },
                            amt: "".to_string() }, bk_tx_cd: BxTxCdAttribute { domn: DomnAttribute {
                            cd: "".to_string(), fmly: FmlyAttribute { cd: "".to_string(), sub_fmly_cd: "".to_string() } },
                            prtry: PrtryAttribute { cd: "".to_string(), issr: "".to_string() } },
                        rltd_pties: RltdPtiesAttribute { dbtr: DbtrAttribute {
                            id: PrvtIdAttribute { othr: IdDtldAttribute { id: "".to_string() } },
                            nm: "".to_string(), pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                                bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(), ctry: "".to_string(),
                                adr_line: Vec::new()} }, dbtr_acct: IdTxDtlsAttribute {
                            id: IdIbanAttribute { iban: "".to_string(), othr: OtherAttribute {
                                id: "".to_string(), schme_nm: ShemeNumberAttribute { cd: "".to_string() } } },
                            other: IdDtldAttribute { id: "".to_string() } }, cdtr: CdtrAttribue {
                            id: PrvtIdAttribute { othr: IdDtldAttribute { id: "".to_string() } },
                            nm: "GOLDMAN".to_string(), pstl_adr: PostalAddressAttribute {
                                strt_nm: "".to_string(), bldg_nb: "".to_string(),
                                pst_cd: "".to_string(), twn_nm: "".to_string(),
                                ctry: "".to_string(), adr_line: Vec::new() } }, cdtr_acct: IdTxDtlsAttribute {
                            id: IdIbanAttribute { iban: "".to_string(), othr: OtherAttribute { id: "".to_string(),
                                schme_nm: ShemeNumberAttribute { cd: "".to_string() } } },
                            other: IdDtldAttribute { id: "107045863".to_string() } } },
                        rltd_agts: CdtrAgtAttribute { cdtr_agt: SvcrAttribute {
                            fin_instn_id: FinInstIdAttribute { bic: "GSCRUS30XXX".to_string(),
                                nm: "".to_string(),
                                pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                                    bldg_nb: "".to_string(), pst_cd: "".to_string(),
                                    twn_nm: "".to_string(), ctry: "".to_string(),
                                    adr_line: Vec::new() } } }, dbtr_agt: SvcrAttribute {
                            fin_instn_id: FinInstIdAttribute { bic: "".to_string(), nm: "".to_string(),
                                pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                                    bldg_nb: "".to_string(), pst_cd: "".to_string(),
                                    twn_nm: "".to_string(), ctry: "".to_string(),
                                    adr_line: Vec::new() } } } }, rmt_inf: RmtInfAttribute {
                            ustrd: vec!["The".to_string()], strd: StrdAttribute { cdtr_ref_inf:
                            CdtrRefInfAttribute { tp: CdOrPrtryAttribute {
                                cd_or_prtry: CdAttribute { cd: "".to_string() } },
                                ref_cdtr: "".to_string() } } },
                        rltd_dts: RltdDtsAttribute { accptnc_dt_tm: "".to_string() },
                        addtl_tx_inf: "Tag".to_string() }] } }] } };
        assert_eq!(test, result);
    }
    #[test]
    fn test_extract_field_6x_mt940(){
        let document = BkToCstmrStmt { grp_hdr: HeaderAttribute {
            msg_id: "GSCRUS30XXXXN".to_string(), cre_dt_tm: "".to_string() },
            stmt: StatementAttribute { id: "GSCRUS30XXXXN-940".to_string(),
                elctrnc_seq_nb: "49".to_string(), lgl_seq_nb: "2".to_string(), cre_dt_tm: "".to_string(),
                fr_to_dt: FromToDtAttribute { fr_dt_tm: "".to_string(), to_dt_tm: "".to_string() },
                acct: AcctAttribute { id: IdIbanAttribute { iban: "".to_string(),
                    othr: OtherAttribute { id: "".to_string(),
                        schme_nm: ShemeNumberAttribute { cd: "".to_string() } } },
                    ccy: "".to_string(), nm: "".to_string(), ownr: OwnerAttribute {
                        nm: "".to_string(), pstl_adr: PostalAddressAttribute {
                            strt_nm: "".to_string(),
                            bldg_nb: "".to_string(),
                            pst_cd: "".to_string(),
                            twn_nm: "".to_string(),
                            ctry: "".to_string(),
                            adr_line: vec![],
                        }, bldg_nb: 0,
                        pst_cd: 0, twn_nm: "".to_string(), ctry: "".to_string(),
                        id: IdAttribute { org_id: OrgIdAttribute {
                            othr: OtherAttribute { id: "107048825".to_string(),
                                schme_nm: ShemeNumberAttribute {
                                    cd: "".to_string() } } } } },
                    svcr: SvcrAttribute { fin_instn_id: FinInstIdAttribute {
                        bic: "GSCRUS30XXXX".to_string(), nm: "".to_string(),
                        pstl_adr: Default::default(),
                    } } },
                bal: vec![BalanceAttribute { tp: TpBalanceAttribute {
                    cd_or_prtry: CdAttribute { cd: "OPAV".to_string() } },
                    amt: AmtAttribute { ccy: "USD".to_string(), amt: "2732398848.02".to_string() },
                    cdt_dbt_ind: "".to_string(),
                    dt: DtAttribute { dt: "2025-02-18".to_string() }, cd: "C".to_string() }],
                txs_summry: TxsSummryAttribute { ttl_ntries: TtlNtriesAttribute {
                    nb_of_ntries: "".to_string(), ttl_net_ntry_amt: 0.0,
                    cdt_dbt_ind: "".to_string() }, ttl_cdt_ntries: TtlCdtDbtNtriesAttribute {
                    nb_of_ntries: 0, sum: "".to_string() },
                    ttl_dbt_ntries: TtlCdtDbtNtriesAttribute { nb_of_ntries: 0,
                        sum: "".to_string() } }, ntry: Vec::new()} };
        let mut result = String::new();
        DocumentMt940::extract_field_6x_mt940(&document, &mut result);
        assert_eq!(":60M:C250218USD2732398848,02\n".to_string(), result);
    }
    #[test]
    fn test_extract_field_61_86_mt940(){
        let vec_camt = vec![NtryAttribute { ntry_ref: 0,
                    amt: AmtAttribute { ccy: "USD".to_string(), amt: "12.01".to_string() },
                    cdt_dbt_ind: "DBIT".to_string(), sts: "".to_string(), bookg_dt: DtAttribute {
                        dt: "2025-02-18".to_string() }, val_dt: DtAttribute { dt: "2025-02-18".to_string() },
                    acct_svcr_ref: "".to_string(), bk_tx_cd: BxTxCdAttribute {
                        domn: DomnAttribute { cd: "".to_string(), fmly: FmlyAttribute {
                            cd: "".to_string(), sub_fmly_cd: "".to_string() } },
                        prtry: PrtryAttribute { cd: "NTRF".to_string(), issr: "".to_string() } },
                    addtl_inf_ind: AddtlTxInfAtttribute { msg_nm_id: "".to_string() },
                    ntry_dtls: NtryDtlsAttribute { btch: BtchAttribute {
                        nb_of_txs: 0 }, tx_dtls: vec![TxDtlsAttribute {
                        refs: EndToEndIdAttribute { pmt_inf_id: "".to_string(),
                            instr_id: "".to_string(), end_to_end_id: "GSLNVSHSUTKWDR".to_string(),
                            tx_id: "".to_string(), prtry: PrtryDetAttribute { tp: "".to_string(),
                                refdt: "".to_string() } }, amt_dtls: TxAmtAttribute {
                            end_to_end_id: "".to_string(), instd_amt: PrtryAmtAttribute {
                                tp: "".to_string(), amt: AmtAttribute { ccy: "".to_string(), amt: "".to_string() },
                                ccy_xchg: CcyXchgAttribute { src_ccy: "".to_string(),
                                    trgt_ccy: "".to_string(), unit_ccy: "".to_string(), xchg_rate: "".to_string() } },
                            tx_amt: PrtryAmtAttribute { tp: "".to_string(), amt: AmtAttribute {
                                ccy: "".to_string(), amt: "".to_string() }, ccy_xchg: CcyXchgAttribute {
                                src_ccy: "".to_string(), trgt_ccy: "".to_string(), unit_ccy: "".to_string(),
                                xchg_rate: "".to_string() } }, prtry_amt: PrtryAmtAttribute {
                                tp: "".to_string(), amt: AmtAttribute { ccy: "".to_string(), amt: "".to_string() },
                                ccy_xchg: CcyXchgAttribute { src_ccy: "".to_string(),
                                    trgt_ccy: "".to_string(), unit_ccy: "".to_string(), xchg_rate: "".to_string() } },
                            amt: "".to_string() }, bk_tx_cd: BxTxCdAttribute { domn: DomnAttribute {
                            cd: "".to_string(), fmly: FmlyAttribute { cd: "".to_string(), sub_fmly_cd: "".to_string() } },
                            prtry: PrtryAttribute { cd: "".to_string(), issr: "".to_string() } },
                        rltd_pties: RltdPtiesAttribute { dbtr: DbtrAttribute {
                            id: PrvtIdAttribute { othr: IdDtldAttribute {
                                id: "".to_string() } }, nm: "".to_string(), pstl_adr: PostalAddressAttribute {
                                strt_nm: "".to_string(), bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(),
                                ctry: "".to_string(), adr_line: Vec::new() } }, dbtr_acct: IdTxDtlsAttribute {
                            id: IdIbanAttribute { iban: "".to_string(), othr: OtherAttribute { id: "".to_string(),
                                schme_nm: ShemeNumberAttribute { cd: "".to_string() } } },
                            other: IdDtldAttribute { id: "".to_string() } }, cdtr: CdtrAttribue {
                            id: PrvtIdAttribute { othr: IdDtldAttribute { id: "".to_string() } },
                            nm: "GOLDMAN".to_string(), pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                                bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(),
                                ctry: "".to_string(), adr_line: Vec::new()} },
                            cdtr_acct: IdTxDtlsAttribute { id: IdIbanAttribute { iban: "".to_string(),
                                othr: OtherAttribute { id: "".to_string(), schme_nm: ShemeNumberAttribute {
                                    cd: "".to_string() } } }, other: IdDtldAttribute { id: "107045863".to_string() } } },
                        rltd_agts: CdtrAgtAttribute { cdtr_agt: SvcrAttribute { fin_instn_id:
                        FinInstIdAttribute { bic: "GSCRUS30XXX".to_string(), nm: "".to_string(),
                            pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                                bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(), ctry: "".to_string(),
                                adr_line: Vec::new() } } }, dbtr_agt: SvcrAttribute {
                            fin_instn_id: FinInstIdAttribute { bic: "".to_string(), nm: "".to_string(),
                                pstl_adr: PostalAddressAttribute { strt_nm: "".to_string(),
                                    bldg_nb: "".to_string(), pst_cd: "".to_string(), twn_nm: "".to_string(),
                                    ctry: "".to_string(),
                                    adr_line: Vec::new() } } } }, rmt_inf: RmtInfAttribute {
                            ustrd: vec!["USD".to_string()], strd: StrdAttribute { cdtr_ref_inf:
                            CdtrRefInfAttribute { tp: CdOrPrtryAttribute { cd_or_prtry:
                            CdAttribute { cd: "".to_string() } }, ref_cdtr: "".to_string() } } },
                        rltd_dts: RltdDtsAttribute { accptnc_dt_tm: "".to_string() },
                        addtl_tx_inf: "Tag".to_string() }] } }];
        let mut result = String::new();
        let test = ":61:2502180218D12,01NTRFGSLNVSHSUTKWDR \n:86:/NREF/GSLNVSHSUTKWDR\n\
        /CRNM/GOLDMAN\n/CACT/107045863\n/CBIC/GSCRUS30XXX\n/REMI/USD/\n/OPRP/Tag\n";
        DocumentMt940::extract_field_61_86_mt940(&vec_camt, &mut result);
        assert_eq!(test, result);
    }
}
