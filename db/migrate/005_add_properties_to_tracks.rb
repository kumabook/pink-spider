class AddPropertiesToTracks < ActiveRecord::Migration
  def change
    add_column :tracks, :description  , :text
    add_column :tracks, :artist       , :string
    add_column :tracks, :thumbnail_url, :string
    add_column :tracks, :artwork_url  , :string
    add_column :tracks, :duration     , :integer, null: false, default: 0
    add_column :tracks, :published_at , :timestamp, null: false, default: -> { 'NOW()' }
    add_column :tracks, :state        , :string, null: false, default: 'alive'
  end
end
