# coding: utf-8
require 'active_record'
require 'yaml'
require 'erb'
require 'logger'
include ActiveRecord::Tasks

namespace :db do
  env = ENV['ENV'] || 'development'

  def db_config(env)
    if ENV['DATABASE_URL'].nil?
      return YAML.load(ERB.new(File.read('config/database.yml')).result)
    end
    clazz = ActiveRecord::ConnectionAdapters::ConnectionSpecification::ConnectionUrlResolver
    config =  clazz.new(ENV['DATABASE_URL']).to_hash
    { env => config }
  end

  task :default => :migrate

  task :environment do
    config = YAML.load(ERB.new(File.read('config/database.yml')).result)
    ActiveRecord::Base.establish_connection(ENV['DATABASE_URL'] || config[env])
    ActiveRecord::Base.logger = Logger.new('db/database.log')
    ActiveRecord::Base.connection.tables.each do |table_name|
      class_name = table_name.singularize.camelcase
      self.class.const_set class_name, Class.new(ActiveRecord::Base)
    end
  end

  desc "Migrate database"
  task :migrate => :environment do
    ActiveRecord::Migrator.migrate('db/migrate', ENV["VERSION"] ? ENV["VERSION"].to_i : nil )
  end

  desc "Create database"
  task :create do
    ActiveRecord::Base.configurations = db_config(env)
    DatabaseTasks.db_dir = 'db'
    DatabaseTasks.create_current(env);
  end

  desc "Drop database"
  task :drop => :environment do
    ActiveRecord::Base.configurations = db_config(env)
    DatabaseTasks.db_dir = 'db'
    DatabaseTasks.create_current(env);
  end

  desc "normalize track"
  task :normalize_tracks => :environment do
    Track.find_each do |track|
      track.identifier.match(/[a-zA-Z0-9\-\_]+/) do |md|
        if md[0] != track.identifier
          puts "#{track.provider} id: #{track.identifier} -> #{md[0]}"
          track.update! identifier: md[0]
        end
      end
    end
  end
end
