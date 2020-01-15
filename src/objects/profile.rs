#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(transparent)]
pub struct PartialProfile(pub String);
