use serde::Deserialize;
// Serialize the xml below
// <Acct>
// <Id>
//     <Othr>
//         <Id>DD01100056869</Id>
//     </Othr>
// </Id>
// <Tp>
//     <Prtry>IBDA_DDA</Prtry>
// </Tp>
// <Ccy>USD</Ccy>
// <Nm>Sample Name 123</Nm>
// <Svcr>
//     <FinInstnId>
//         <BIC>GSCRUS30</BIC>
//         <Nm>Goldman Sachs Bank</Nm>
//     </FinInstnId>
// </Svcr>
// </Acct>
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Acct {
    #[serde(rename = "Id")]
    pub account_id: AccountId,
    #[serde(rename = "Tp")]
    pub tp: AccountTp,
    pub ccy: String,
    pub nm: String,
    pub svcr: Svcr,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountId {
    #[serde(rename = "Othr")]
    pub othr: Othr,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Othr {
    pub id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Tp")]
pub struct AccountTp {
    #[serde(rename = "Prtry")]
    pub prtry: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Svcr {
    pub fin_instn_id: FinInstnId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FinInstnId {
    #[serde(rename = "BIC")]
    pub bic: String,
    pub nm: String,
}

impl Acct {
    pub fn to_mt940_tag25(&self) -> String {
        let mut tag25: String = String::new();
        tag25.push_str(&format!("25:{}\n", self.account_id.othr.id));

        return tag25;
    }
}
