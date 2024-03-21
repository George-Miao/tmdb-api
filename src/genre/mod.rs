#[cfg(feature = "commands")]
pub mod list;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
pub struct Genre {
    pub id: u64,
    pub name: String,
}
