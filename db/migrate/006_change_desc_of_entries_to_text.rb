class ChangeDescOfEntriesToText < ActiveRecord::Migration[4.2]
  def change
    change_column(:entries, :description, :text)
  end
end
