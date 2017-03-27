class CreatePlaylistTracks < ActiveRecord::Migration[4.2]
  def self.up
    create_table :playlist_tracks do |a|
      a.uuid :track_id   , :null => false
      a.uuid :playlist_id, :null => false
      a.timestamps
    end
    add_index :playlist_tracks, [:track_id, :playlist_id], :unique => true
  end

  def self.down
    drop_table :playlist_tracks
  end
end
