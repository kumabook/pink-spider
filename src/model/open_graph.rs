use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};

#[derive(Debug, Default, Clone)]
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
    pub locale_alternate: Vec<String>,
    pub site_name:        Option<String>,
}

impl Object {
    pub fn new<'a>(props: &'a Vec<(String, String)>) -> Object {
        let mut obj = Object::default();
        for prop in props.iter() {
            let key: &str = &(prop.0);
            let v         = prop.1.clone();
            match key {
                "title"       => { obj.title       = v; },
                "type"        => { obj.obj_type    = ObjectType::new(v); },
                "url"         => { obj.url         = v; },
                "description" => { obj.description = Some(v); },
                "determiner"  => { obj.determiner  = Some(Determiner::new(v)); },
                "locale"      => { obj.locale      = Some(v); },
                "site_name"   => { obj.site_name   = Some(v); },

                "image"            => { obj.images.push(Image::new(v)); },
                "video"            => { obj.videos.push(Video::new(v)); },
                "audio"            => { obj.audios.push(Audio::new(v)); },
                "locale:alternate" => {
                    obj.locale_alternate.push(v)
                },
                v if v.starts_with("image") => {
                },
                v if v.starts_with("music") => {
                },
                v if v.starts_with("video") => {
                },
                v if v.starts_with("article") => {
                },
                v if v.starts_with("book") => {
                },
                v if v.starts_with("profile") => {
                },
                _ => {},
            }
        }
        obj
    }
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

#[derive(Debug, Clone)]
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

impl Default for ObjectType {
    fn default() -> ObjectType  { ObjectType::Website }
}

impl ToJson for ObjectType {
    fn to_json(&self) -> Json {
        self.to_string().to_json()
    }
}

#[derive(Debug, Clone)]
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

impl Default for Determiner {
    fn default() -> Determiner  { Determiner::Blank }
}

impl ToJson for Determiner {
    fn to_json(&self) -> Json {
        self.to_string().to_json()
    }
}

#[derive(Debug, Default, Clone)]
pub struct Image {
    pub url:        String,
    pub secure_url: Option<String>,
    pub obj_type:   Option<String>,
    pub width:      Option<i32>,
    pub height:     Option<i32>
}

impl Image {
    pub fn new(url: String) -> Image {
        Image {
            url:        url,
            secure_url: None,
            obj_type:   None,
            width:      None,
            height:     None,
        }
    }
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

#[derive(Debug, Default, Clone)]
pub struct Video {
    pub url:        String,
    pub secure_url: Option<String>,
    pub obj_type:   Option<String>,
    pub width:      Option<i32>,
    pub height:     Option<i32>
}

impl Video {
    pub fn new(url: String) -> Video {
        Video {
            url:        url,
            secure_url: None,
            obj_type:   None,
            width:      None,
            height:     None,
        }
    }
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

#[derive(Debug, Default, Clone)]
pub struct Audio {
    pub url:        String,
    pub secure_url: Option<String>,
    pub obj_type:   Option<String>,
}

impl Audio {
    pub fn new(url: String) -> Audio {
        Audio { url: url, secure_url: None, obj_type: None }
    }
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
