class CreateEntries < ActiveRecord::Migration
  def self.up
    create_table :entries, id: :uuid, default: "uuid_generate_v4()", force: true do |t|
      t.string :url, :null => false
      t.timestamps
    end
    add_index :entries, :url, :unique => true
  end

  def self.down
    drop_table :entries
  end
end
