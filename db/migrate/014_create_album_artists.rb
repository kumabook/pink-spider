class CreateAlbumArtists < ActiveRecord::Migration[4.2]
  def self.up
    create_table :album_artists do |a|
      a.uuid :album_id , :null => false
      a.uuid :artist_id, :null => false
      a.timestamps
    end
    add_index :album_artists, [:album_id, :artist_id], :unique => true
  end

  def self.down
    drop_table :album_artists
  end
end
