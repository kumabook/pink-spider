use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};

#[derive(Debug, Clone)]
pub struct Feed {
    pub id:           String,
    pub subscribers:  Option<i64>,
    pub title:        Option<String>,
    pub description:  Option<String>,
    pub language:     Option<String>,
    pub velocity:     Option<f64>,
    pub website:      Option<String>,
    pub topics:       Option<Vec<String>>,
    pub status:       Option<String>,
    pub curated:      Option<bool>,
    pub featured:     Option<bool>,
    pub last_updated: Option<i64>,

    pub visual_url:   Option<String>,
    pub icon_url:     Option<String>,
    pub cover_url:    Option<String>,
}

impl ToJson for Feed {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("id".to_string()          , self.id.to_string().to_json());
        d.insert("subscribers".to_string() , self.subscribers.to_json());
        d.insert("title".to_string()       , self.title.to_json());
        d.insert("description".to_string() , self.description.to_json());
        d.insert("language".to_string()    , self.language.to_json());
        d.insert("velocity".to_string()    , self.velocity.to_json());
        d.insert("website".to_string()     , self.website.to_json());
        d.insert("topics".to_string()      , self.topics.to_json());
        d.insert("status".to_string()      , self.status.to_json());
        d.insert("curated".to_string()     , self.curated.to_json());
        d.insert("featured".to_string()    , self.featured.to_json());
        d.insert("last_updated".to_string(), self.last_updated.to_json());
        d.insert("visual_url".to_string()  , self.visual_url.to_json());
        d.insert("icon_url".to_string()    , self.icon_url.to_json());
        d.insert("cover_url".to_string()   , self.cover_url.to_json());
        Json::Object(d)
    }
}
