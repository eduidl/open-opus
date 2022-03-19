use serde::Deserialize;
use strum::{EnumIter, IntoStaticStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, EnumIter, IntoStaticStr)]
pub enum Epoch {
    Medieval,
    Renaissance,
    Baroque,
    Classical,
    #[strum(serialize = "Early Romantic")]
    #[serde(rename = "Early Romantic")]
    EarlyRomantic,
    Romantic,
    #[strum(serialize = "Late Romantic")]
    #[serde(rename = "Late Romantic")]
    LateRomantic,
    #[strum(serialize = "20th Century")]
    #[serde(rename = "20th Century")]
    TwentiethCentury,
    #[strum(serialize = "Post-War")]
    #[serde(rename = "Post-War")]
    PostWar,
    #[strum(serialize = "21st Century")]
    #[serde(rename = "21st Century")]
    TwentyFirstCentury,
}

impl Epoch {
    pub(crate) fn into_url_str(self) -> &'static str {
        let string: &'static str = self.into();
        string
    }
}
