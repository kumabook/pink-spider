class CreateTrackEntries < ActiveRecord::Migration
  def self.up
    create_table :track_entries do |t|
      t.uuid :track_id, :null => false
      t.uuid :entry_id, :null => false
      t.timestamps
    end
  end

  def self.down
    drop_table :track_entries
  end
end
