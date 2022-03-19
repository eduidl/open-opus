use serde::Deserialize;
use strum::IntoStaticStr;

use crate::{OpenOpusError, OpenOpusResult, Status, ID};

#[derive(Debug, Deserialize)]
struct Genres {
    status: Status,
    genres: Option<Vec<Genre>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, IntoStaticStr)]
pub enum Genre {
    #[strum(serialize = "all")]
    All,
    Popular,
    Recommended,
    Chamber,
    Keyboard,
    Orchestral,
    Stage,
    Vocal,
}

impl Genre {
    pub(crate) fn into_url_str(self) -> &'static str {
        let string: &'static str = self.into();
        string
    }

    pub(crate) async fn list_by_composer_id(composer_id: ID) -> OpenOpusResult<Vec<Self>> {
        let result = reqwest::get(format!(
            "https://api.openopus.org/genre/list/composer/{}.json",
            composer_id
        ))
        .await?
        .json::<Genres>()
        .await?;

        match result.status {
            Status::Ok(_) => Ok(result.genres.unwrap()),
            Status::Err(err) => Err(OpenOpusError::OpenOpusAPIError(err.error)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_list_by_composer_id() -> anyhow::Result<()> {
        let _ = Genre::list_by_composer_id(180).await?;
        Ok(())
    }
}
