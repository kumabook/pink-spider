class NonNullableAssociationsTimestamps < ActiveRecord::Migration[4.2]
  def change
    [:playlist_tracks,
     :album_tracks,
     :track_artists,
     :album_artists].each do |t|
      change_column(t, :created_at, :timestamp, null: false, default: -> { 'NOW()' })
      change_column(t, :updated_at, :timestamp, null: false, default: -> { 'NOW()' })
    end
  end
end
