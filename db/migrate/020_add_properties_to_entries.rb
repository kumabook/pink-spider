class AddPropertiesToEntries < ActiveRecord::Migration[4.2]
  def change
    add_column :entries, :summary    , :text
    add_column :entries, :content    , :text
    add_column :entries, :author     , :string
    add_column :entries, :crawled    , :timestamp, null: false, default: -> { 'NOW()' }
    add_column :entries, :published  , :timestamp, null: false, default: -> { 'NOW()' }
    add_column :entries, :updated    , :timestamp, null: true , default: -> { 'NOW()' }
    add_column :entries, :fingerprint, :string, null: false, default: ""
    add_column :entries, :alternate  , :jsonb, null: false, default: "null"
    add_column :entries, :enclosure  , :jsonb, null: false, default: "null"
    add_column :entries, :keywords   , :jsonb, null: false, default: "null"
    add_column :entries, :origin_id  , :string, null: false, default: ""
    add_column :entries, :feed_id    , :uuid, null: true

    add_index :entries, :published
    add_index :entries, :origin_id
  end
end

