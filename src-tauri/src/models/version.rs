use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Version {
    pub major: u8, //26.
    pub minor: u8, //1.
    pub patch: u8, //3
}
