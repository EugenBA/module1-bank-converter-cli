use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, PartialEq)]
#[serde(rename="Document", default)]
pub struct DocumentCamt053 {
    #[serde(rename="BkToCstmrStmt")]
    pub(crate) bk_to_cstmr_stmt: Vec<BkToCstmrStmt>
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct BkToCstmrStmt {
    pub(crate) grp_hdr: HeaderAttribute,
    pub(crate) stmt: StatementAttribute,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct HeaderAttribute{
    pub(crate) msg_id: String, //message id
    pub(crate) cre_dt_tm: String, //datetime create file
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate)  struct StatementAttribute{
    pub(crate) id: String, //id
    pub(crate) elctrnc_seq_nb: String, //ElctrncSeqNb
    pub(crate) lgl_seq_nb: String, //LglSeqNb
    pub(crate) cre_dt_tm: String, //CreDtTm
    pub(crate) fr_to_dt: FromToDtAttribute, //FrToDt
    pub(crate) acct: AcctAttribute, //Acct
    pub(crate) bal: Vec<BalanceAttribute>, //Bal
    pub(crate) txs_summry: TxsSummryAttribute, //TxsSummry
    pub(crate) ntry: Vec<NtryAttribute> //Ntry

}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub (crate) struct NtryAttribute{
    pub(crate) ntry_ref: u32, //NtryRef
    pub(crate) amt: AmtAttribute, //Amt
    pub(crate) cdt_dbt_ind: String, //CdtDbtInd
    pub(crate) sts: String, //Sts
    pub(crate) bookg_dt: DtAttribute, //BookgDt
    pub(crate) val_dt: DtAttribute, //ValDt
    pub(crate) acct_svcr_ref: String, //AcctSvcrRef
    pub(crate) bk_tx_cd: BxTxCdAttribute, //BkTxCd
    pub(crate) addtl_inf_ind: AddtlTxInfAtttribute, //AddtlTxInf
    pub(crate) ntry_dtls: NtryDtlsAttribute//NtryDtls

}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct NtryDtlsAttribute{
    pub(crate) btch: BtchAttribute, //Btch
    pub(crate) tx_dtls: Vec<TxDtlsAttribute>//TxDtls
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct BtchAttribute{
    pub(crate) nb_of_txs: u32, //NbOfTxs
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct TxDtlsAttribute {
    pub(crate) refs: EndToEndIdAttribute, //Refs
    pub(crate) amt_dtls: TxAmtAttribute, //AmtDtls
    pub(crate) bk_tx_cd: BxTxCdAttribute, //BxTxCd
    pub(crate) rltd_pties: RltdPtiesAttribute, //RltdPties
    pub(crate) rltd_agts: CdtrAgtAttribute, //RltdAgts
    pub(crate) rmt_inf: RmtInfAttribute, //RmtInf
    pub(crate) rltd_dts: RltdDtsAttribute, //RltdDts
    pub(crate) addtl_tx_inf: String

}

#[derive(Debug, Deserialize, Default, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct AddtlTxInfAtttribute{
    pub(crate) msg_nm_id: String,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct CdtrAgtAttribute{
    pub(crate) cdtr_agt: SvcrAttribute,
    pub(crate) dbtr_agt: SvcrAttribute,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct EndToEndIdAttribute{
    pub(crate) pmt_inf_id: String, //PmtInfId
    pub(crate) instr_id: String,//InstrId
    pub(crate) end_to_end_id: String,
    pub(crate) tx_id: String,
    pub(crate) prtry: PrtryDetAttribute
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct PrtryDetAttribute{
    pub(crate) tp: String,
    #[serde(rename="Ref")]
    pub(crate) refdt: String
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct TxAmtAttribute{
    pub(crate) end_to_end_id: String,
    pub(crate) instd_amt: PrtryAmtAttribute,
    pub(crate) tx_amt: PrtryAmtAttribute, //TxAmt
    pub(crate) prtry_amt: PrtryAmtAttribute,//PrtryAmt
    pub(crate) amt: String,
}



#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct PrtryAmtAttribute{
    pub(crate) tp: String,
    pub(crate) amt: AmtAttribute,
    pub(crate) ccy_xchg: CcyXchgAttribute
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct CcyXchgAttribute{
    pub(crate) src_ccy: String, //SrcCcy
    pub(crate) trgt_ccy: String, //TrgtCcy
    pub(crate) unit_ccy: String, //UnitCcy
    pub(crate) xchg_rate: String //XchgRate
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct RltdPtiesAttribute{
    pub(crate) dbtr: DbtrAttribute,
    pub(crate) dbtr_acct: IdTxDtlsAttribute,
    pub(crate) cdtr: CdtrAttribue,
    pub(crate) cdtr_acct: IdTxDtlsAttribute,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct DbtrAttribute{
    pub(crate) id: PrvtIdAttribute,
    pub(crate) nm: String,
    pub(crate) pstl_adr: PostalAddressAttribute
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct PrvtIdAttribute {
    pub(crate) othr: IdDtldAttribute
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct CdtrAttribue{
    pub(crate) id: PrvtIdAttribute,
    pub(crate) nm: String,
    pub(crate) pstl_adr: PostalAddressAttribute //PstlAdr
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct RmtInfAttribute{
    pub(crate) ustrd: Vec<String>,
    pub(crate) strd: StrdAttribute

}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct StrdAttribute {
   pub(crate) cdtr_ref_inf: CdtrRefInfAttribute //CdrtRefInf
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct RltdDtsAttribute{
    pub(crate) accptnc_dt_tm: String,
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(default)]
pub(crate) struct CdtrRefInfAttribute{
    #[serde(rename="Tp")]
    pub(crate) tp: CdOrPrtryAttribute,//Tp
    #[serde(rename="Ref")]
    pub(crate) ref_cdtr: String //Ref

}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct CdOrPrtryAttribute{
    pub(crate) cd_or_prtry: CdAttribute//CdOrPrtry
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct IdTxDtlsAttribute{
    pub(crate) id: IdIbanAttribute, //Id
    pub(crate) other: IdDtldAttribute//Other
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct IdDtldAttribute{
    pub(crate) id: String
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct AmtAttribute{
    #[serde(rename="@Ccy")]
    pub(crate) ccy: String,//Ccy
    #[serde(rename="#text")]
    pub(crate) amt: String
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct BxTxCdAttribute{
    pub(crate) domn: DomnAttribute, //Domn
    pub(crate) prtry: PrtryAttribute//Prtry
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct DomnAttribute{
    pub(crate) cd: String, //Cd
    pub(crate) fmly: FmlyAttribute, //Fmly
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct FmlyAttribute{
    pub(crate) cd: String, //Cd
    pub(crate) sub_fmly_cd: String//SubFmlyCd
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct PrtryAttribute{
    pub(crate) cd: String, //cd
    pub(crate) issr: String//Issr
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct TxsSummryAttribute{
    pub(crate) ttl_ntries: TtlNtriesAttribute, //TtlNtries
    pub(crate) ttl_cdt_ntries: TtlCdtDbtNtriesAttribute,//TtlCdtNtries
    pub(crate) ttl_dbt_ntries: TtlCdtDbtNtriesAttribute//TtlDbtNtries

}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct TtlNtriesAttribute{
    pub(crate) nb_of_ntries: String, //NbOfNtries
    pub(crate) ttl_net_ntry_amt: f64,//TtlNetNtryAmt
    pub(crate) cdt_dbt_ind: String//CdtDbtInd

}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct TtlCdtDbtNtriesAttribute{
    pub(crate) nb_of_ntries: u32, //NbOfNtries
    pub(crate) sum: String, //Sum
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct BalanceAttribute{
    pub(crate) tp: TpBalanceAttribute, // tp
    pub(crate) amt: AmtAttribute,
    pub(crate) cdt_dbt_ind: String, //CdtDbtInd
    pub(crate) dt:  DtAttribute,
    #[serde(skip_serializing)]
    pub(crate) cd: String
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct DtAttribute{
    pub(crate) dt: String,
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct TpBalanceAttribute{
    pub(crate) cd_or_prtry: CdAttribute
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct CdAttribute{
    pub(crate) cd: String,
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct FromToDtAttribute{
    pub(crate) fr_dt_tm: String, //FrDtTm
    pub(crate) to_dt_tm: String, //ToDtTm
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate)  struct AcctAttribute {
    pub(crate) id: IdIbanAttribute, //id
    pub(crate) ccy: String, //Ccy
    pub(crate) nm: String, //nm
    pub(crate) ownr: OwnerAttribute, //ownr
    pub(crate) svcr: SvcrAttribute, //svcr


}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(default)]
pub(crate) struct IdIbanAttribute{
    #[serde(rename="IBAN")]
    pub(crate) iban: String, //IBAN
    #[serde(rename="Othr")]
    pub(crate) othr: OtherAttribute,
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate)  struct SvcrAttribute{
    pub(crate) fin_instn_id: FinInstIdAttribute //FinInstnId
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(default)]
pub(crate)  struct FinInstIdAttribute{
    #[serde(rename = "BIC")]
    pub(crate) bic: String, //BIC
    #[serde(rename = "Nm")]
    pub(crate) nm: String,
    #[serde(rename = "PstlAdr")]
    pub(crate) pstl_adr: PostalAddressAttribute
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct OwnerAttribute{
    pub(crate) nm: String, //nm
    pub(crate) pstl_adr: PostalAddressAttribute, //pstl_addr
    pub(crate) bldg_nb: u32, //BldgNb
    pub(crate) pst_cd: u32, //PstCd
    pub(crate) twn_nm: String, //TwnNm
    pub(crate) ctry: String, //Ctry
    pub(crate) id: IdAttribute, //Id
}

#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct IdAttribute{
    pub(crate) org_id: OrgIdAttribute//OrgId
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct OrgIdAttribute{
    pub(crate) othr: OtherAttribute //Othr
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct OtherAttribute{
    pub(crate) id: String, //id
    pub(crate) schme_nm: ShemeNumberAttribute //SchmeNm
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct ShemeNumberAttribute{
    pub(crate) cd: String, //cd
}
#[derive(Debug, Deserialize, Serialize, Default, PartialEq)]
#[serde(rename_all = "PascalCase", default)]
pub(crate) struct PostalAddressAttribute{
    pub(crate) strt_nm: String, //strt_nm
    pub(crate) bldg_nb: String, //BldgNb
    pub(crate) pst_cd: String, //PstCd
    pub(crate) twn_nm: String, //TwnNm
    pub(crate) ctry: String,
    pub(crate) adr_line: Vec<String>
}

impl Default for DocumentCamt053{
    fn default() -> Self {
        Self {
            bk_to_cstmr_stmt: Vec::new()
        }
    }
}

impl DtAttribute {
    pub(crate) fn format_dt(dt_str: &str) -> Self {
        Self {
            dt: if dt_str.len() > 5 {
                format!("20{}-{}-{}", dt_str[0..2].to_string(),
                        dt_str[2..4].to_string(), dt_str[4..6].to_string())
            } else {
                "1979-01-01".to_string()
            }
        }
    }
}
