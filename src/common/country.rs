#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
pub struct Country {
    pub iso_3166_1: String,
    pub name: String,
}
