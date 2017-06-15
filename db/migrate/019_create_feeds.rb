class CreateFeeds < ActiveRecord::Migration[5.0]
  def self.up
    create_table :feeds, id: :uuid, default: "uuid_generate_v4()", force: true do |t|
      t.string :url, null: false
      t.string :title, null: false, default: ""
      t.string :description
      t.string :language
      t.float :velocity, null: false, default: 0
      t.string :website
      t.string :state, null: false, default: 'alive'
      t.timestamp :last_updated, null: false, default: -> { 'NOW()' }
      t.timestamp :crawled, null: false, default: -> { 'NOW()' }
      t.string :visual_url
      t.string :icon_url
      t.string :cover_url
      t.timestamps(null: false, default: -> { 'NOW()' })
    end
    add_index :feeds, :url, :unique => true
    add_index :feeds, :last_updated, unique: false
    add_index :feeds, :crawled, unique: false
    add_index :feeds, :created_at, unique: false
    add_index :feeds, :updated_at, unique: false
  end

  def self.down
    drop_table :feeds
  end
end
