class AddTextToEntries < ActiveRecord::Migration[4.2]
  def change
    add_column :entries, :text, :text
  end
end
