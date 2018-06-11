use std::io::Read;
use std::str::FromStr;
use get_env;
use http;
use error::Error;
use tendril::TendrilSink;
use xml5ever::rcdom::{RcDom};
use xml5ever::{QualName, LocalName};
use xml5ever::tree_builder::NodeOrText;
use xml5ever::tree_builder::TreeSink;
use xml5ever::interface::ElementFlags;
use xml5ever::interface::Attribute;
use xml5ever::serialize::serialize;
use xml5ever::tendril::StrTendril;
use xml5ever::rcdom::{Handle, NodeData};
use xml5ever::rcdom::NodeData::Element;
use xml5ever::driver::parse_document;

lazy_static! {
    static ref CLIENT_ID: String = {
        get_env::var("GRACENOTE_CLIENT_ID").unwrap_or("".to_string())
    };
    static ref CLIENT_WEB_ID: String = {
        get_env::var("GRACENOTE_CLIENT_WEB_ID").unwrap_or("".to_string())
    };
    static ref USER_ID: String = {
        get_env::var("GRACENOTE_USER_ID").unwrap_or("".to_string())
    };
}

pub struct Auth {
    client: String,
    user:   String,
}

pub struct Query {
    pub cmd:     String,
    pub mode:    String,
    pub texts:   Vec<Text>,
    pub range:   Option<Range>,
    pub options: Vec<QueryOption>,
}

pub struct Range {
    start: i64,
    end:   i64,
}

pub struct Text {
    pub target_type: String,
    pub value:       String,
}

#[derive(Debug, Clone)]
pub enum QueryOption {
    SelectExtended(Vec<SelectExtended>),
    SelectDetail(Vec<SelectDetail>),
}

impl QueryOption {
    pub fn parameter(self) -> String {
        match self {
            QueryOption::SelectExtended(_) => "SELECT_EXTENDED".to_string(),
            QueryOption::SelectDetail(_)   => "SELECT_DETAIL".to_string(),
        }
    }
    pub fn value(self) -> String {
        match self {
            QueryOption::SelectExtended(vals) => vals.iter().map(|v| v.clone().to_string()).collect::<Vec<_>>().join(","),
            QueryOption::SelectDetail(vals)   => vals.iter().map(|v| v.clone().to_string()).collect::<Vec<_>>().join(","),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SelectExtended {
    Mood,
    Tempo,
    Cover,
    Review,
    ArtistBiography,
    ArtistImage,
    Content,
    Link,
    ArtistOet,
}

impl SelectExtended {
    fn to_string(self) -> String {
        match self {
            SelectExtended::Mood            => "MOOD".to_string(),
            SelectExtended::Tempo           => "TEMPO".to_string(),
            SelectExtended::Cover           => "COVER".to_string(),
            SelectExtended::Review          => "REVIEW".to_string(),
            SelectExtended::ArtistBiography => "ARTIST_BIOGRAPHY".to_string(),
            SelectExtended::ArtistImage     => "ARTIST_IMAGE".to_string(),
            SelectExtended::Content         => "CONTENT".to_string(),
            SelectExtended::Link            => "LINK".to_string(),
            SelectExtended::ArtistOet       => "ARTIST_OET".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum SelectDetail {
    Genre3Level,
    Mood2Level,
    Tempo3Level,
    ArtistOrigin4Level,
    ArtistEra2Level,
    ArtistType2Level,
}

impl SelectDetail {
    fn to_string(&self) -> String {
        match self {
            SelectDetail::Genre3Level        => "GENRE:3LEVEL".to_string(),
            SelectDetail::Mood2Level         => "MOOD:2LEVEL".to_string(),
            SelectDetail::Tempo3Level        => "TEMPO:3LEVEL".to_string(),
            SelectDetail::ArtistOrigin4Level => "ARTIST_ORIGIN:4LEVEL".to_string(),
            SelectDetail::ArtistEra2Level    => "ARTIST_ERA:2LEVEL".to_string(),
            SelectDetail::ArtistType2Level   => "ARTIST_TYPE:2LEVEL".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Track {
    id:       String,
    title:    String,
    artists:  Vec<Artist>,
    genres:   Vec<Genre>,
    moods:    Vec<String>,
    tempos:   Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Album {
    id:       String,
    date:     String,
    title:    String,
    url:      String,
    pkg_lang: String,
    artists:  Vec<Artist>,
    genres:   Vec<Genre>,
    tracks:   Vec<Track>,
}

#[derive(Debug, Clone)]
pub struct Artist {
    name:    String,
    types:   Vec<String>,
    origins: Vec<String>,
    eras:    Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Genre {
    id:   String,
    num:  String,
    name: String,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub albums: Vec<Album>,
}

pub fn qual_name(name: &str) -> QualName {
    QualName::new(None, ns!(), LocalName::from(name))
}

pub fn auth() -> Auth {
    Auth {
        client: CLIENT_WEB_ID.to_string(),
        user:   USER_ID.to_string(),
    }
}

pub fn build_queries(a: Auth, q: Query) -> String {
    let mut bytes = vec![];
    let mut dom   = RcDom::default();
    let handle    = dom.document.clone();
    let queries   = dom.create_element(qual_name("QUERIES"),
                                       vec![],
                                       ElementFlags::default());

    let auth = dom.create_element(qual_name("AUTH"), vec![], ElementFlags::default());
    let client = dom.create_element(qual_name("CLIENT"), vec![], ElementFlags::default());
    dom.append(&client, NodeOrText::AppendText(
        StrTendril::from_str(&a.client).unwrap()
    ));
    let user = dom.create_element(qual_name("USER"), vec![], ElementFlags::default());
    dom.append(&user, NodeOrText::AppendText(
        StrTendril::from_str(&a.user).unwrap()
    ));
    dom.append(&auth, NodeOrText::AppendNode(client));
    dom.append(&auth, NodeOrText::AppendNode(user));

    let query = dom.create_element(qual_name("QUERY"), vec![
        Attribute {
            name: qual_name("CMD"),
            value: StrTendril::from_str(&q.cmd).unwrap(),
        },
    ], ElementFlags::default());

    let mode = dom.create_element(qual_name("MODE"), vec![], ElementFlags::default());
    dom.append(&mode, NodeOrText::AppendText(
        StrTendril::from_str(&q.mode).unwrap()
    ));
    dom.append(&query, NodeOrText::AppendNode(mode));

    for text in q.texts.iter() {
        let e = dom.create_element(qual_name("TEXT"), vec![
            Attribute {
                name: qual_name("TYPE"),
                value: StrTendril::from_str(&text.target_type).unwrap(),
            },
        ], ElementFlags::default());
        dom.append(&e, NodeOrText::AppendText(
            StrTendril::from_str(&text.value).unwrap()
        ));
        dom.append(&query, NodeOrText::AppendNode(e));
    }

    if let Some(range) = q.range {
        let r = dom.create_element(qual_name("RANGE"), vec![], ElementFlags::default());
        let s = dom.create_element(qual_name("START"), vec![], ElementFlags::default());
        let e = dom.create_element(qual_name("END"), vec![], ElementFlags::default());
        dom.append(&s, NodeOrText::AppendText(
            StrTendril::from_str(&range.start.to_string()).unwrap()
        ));
        dom.append(&e, NodeOrText::AppendText(
            StrTendril::from_str(&range.end.to_string()).unwrap()
        ));
        dom.append(&r, NodeOrText::AppendNode(s));
        dom.append(&r, NodeOrText::AppendNode(e));
        dom.append(&query, NodeOrText::AppendNode(r));
    }

    for option in q.options {
        let o = dom.create_element(qual_name("OPTION"), vec![], ElementFlags::default());
        let p = dom.create_element(qual_name("PARAMETER"), vec![], ElementFlags::default());
        dom.append(&p, NodeOrText::AppendText(
            StrTendril::from_str(&option.clone().parameter()).unwrap()
        ));
        let v = dom.create_element(qual_name("VALUE"), vec![], ElementFlags::default());
        dom.append(&v, NodeOrText::AppendText(
            StrTendril::from_str(&option.value()).unwrap()
        ));
        dom.append(&o, NodeOrText::AppendNode(p));
        dom.append(&o, NodeOrText::AppendNode(v));
        dom.append(&query, NodeOrText::AppendNode(o));
    }

    dom.append(&queries, NodeOrText::AppendNode(query));
    dom.append(&queries, NodeOrText::AppendNode(auth));
    dom.append(&handle, NodeOrText::AppendNode(queries));

    serialize(&mut bytes, &handle, Default::default()).ok();
    String::from_utf8(bytes).unwrap_or_default()
}

pub fn send(query: Query) -> Result<String, Error> {
    let queries = build_queries(auth(), query);
    let url = format!("https://c{}.web.cddbp.net/webapi/xml/1.0/", CLIENT_ID.to_string());
    let mut res = http::client().post(&url)
                                .body(queries)
                                .send().unwrap();
    let mut body = String::new();
    res.read_to_string(&mut body).unwrap();
    Ok(body)
}

pub fn parse(res: String) -> Result<Response, Error> {
    let dom = parse_document(RcDom::default(), Default::default())
        .from_utf8()
        .read_from(&mut res.as_bytes())
        .unwrap();
    let handle = dom.document.clone();
    let mut response = Response { albums: vec![] };
    for child in handle.children.borrow().iter() {
        match child.clone().data {
            Element { ref name, .. } => {
                let tag_name = name.local.as_ref();
                match tag_name.as_ref() {
                    "RESPONSES" => parse_as_response(child.clone(), &mut response),
                    _           => (),
                }
            },
            _ => (),
        }
    }
    Ok(response)
}

pub fn parse_as_response(handle: Handle, response: &mut Response) {
    for child in handle.children.borrow().iter() {
        match child.clone().data {
            Element { ref name, .. } => {
                let tag_name = name.local.as_ref();
                match tag_name.as_ref() {
                    "RESPONSE" => parse_as_response(child.clone(), response),
                    "ALBUM" => if let Some(album) = parse_as_album(child.clone()) {
                        response.albums.push(album)
                    },
                    "TRACK" => (),
                    _       => (),
                }
            },
            _ => (),
        }
    }
}


fn parse_as_album(handle: Handle) -> Option<Album> {
    let mut album: Album = Album {
        id:       "".to_string(),
        date:     "".to_string(),
        title:    "".to_string(),
        url:      "".to_string(),
        pkg_lang: "".to_string(),
        artists:  vec![],
        genres:   vec![],
        tracks:   vec![],
    };
    for child in handle.children.borrow().iter() {
        match child.clone().data {
            Element { ref name, .. } => {
                let tag_name = name.local.as_ref();
                match tag_name.as_ref() {
                    "GN_ID"    => album.id       = get_text(child.clone()),
                    "TITLE"    => album.title    = get_text(child.clone()),
                    "PKG_LANG" => album.pkg_lang = get_text(child.clone()),
                    "DATE"     => album.date     = get_text(child.clone()),
                    "URL"      => album.url      = get_text(child.clone()),
                    "GENRE"    => if let Some(genre) = parse_as_genre(child.clone()) {
                        album.genres.push(genre)
                    },
                    "TRACK"    => if let Some(track) = parse_as_track(child.clone()) {
                        album.tracks.push(track)
                    },
                    "ARTIST"   =>  album.artists.push(Artist {
                        name:    get_text(child.clone()),
                        types:   vec![],
                        origins: vec![],
                        eras:    vec![],
                    }),
                    "ARTIST_ORIGIN" => if let Some(last) = album.artists.last_mut() {
                        last.origins.push(get_text(child.clone()))
                    },
                    "ARTIST_TYPE"   => if let Some(last) = album.artists.last_mut() {
                        last.types.push(get_text(child.clone()))
                    },
                    "ARTIST_ERA"    => if let Some(last) = album.artists.last_mut() {
                        last.eras.push(get_text(child.clone()))
                    },
                    _     => (),
                }
            },
            _ => (),
        }
    }
    if !album.id.is_empty() {
        Some(album)
    } else {
        None
    }
}

fn parse_as_track(handle: Handle) -> Option<Track> {
    let mut track: Track = Track {
        id:       "".to_string(),
        title:    "".to_string(),
        moods:    vec![],
        tempos:   vec![],
        artists:  vec![],
        genres:   vec![],
    };
    for child in handle.children.borrow().iter() {
        match child.clone().data {
            Element { ref name, .. } => {
                let tag_name = name.local.as_ref();
                match tag_name.as_ref() {
                    "GN_ID"    => track.id       = get_text(child.clone()),
                    "TITLE"    => track.title    = get_text(child.clone()),
                    "MOOD"     => track.moods.push(get_text(child.clone())),
                    "TEMPO"    => track.tempos.push(get_text(child.clone())),
                    "GENRE"    => if let Some(genre) = parse_as_genre(child.clone()) {
                        track.genres.push(genre)
                    },
                    "ARTIST"   =>  track.artists.push(Artist {
                        name:    get_text(child.clone()),
                        types:   vec![],
                        origins: vec![],
                        eras:    vec![],
                    }),
                    "ARTIST_ORIGIN" => if let Some(last) = track.artists.last_mut() {
                        last.origins.push(get_text(child.clone()))
                    },
                    "ARTIST_TYPE"   => if let Some(last) = track.artists.last_mut() {
                        last.types.push(get_text(child.clone()))
                    },
                    "ARTIST_ERA"    => if let Some(last) = track.artists.last_mut() {
                        last.eras.push(get_text(child.clone()))
                    },
                    _     => (),
                }
            },
            _ => (),
        }
    }
    if !track.id.is_empty() {
        Some(track)
    } else {
        None
    }
}

fn parse_as_genre(handle: Handle) -> Option<Genre> {
    let mut genre = Genre {
        num:  "".to_string(),
        id:   "".to_string(),
        name: "".to_string(),
    };
    match handle.clone().data {
        Element { ref attrs, .. } => {
            genre.num = get_attr("NUM", &attrs.borrow()).unwrap_or("".to_string());
            genre.id  = get_attr("ID", &attrs.borrow()).unwrap_or("".to_string());
        },
        _ => (),
    }
    genre.name = get_text(handle);
    if !genre.name.is_empty() && !genre.id.is_empty() && !genre.num.is_empty() {
        return Some(genre)
    }
    return None
}

pub fn get_text(handle: Handle) -> String {
    for child in handle.children.borrow().iter() {
        match child.clone().data {
            NodeData::Text { ref contents } => return contents.borrow().to_string(),
            _ => (),
        }
    }
    return "".to_string();
}

pub fn get_attr(attr_name: &str, attrs: &Vec<Attribute>) -> Option<String> {
    for attr in attrs.iter() {
        if attr.name.local.as_ref() == attr_name {
            return Some(attr.value.to_string())
        }
    }
    None
}
