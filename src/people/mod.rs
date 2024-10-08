#[cfg(feature = "commands")]
pub mod details;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
pub struct PersonShort {
    pub id: u64,
    pub credit_id: Option<String>,
    pub name: String,
    pub gender: Option<u64>,
    pub profile_path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
pub struct Person {
    #[serde(flatten)]
    pub inner: PersonShort,
    pub adult: bool,
    pub also_known_as: Vec<String>,
    pub biography: String,
    #[serde(with = "crate::util::date")]
    pub birthday: chrono::NaiveDate,
    #[serde(with = "crate::util::optional_date")]
    pub deathday: Option<chrono::NaiveDate>,
    pub homepage: Option<String>,
    pub imdb_id: String,
    pub known_for_department: String,
    pub popularity: f64,
    pub place_of_birth: Option<String>,
    pub profile_path: Option<String>,
}
