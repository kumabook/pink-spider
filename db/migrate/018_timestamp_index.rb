class TimestampIndex < ActiveRecord::Migration[4.2]
  MODELS = [:entries, :tracks, :albums, :playlists, :artists]
  def self.up
    MODELS.each do |table|
      add_index table, :created_at, unique: false
      add_index table, :updated_at, unique: false
    end
  end

  def self.down
    MODELS.each do |table|
      remove_index table, :created_at
      remove_index table, :updated_at
    end
  end
end
