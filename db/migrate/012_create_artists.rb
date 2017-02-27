class CreateArtists < ActiveRecord::Migration[4.2]
  def self.up
    create_table :artists, id: :uuid, default: "uuid_generate_v4()", force: true do |t|
      t.string :provider     , null: false
      t.string :identifier   , null: false
      t.string :url          , null: false
      t.string :name         , null: false
      t.string :thumbnail_url
      t.string :artwork_url

      t.timestamps(null: false, default: -> { 'NOW()' })
    end
    add_index :artists, [:provider, :identifier], unique: true
  end

  def self.down
    drop_table :artists
  end
end
