use std::collections::BTreeMap;
use uuid::Uuid;
use chrono::{NaiveDateTime, Utc};
use postgres;
use error::Error;
use super::{conn, Model};
use model::track::{Track, PROPS as TRACK_PROPS};
use model::playlist::Playlist;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlaylistTrack {
    pub playlist_id:   Uuid,
    pub track_id:      Uuid,
    pub created_at:    NaiveDateTime,
    pub updated_at:    NaiveDateTime,
    pub track:         Track,
}

static PROPS: [&'static str; 4]  = ["playlist_id",
                                    "track_id",
                                    "created_at",
                                    "updated_at"];

impl PlaylistTrack {
    fn props_str(prefix: &str) -> String {
        PROPS
            .iter()
            .map(|&p| format!("{}{}", prefix, p))
            .collect::<Vec<String>>().join(",")
    }
    fn row_to_item(row: postgres::rows::Row) -> Self {
        let offset = TRACK_PROPS.len();
        PlaylistTrack {
            playlist_id: row.get(offset),
            track_id:    row.get(offset + 1),
            created_at:  row.get(offset + 2),
            updated_at:  row.get(offset + 3),
            track:       Track::row_to_item(row),
        }
    }
    pub fn find_by_playlist_ids(playlist_ids: Vec<Uuid>) -> Result<BTreeMap<Uuid, Vec<PlaylistTrack>>, Error> {
        let conn = conn().unwrap();
        let sql = format!("SELECT {}, {} FROM tracks
                      LEFT OUTER JOIN playlist_tracks
                      ON playlist_tracks.track_id = tracks.id
                      WHERE playlist_tracks.playlist_id = ANY($1)
                      ORDER BY playlist_tracks.created_at DESC LIMIT {}",
                          Track::props_str("tracks."),
                          PlaylistTrack::props_str("playlist_tracks."),
                          playlist_ids.len() * 200);
        let stmt = conn.prepare(&sql).map_err(|e| {
            println!("{:?}", e);
            e
        })?;
        let rows = stmt.query(&[&playlist_ids]).map_err(|e| {
            println!("{:?}", e);
            e
        })?;
        let mut items: BTreeMap<Uuid, Vec<PlaylistTrack>> = BTreeMap::new();
        for id in playlist_ids.iter() {
            items.insert(*id, vec![]);
        }
        for row in rows.iter() {
            let id: Uuid = row.get(TRACK_PROPS.len());
            if let Some(playlist_tracks) = items.get_mut(&id) {
                let pt = PlaylistTrack::row_to_item(row);
                playlist_tracks.push(pt);
            }
        };
        Ok(items)
    }
    pub fn upsert(playlist: &Playlist, track: &Track) -> Result<PlaylistTrack, Error> {
        let conn = conn()?;
        let stmt = conn.prepare("INSERT INTO playlist_tracks
                      (track_id, playlist_id, created_at, updated_at)
                      VALUES ($1, $2, $3, $4)
                      ON CONFLICT (track_id, playlist_id)
                      DO UPDATE SET updated_at=$4
                      RETURNING playlist_tracks.created_at, playlist_tracks.updated_at")?;
        let now = Utc::now().naive_utc();
        let rows = stmt.query(&[&track.id, &playlist.id, &now, &now])?;
        let row = rows.iter().next().ok_or(Error::Unexpected)?;
        Ok(PlaylistTrack {
            playlist_id: playlist.id,
            track_id:    track.id,
            created_at:  row.get(0),
            updated_at:  row.get(1),
            track:       track.clone(),
        })
    }
}
