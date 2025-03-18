use std::collections::HashMap;

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Images {
    pub original: String,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MangaAttributes {
    pub description: Option<String>,
    pub canonical_title: String,
    pub average_rating: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub age_rating: Option<String>,
    pub age_rating_guide: Option<String>,
    pub status: String,
    pub poster_image: Images,
    pub volume_count: Option<u8>,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Manga {
    pub id: String,
    pub links: HashMap<String, String>,
    pub attributes: MangaAttributes,
}

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct ResponseError {
    pub title: String,
    pub detail: String,
    pub code: String,
    pub status: String,
}

#[derive(Debug, Clone)]
pub enum ResponseType {
    Ok(Vec<Manga>),
    Err(ResponseError),
}

impl Manga {
    pub async fn fetch(id: &str) -> ResponseType {
        let id = id.to_owned();

        let client = reqwest::Client::new();

        let response = client
            .get(format!("https://kitsu.io/api/edge/manga/{id}"))
            .header("Content-Type", "application/vnd.api+json")
            .send()
            .await
            .expect("Unable to send request");

        let response = if response.status().is_success() {
            ResponseType::Ok(vec![response
                .json::<HashMap<String, Manga>>()
                .await
                .expect("Unable to parse response")
                .get("data")
                .unwrap()
                .clone()])
        } else {
            ResponseType::Err(
                response
                    .json::<HashMap<String, Vec<ResponseError>>>()
                    .await
                    .expect("Unable to parse error response")
                    .get("errors")
                    .unwrap()
                    .first()
                    .unwrap()
                    .clone(),
            )
        };

        response
    }
}
