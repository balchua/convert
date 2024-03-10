use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ntry {
    pub ntry_ref: String,
    pub amt: Amt,
    pub cdt_dbt_ind: String,
    pub rvsl_ind: String,
    pub sts: String,
    pub bookg_dt: DtTm,
    pub val_dt: Dt,
    pub bk_tx_cd: BkTxCd,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Amt {
    #[serde(rename = "$value")]
    pub value: String,
    pub ccy: String,
}

#[derive(Debug, Deserialize)]
pub struct Dt {
    #[serde(rename = "Dt")]
    pub dt: String,
}
#[derive(Debug, Deserialize)]
pub struct DtTm {
    #[serde(rename = "DtTm")]
    pub dt_tm: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BkTxCd {
    pub prtry: Prtry,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Prtry {
    pub cd: String,
}

impl Ntry {
    pub fn to_mt940_transactions(&self) -> String {
        let mut transaction: String = String::new();
        transaction.push_str(&format!("61:{}{}\n", self.bookg_dt.dt_tm, self.val_dt.dt));
        transaction.push_str(&format!("86:{}{}\n", self.bk_tx_cd.prtry.cd, self.ntry_ref));
        transaction.push_str(&format!("86:{}{}\n", self.amt.value, self.amt.ccy));
        transaction.push_str(&format!("86:{}{}\n", self.cdt_dbt_ind, self.rvsl_ind));
        transaction.push_str(&format!("86:{}{}\n", self.sts, self.bookg_dt.dt_tm));
        return transaction;
    }
}
