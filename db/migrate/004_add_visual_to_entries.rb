class AddVisualToEntries < ActiveRecord::Migration
  def change
    add_column :entries, :title      , :string
    add_column :entries, :description, :string
    add_column :entries, :visual_url , :string
    add_column :entries, :locale     , :string
  end
end
