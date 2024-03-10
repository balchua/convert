use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GrpHdr {
    pub msg_id: String,
    pub cre_dt_tm: String,
    pub msg_rcpt: MsgRcpt,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MsgRcpt {
    #[serde(rename = "Nm")]
    pub nm: String,
    pub id: MsgRcptId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(rename = "Id")]
pub struct MsgRcptId {
    pub org_id: MsgRcptOrgId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MsgRcptOrgId {
    #[serde(rename = "Othr")]
    pub othr: MsgRcptOthr,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MsgRcptOthr {
    pub id: String,
}

impl GrpHdr {
    pub fn to_mt940_header(&self) -> String {
        let mut header: String = String::new();
        header.push_str(&format!("20:{}\n", self.msg_id));
        return header;
    }
}
