class AddAudioUrlToTracks < ActiveRecord::Migration[4.2]
  def change
    add_column :tracks, :audio_url, :string
  end
end
