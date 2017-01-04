class CreateTracks < ActiveRecord::Migration[4.2]
  def self.up
    create_table :tracks, id: :uuid, default: "uuid_generate_v4()", force: true do |t|
      t.string :provider  , :null => false
      t.string :identifier, :null => false
      t.string :title     , :null => false
      t.string :url       , :null => false
      t.timestamps
    end
    add_index :tracks, [:provider, :identifier], :unique => true
  end

  def self.down
    drop_table :tracks
  end
end
