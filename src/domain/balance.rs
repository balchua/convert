use serde::Deserialize;

//<Bal>
//   <Tp>
//     <CdOrPrtry>
//       <Cd>OPBD</Cd>
//     </CdOrPrtry>
//   </Tp>
//   <Amt Ccy="USD">843686.20</Amt>
//   <CdtDbtInd>DBIT</CdtDbtInd>
//   <Dt>
//     <DtTm>2023-09-31T20:00:00.000</DtTm>
//   </Dt>
// </Bal>
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "Bal")]
pub struct Balance {
    pub tp: Option<Tp>,
    #[serde(rename = "Amt")]
    pub amount: Option<Amount>,
    pub cdt_dbt_ind: String,
    pub dt: Dt,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Tp {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prty: Option<CdOrPrtry>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Amount {
    pub ccy: Option<String>,
    #[serde(rename = "$value")]
    pub amount: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct CdOrPrtry {
    #[serde(rename = "Cd")]
    pub cd: String,
}
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Dt {
    #[serde(rename = "DtTm")]
    pub dt_tm: String,
}

impl Balance {
    pub fn to_mt940_opening_balance(&self) -> String {
        let mut opening_balance: String = String::new();
        if let Some(tp) = &self.tp {
            if let Some(cd_or_prty) = &tp.cd_or_prty {
                if cd_or_prty.cd == "OPBD" {
                    opening_balance.push_str(&format!(
                        "60F:{}{}\n",
                        self.cdt_dbt_ind.chars().nth(0).unwrap().to_string(),
                        self.amount.as_ref().unwrap().amount
                    ));
                }
            }
        }

        return opening_balance;
    }
}
