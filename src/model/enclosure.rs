use uuid::Uuid;
use error::Error;
use model::provider::Provider;
use super::{conn, Model};

pub trait Enclosure<'a> where Self: Model<'a> {
    fn new(provider: Provider, identifier: String) -> Self;
    fn set_owner_id(&mut self, owner_id: Option<String>) -> &mut Self;
    fn set_url(&mut self, url: String) -> &mut Self;
    fn fetch_props(&mut self) -> &mut Self;
    fn find_by_entry_id(entry_id: Uuid) -> Vec<Self>;
    fn find_by(provider: &Provider, identifier: &str) -> Result<Self, Error> {
        let conn = conn().unwrap();
        let stmt = conn.prepare(
            &format!("SELECT {} FROM {}
                     WHERE provider = $1 AND identifier = $2
                     ORDER BY published_at DESC",
                     Self::props_str(""), Self::table_name())).unwrap();
        let rows = stmt.query(&[&(*provider).to_string(), &identifier]).unwrap();
        let items = Self::rows_to_items(rows);
        if items.len() > 0 {
            return Ok(items[0].clone());
        }
        return Err(Error::NotFound)
    }
    fn find_or_create(provider: Provider, identifier: String) -> Result<Self, Error> {
        return match Self::find_by(&provider, &identifier) {
            Ok(item) => Ok(item),
            Err(_)   => Self::new(provider, identifier).create()
        }
    }
}
