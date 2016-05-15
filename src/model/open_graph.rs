use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};

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

impl ToJson for Object {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("title".to_string()           , self.title.to_json());
        d.insert("type".to_string()            , self.obj_type.to_json());
        d.insert("url".to_string()             , self.url.to_json());

        d.insert("images".to_string(),
                 Json::Array(self.images.iter().map(|x| x.to_json()).collect()));
        d.insert("audios".to_string(),
                 Json::Array(self.audios.iter().map(|x| x.to_json()).collect()));
        d.insert("videos".to_string(),
                 Json::Array(self.videos.iter().map(|x| x.to_json()).collect()));

        d.insert("description".to_string()     , self.description.to_json());
        d.insert("determiner".to_string()      , self.determiner.to_json());
        d.insert("locale".to_string()          , self.locale.to_json());
        d.insert("locale_alternate".to_string(), self.locale.to_json());
        d.insert("site_name".to_string()       , self.locale.to_json());

        Json::Object(d)
    }
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
    VideoOther,
}

impl ObjectType {
    fn to_string(&self) -> String {
        match *self {
            ObjectType::Article      => "article",
            ObjectType::Book         => "book",
            ObjectType::Profile      => "profile",
            ObjectType::Website      => "website",
            ObjectType::Song         => "music.song",
            ObjectType::Album        => "music.album",
            ObjectType::Playlist     => "music.playlist",
            ObjectType::RadioStation => "music.radio_station",
            ObjectType::Movie        => "video.movie",
            ObjectType::Episode      => "video.episode",
            ObjectType::TVShow       => "video.tv_show",
            ObjectType::VideoOther   => "video.other"
        }.to_string()
    }
    pub fn new(str: String) -> ObjectType {
        match str.as_ref() {
            "article"             => ObjectType::Article,
            "book"                => ObjectType::Book,
            "profile"             => ObjectType::Profile,
            "website"             => ObjectType::Website,
            "music.song"          => ObjectType::Song,
            "music.album"         => ObjectType::Album,
            "music.playlist"      => ObjectType::Playlist,
            "music.radio_station" => ObjectType::RadioStation,
            "video.movie"         => ObjectType::Movie,
            "video.episode"       => ObjectType::Episode,
            "video.tv_show"       => ObjectType::TVShow,
            "video.other"         => ObjectType::VideoOther,
            _                     => ObjectType::Website,
        }
    }
}


impl ToJson for ObjectType {
    fn to_json(&self) -> Json {
        self.to_string().to_json()
    }
}

#[derive(Debug)]
pub enum Determiner {
    A,
    An,
    The,
    Blank,
    Auto,
}

impl Determiner {
    fn to_string(&self) -> String {
        match *self {
            Determiner::A     => "a",
            Determiner::An    => "an",
            Determiner::The   => "the",
            Determiner::Blank => "",
            Determiner::Auto  => "auto",
        }.to_string()
    }
    pub fn new(str: String) -> Determiner {
        match str.as_ref() {
            "a"     => Determiner::A,
            "an"    => Determiner::An,
            "the"   => Determiner::The,
            "auto"  => Determiner::Auto,
            _       => Determiner::Blank,
        }
    }
}

impl ToJson for Determiner {
    fn to_json(&self) -> Json {
        self.to_string().to_json()
    }
}

#[derive(Debug)]
pub struct Image {
    pub url:        String,
    pub secure_url: Option<String>,
    pub obj_type:   Option<String>,
    pub width:      Option<i32>,
    pub height:     Option<i32>
}

impl ToJson for Image {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("url".to_string()       , self.url.to_json());
        d.insert("secure_url".to_string(), self.secure_url.to_json());
        d.insert("type".to_string()      , self.obj_type.to_json());
        d.insert("width".to_string()     , self.width.to_json());
        d.insert("height".to_string()    , self.height.to_json());
        Json::Object(d)
    }
}

#[derive(Debug)]
pub struct Video {
    pub url:        String,
    pub secure_url: Option<String>,
    pub obj_type:   Option<String>,
    pub width:      Option<i32>,
    pub height:     Option<i32>
}

impl ToJson for Video {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("url".to_string()       , self.url.to_json());
        d.insert("secure_url".to_string(), self.secure_url.to_json());
        d.insert("type".to_string()      , self.obj_type.to_json());
        d.insert("width".to_string()     , self.width.to_json());
        d.insert("height".to_string()    , self.height.to_json());
        Json::Object(d)
    }
}

#[derive(Debug)]
pub struct Audio {
    pub url:        String,
    pub secure_url: Option<String>,
    pub obj_type:   Option<String>,
}

impl ToJson for Audio {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert("url".to_string()       , self.url.to_json());
        d.insert("secure_url".to_string(), self.secure_url.to_json());
        d.insert("type".to_string()      , self.obj_type.to_json());
        Json::Object(d)
    }
}
