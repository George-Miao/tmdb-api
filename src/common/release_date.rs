#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
pub struct LocatedReleaseDates {
    pub iso_3166_1: String,
    pub release_dates: Vec<ReleaseDate>,
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
#[repr(u8)]
pub enum ReleaseDateKind {
    Premiere = 1,
    TheatricalLimited = 2,
    Theatrical = 3,
    Digital = 4,
    Physical = 5,
    TV = 6,
}

#[derive(Debug, Deserialize, Serialize)]
#[cfg_attr(feature = "ts", derive(ts_rs::TS))]
pub struct ReleaseDate {
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub certification: Option<String>,
    #[serde(deserialize_with = "crate::util::empty_string::deserialize")]
    pub iso_639_1: Option<String>,
    #[serde(default, deserialize_with = "crate::util::empty_string::deserialize")]
    pub note: Option<String>,
    pub release_date: chrono::DateTime<chrono::Utc>,
    #[serde(rename = "type")]
    pub kind: ReleaseDateKind,
}
