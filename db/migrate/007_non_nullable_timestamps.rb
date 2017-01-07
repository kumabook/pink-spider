class NonNullableTimestamps < ActiveRecord::Migration[4.2]
  def change
    change_column(:entries, :created_at, :timestamp, null: false, default: -> { 'NOW()' })
    change_column(:entries, :updated_at, :timestamp, null: false, default: -> { 'NOW()' })
    change_column(:tracks,  :created_at, :timestamp, null: false, default: -> { 'NOW()' })
    change_column(:tracks,  :updated_at, :timestamp, null: false, default: -> { 'NOW()' })
  end
end
