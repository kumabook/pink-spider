class AddVelocityToPlaylists < ActiveRecord::Migration[5.0]
  def change
    add_column :playlists, :velocity, :float, null: false, default: 0
  end
end
