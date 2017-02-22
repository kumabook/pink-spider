class CreatePlaylistEntries < ActiveRecord::Migration[4.2]
  def self.up
    create_table :playlist_entries do |t|
      t.uuid :playlist_id, :null => false
      t.uuid :entry_id,    :null => false
      t.timestamps
    end
    add_index :playlist_entries, [:playlist_id, :entry_id], :unique => true
  end

  def self.down
    drop_table :playlist_entries
  end
end
