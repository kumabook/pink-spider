class CreateTrackEntries < ActiveRecord::Migration
  def self.up
    create_table :track_entries do |t|
      t.uuid :track_id, :null => false
      t.uuid :entry_id, :null => false
      t.timestamps
    end
    add_index :track_entries, [:track_id, :entry_id], :unique => true
  end

  def self.down
    drop_table :track_entries
  end
end
