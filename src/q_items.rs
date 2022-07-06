use std::collections::VecDeque;
use std::error::Error;

use crate::URL;

use log::{info, warn, error};

#[derive(Debug, Clone)]
pub struct QItem(String);

impl QItem {
    #[async_recursion::async_recursion]
    pub async fn merge(&self, other: &QItem, retries: usize) -> Result<(), Box<dyn Error>> {
        if retries >= 3 {
            error!("Failed to merge {} and {} after {} retries", self.0, other.0, retries);
            return Err("Failed to merge".into());
        }

        let url = URL
            .lock()
            .expect("Failed to read URL variable")
            .as_ref()
            .expect("A Wikibase URL is expected")
            .clone();

        let client = reqwest::Client::new();

        let post_params = vec![
            ("action", "wbmergeitems"),
            ("fromid", &other.0),
            ("toid", &self.0),
            ("token", "+\\"),
        ];

        let response = client.post(url).form(&post_params).send().await?;

        if let Some(error) = response.headers().get("mediawiki-api-error") {
            warn!(
                "merging {} with {}. Wikibase returned an error : {}",
                other.0,
                self.0,
                error.to_str()?
            );

            if error.to_str().unwrap().contains("DBConnectionError") {
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                warn!("retrying...");
                self.merge(other, retries+1).await?
            }
        } else {
            match response.status() {
                reqwest::StatusCode::OK => info!("{} merged with {}", other.0, self.0),
                _ => warn!("merge failed: {:?}", response),
            }
        }

        Ok(())
    }
}

impl TryFrom<&str> for QItem {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let trimed_value = value.trim();
        let mut chars = trimed_value.chars().collect::<VecDeque<char>>();

        match (chars.pop_front(), chars.iter().all(|c| c.is_numeric())) {
            (Some('Q'), true) => Ok(QItem(trimed_value.to_string())),
            _ => Err(format!("{} is not a valid QItem", value).into()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct QItems(VecDeque<QItem>);

impl QItems {
    pub async fn merge(mut self) -> Result<(), Box<dyn Error>> {
        let last = self.0.pop_back();

        let mut iterator = self.0.iter();

        while let (Some(last), Some(next)) = (&last, iterator.next()) {
            last.merge(next, 0).await?;
        }

        Ok(())
    }
}

impl TryFrom<&String> for QItems {
    type Error = Box<dyn Error>;

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let q_items = value
            .split(',')
            .map(QItem::try_from)
            .collect::<Result<VecDeque<_>, _>>()?;
        Ok(QItems(q_items))
    }
}
