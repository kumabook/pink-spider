#[derive(Debug)]
pub struct Object {
    pub title:            String,
    pub obj_type:         ObjectType,
    pub url:              String,

    pub images:           Vec<Image>,
    pub audios:           Vec<Audio>,
    pub videos:           Vec<Video>,

    pub description:      Option<String>,
    pub determiner:       Option<Determiner>,
    pub locale:           Option<String>,
    pub locale_alternate: Option<Vec<String>>,
    pub site_name:        Option<String>,
}

#[derive(Debug)]
pub enum ObjectType {
    // No Vetical
    Article,
    Book,
    Profile,
    Website,
    // Music
    Song,
    Album,
    Playlist,
    RadioStation,
    // Video
    Movie,
    Episode,
    TVShow,
}

#[derive(Debug)]
pub enum Determiner {
    A,
    An,
    The,
    Blank,
    Auto,
}

#[derive(Debug)]
pub struct Image {
    pub url:        String,
    pub secure_url: Option<String>,
    pub obj_type:   Option<String>,
    pub width:      Option<i32>,
    pub height:     Option<i32>
}

#[derive(Debug)]
pub struct Video {
    pub url:        String,
    pub secure_url: Option<String>,
    pub obj_type:   Option<String>,
    pub width:      Option<i32>,
    pub height:     Option<i32>
}

#[derive(Debug)]
pub struct Audio {
    pub url:        String,
    pub secure_url: Option<String>,
    pub obj_type:   Option<String>,
}

