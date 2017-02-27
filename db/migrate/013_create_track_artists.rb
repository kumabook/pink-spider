class CreateTrackArtists < ActiveRecord::Migration[4.2]
  def self.up
    create_table :track_artists do |t|
      t.uuid :track_id , :null => false
      t.uuid :artist_id, :null => false
      t.timestamps
    end
    add_index :track_artists, [:track_id, :artist_id], :unique => true
  end

  def self.down
    drop_table :track_artists
  end
end
