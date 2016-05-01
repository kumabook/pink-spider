class CreateTracks < ActiveRecord::Migration
  def self.up
    create_table :tracks, id: :uuid, default: "uuid_generate_v4()", force: true do |t|
      t.string :provider  , :null => false
      t.string :identifier, :null => false
      t.string :title     , :null => false
      t.string :url       , :null => false
      t.timestamps
    end
  end

  def self.down
    drop_table :tracks
  end
end
