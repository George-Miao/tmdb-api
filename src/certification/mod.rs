#[cfg(feature = "commands")]
pub mod list;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
pub struct Certification {
    pub certification: String,
    pub meaning: String,
    pub order: usize,
}
