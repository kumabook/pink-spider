require 'active_record'
require 'yaml'
require 'erb'
require 'logger'


namespace :db do
  env = ENV['ENV'] || 'development'

  task :default => :migrate

  task :environment do
    config = YAML.load(ERB.new(File.read('config/database.yml')).result)
    ActiveRecord::Base.establish_connection(config[env])
    ActiveRecord::Base.logger = Logger.new('db/database.log')
  end

  desc "Migrate database"
  task :migrate => :environment do
    ActiveRecord::Migrator.migrate('db/migrate', ENV["VERSION"] ? ENV["VERSION"].to_i : nil )
  end

  desc "Create database"
  task :create do
    config = YAML.load(ERB.new(File.read('config/database.yml')).result)
    %x( createdb -E UTF8 -T template0 #{config[env]['database']})

    # prepare hstore
    if %x( createdb --version ).strip.gsub(/(.*)(\d\.\d\.\d)$/, "\\2") < "9.1.0"
      puts "Please prepare hstore data type. See http://www.postgresql.org/docs/current/static/hstore.html"
    end
  end

  desc "Drop database"
  task :drop do
    config = YAML.load(ERB.new(File.read('config/database.yml')).result)
    %x( dropdb #{config[env]['database']} )
  end
end
