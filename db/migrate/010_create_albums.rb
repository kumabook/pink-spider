class CreateAlbums < ActiveRecord::Migration[4.2]
  def self.up
    create_table :albums, id: :uuid, default: "uuid_generate_v4()", force: true do |t|
      t.string :provider     , null: false
      t.string :identifier   , null: false
      t.string :owner_id
      t.string :owner_name
      t.string :url          , null: false
      t.string :title        , null: false
      t.text   :description
      t.string :thumbnail_url
      t.string :artwork_url
      t.string :state        , null: false, default: 'alive'

      t.timestamp :published_at , null: false, default: -> { 'NOW()' }
      t.timestamps(null: false, default: -> { 'NOW()' })
    end
    add_index :albums, [:provider, :identifier], unique: true
  end

  def self.down
    drop_table :albums
  end
end
