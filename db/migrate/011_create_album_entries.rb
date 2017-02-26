class CreateAlbumEntries < ActiveRecord::Migration[4.2]
  def self.up
    create_table :album_entries do |t|
      t.uuid :album_id, :null => false
      t.uuid :entry_id, :null => false
      t.timestamps
    end
    add_index :album_entries, [:album_id, :entry_id], :unique => true
  end

  def self.down
    drop_table :album_entries
  end
end
