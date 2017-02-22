class CreatePlaylists < ActiveRecord::Migration[4.2]
  def self.up
    create_table :playlists, id: :uuid, default: "uuid_generate_v4()", force: true do |t|
      t.string :provider     , null: false
      t.string :identifier   , null: false
      t.string :url          , null: false
      t.string :title        , null: false
      t.text   :description
      t.string :thumbnail_url
      t.string :artwork_url
      t.string :state        , null: false, default: 'alive'

      t.timestamp :published_at , null: false, default: -> { 'NOW()' }
      t.timestamps(null: false, default: -> { 'NOW()' })
    end
    add_index :playlists, [:provider, :identifier], unique: true
  end

  def self.down
    drop_table :playlists
  end
end
