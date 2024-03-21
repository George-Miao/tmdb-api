use std::fmt::Display;

#[cfg(feature = "ts")]
use ts_rs::TS;

pub mod country;
pub mod credits;
pub mod image;
pub mod keyword;
pub mod language;
pub mod release_date;
pub mod status;
pub mod video;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
pub struct PaginatedResult<T> {
    pub page: u64,
    pub total_results: u64,
    pub total_pages: u64,
    pub results: Vec<T>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Movie,
    Tv,
}

impl Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MediaType::Movie => "movie",
            MediaType::Tv => "tv",
        };

        write!(f, "{}", s)
    }
}
