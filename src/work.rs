use serde::{de, Deserialize};
use serde_with::{serde_as, DeserializeAs, DisplayFromStr};

use crate::{Genre, OpenOpusError, OpenOpusResult, Status, ID};

#[derive(Debug, Deserialize)]
struct Works {
    status: Status,
    works: Option<Vec<Work>>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Work {
    pub title: String,
    pub subtitle: String,
    #[serde(rename = "searchterms")]
    pub search_terms: String,
    #[serde_as(as = "IntStrBool")]
    pub popular: bool,
    #[serde_as(as = "IntStrBool")]
    pub recommended: bool,
    #[serde_as(as = "DisplayFromStr")]
    id: ID,
    pub genre: Genre,
}

impl Work {
    async fn list_common(url: &str) -> OpenOpusResult<Vec<Self>> {
        let result = reqwest::get(url).await?.json::<Works>().await?;

        dbg!(&result);

        match result.status {
            Status::Ok(_) => Ok(result.works.unwrap()),
            Status::Err(err) => Err(OpenOpusError::OpenOpusAPIError(err.error)),
        }
    }

    pub(crate) async fn list_by_composer_id_and_genre(
        composer_id: ID,
        genre: Genre,
    ) -> OpenOpusResult<Vec<Self>> {
        Self::list_common(&format!(
            "https://api.openopus.org/work/list/composer/{}/genre/{}.json",
            composer_id,
            genre.into_url_str()
        ))
        .await
    }

    pub(crate) async fn saerch_with_composer_id_and_genre(
        composer_id: ID,
        genre: Genre,
        word: &str,
    ) -> OpenOpusResult<Vec<Self>> {
        Self::list_common(&format!(
            "https://api.openopus.org/work/list/composer/{}/genre/{}/search/{}.json",
            composer_id,
            genre.into_url_str(),
            word
        ))
        .await
    }
}

struct IntStrBool;

impl<'de> DeserializeAs<'de, bool> for IntStrBool {
    fn deserialize_as<D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer).map_err(de::Error::custom)?;
        Ok(match s.as_ref() {
            "0" => false,
            "1" => true,
            _ => {
                return Err(de::Error::custom(
                    r#"Invalid string: {}. should be "0" or "1""#,
                ))
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_list_by_composer_id() -> anyhow::Result<()> {
        let _ = Work::list_by_composer_id_and_genre(130, Genre::All).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_list_by_composer_id_and_genre() -> anyhow::Result<()> {
        let _ = Work::list_by_composer_id_and_genre(2, Genre::Orchestral).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_saerch_with_composer_id() -> anyhow::Result<()> {
        let _ = Work::saerch_with_composer_id_and_genre(196, Genre::All, "Sonata").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_saerch_with_composer_id_and_genre() -> anyhow::Result<()> {
        let _ =
            Work::saerch_with_composer_id_and_genre(145, Genre::Chamber, "Cello Sonata").await?;
        Ok(())
    }
}
