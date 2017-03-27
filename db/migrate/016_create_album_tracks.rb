class CreateAlbumTracks < ActiveRecord::Migration[4.2]
  def self.up
    create_table :album_tracks do |a|
      a.uuid :track_id, :null => false
      a.uuid :album_id, :null => false
      a.timestamps
    end
    add_index :album_tracks, [:track_id, :album_id], :unique => true
  end

  def self.down
    drop_table :album_tracks
  end
end
