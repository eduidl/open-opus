use chrono::NaiveDate;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr};

use crate::{Epoch, Genre, OpenOpusError, OpenOpusResult, Status, Work, ID};

#[derive(Debug, Deserialize)]
struct Composers {
    status: Status,
    composers: Option<Vec<Composer>>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct Composer {
    #[serde_as(as = "DisplayFromStr")]
    pub id: ID,
    pub name: String,
    pub complete_name: String,
    pub birth: Option<NaiveDate>,
    pub death: Option<NaiveDate>,
    pub epoch: Epoch,
    pub portrait: String,
}

impl Composer {
    async fn list_common(url: &str) -> OpenOpusResult<Vec<Self>> {
        let result = reqwest::get(url).await?.json::<Composers>().await?;

        dbg!(&result);

        match result.status {
            Status::Ok(_) => Ok(result.composers.unwrap()),
            Status::Err(err) => Err(OpenOpusError::OpenOpusAPIError(err.error)),
        }
    }

    /// List popular composers
    /// GET /composer/list/pop.json
    pub async fn list_popular() -> OpenOpusResult<Vec<Self>> {
        Self::list_common("https://api.openopus.org/composer/list/pop.json").await
    }

    /// List essential composers
    /// GET /composer/list/rec.json
    pub async fn list_essential() -> OpenOpusResult<Vec<Self>> {
        Self::list_common("https://api.openopus.org/composer/list/rec.json").await
    }

    /// List composers by first letter
    /// GET /composer/list/name/<first letter>.json
    pub async fn list_by_first_letter(first_letter: char) -> OpenOpusResult<Vec<Self>> {
        Self::list_common(&format!(
            "https://api.openopus.org/composer/list/name/{}.json",
            first_letter
        ))
        .await
    }

    /// List composers by period
    /// GET /composer/list/epoch/<period>>.json
    pub async fn list_by_period(epoch: Epoch) -> OpenOpusResult<Vec<Self>> {
        Self::list_common(
            &format!(
                "https://api.openopus.org/composer/list/epoch/{}.json",
                epoch.into_url_str()
            )
            .to_string(),
        )
        .await
    }

    /// Search composers by name
    /// GET /composer/list/search/<search word>.json
    pub async fn search(word: &str) -> OpenOpusResult<Vec<Self>> {
        Self::list_common(
            &format!(
                "https://api.openopus.org/composer/list/search/{}.json",
                word
            )
            .to_string(),
        )
        .await
    }

    /// List composers by ID
    /// GET /composer/list/ids/<id>.json
    pub async fn get_by_id(id: ID) -> OpenOpusResult<Self> {
        Self::list_common(
            &format!("https://api.openopus.org/composer/list/ids/{}.json", id).to_string(),
        )
        .await
        .map(|v| v.into_iter().next().unwrap())
    }

    /// List genres by composer ID
    /// GET /genre/list/composer/<composer id>.json
    pub async fn genres(&self) -> OpenOpusResult<Vec<Genre>> {
        Genre::list_by_composer_id(self.id).await
    }

    /// List works by composer ID
    /// GET /work/list/composer/<composer id>/genre/all.json
    pub async fn works(&self) -> OpenOpusResult<Vec<Work>> {
        Work::list_by_composer_id_and_genre(self.id, Genre::All).await
    }

    /// List popular works by composer ID
    /// GET /work/list/composer/<composer id>/genre/Popular.json
    pub async fn popular_works(&self) -> OpenOpusResult<Vec<Work>> {
        Work::list_by_composer_id_and_genre(self.id, Genre::Popular).await
    }

    /// List essential works by composer ID
    /// GET /work/list/composer/<composer id>/genre/Recommended.json
    pub async fn recommended_works(&self) -> OpenOpusResult<Vec<Work>> {
        Work::list_by_composer_id_and_genre(self.id, Genre::Recommended).await
    }

    /// List works by composer ID and genre
    /// GET /work/list/composer/<composer id>/genre/<genre>.json
    pub async fn works_by_genre(&self, genre: Genre) -> OpenOpusResult<Vec<Work>> {
        Work::list_by_composer_id_and_genre(self.id, genre).await
    }

    // Search works by composer ID and title
    // GET /work/list/composer/<composer id>/genre/all/search/<search word>.json
    pub async fn search_works(&self, search_word: &str) -> OpenOpusResult<Vec<Work>> {
        Work::saerch_with_composer_id_and_genre(self.id, Genre::All, search_word).await
    }

    /// Search works by composer ID, genre and title
    /// GET /work/list/composer/<composer id>/genre/<genre>/search/<search word>.json
    pub async fn search_works_with_genre(
        &self,
        search_word: &str,
        genre: Genre,
    ) -> OpenOpusResult<Vec<Work>> {
        Work::saerch_with_composer_id_and_genre(self.id, genre, search_word).await
    }
}

#[cfg(test)]
mod test {
    use std::{thread, time::Duration};

    use super::*;

    #[tokio::test]
    async fn test_list_popular() -> anyhow::Result<()> {
        let _ = Composer::list_popular().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_list_essential() -> anyhow::Result<()> {
        let _ = Composer::list_essential().await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_filter_by_first_letter() -> anyhow::Result<()> {
        let _ = Composer::list_by_first_letter('A').await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_filter_by_period() -> anyhow::Result<()> {
        use strum::IntoEnumIterator;

        for epoch in Epoch::iter() {
            let _ = Composer::list_by_period(epoch).await?;

            thread::sleep(Duration::from_millis(100));
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_search() -> anyhow::Result<()> {
        let _ = Composer::search("bruc").await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_by_id() -> anyhow::Result<()> {
        let _ = Composer::get_by_id(186).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_by_id_not_exists() -> anyhow::Result<()> {
        assert!(matches!(
            Composer::get_by_id(999999999).await,
            Err(OpenOpusError::OpenOpusAPIError(_))
        ));
        Ok(())
    }
}
