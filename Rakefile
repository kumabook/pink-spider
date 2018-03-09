# coding: utf-8
require 'irb'
require 'active_record'
require 'yaml'
require 'erb'
require 'logger'
include ActiveRecord::Tasks

task :default => :console

task :console => "db:environment" do
  ARGV.clear
  IRB.start
end

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
    DatabaseTasks.drop_current(env);
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

  desc "clear partial models"
  task :clear_partial_models => :environment do
    Track.where(title: "").find_in_batches do |tracks|
      track_ids = tracks.map {|t| t.id }
      TrackArtist.where(track_id: track_ids).delete_all
      PlaylistTrack.where(track_id: track_ids).delete_all
      AlbumTrack.where(track_id: track_ids).delete_all
      TrackEntry.where(track_id: track_ids).delete_all
    end
    Album.where(title: "").find_in_batches do |albums|
      album_ids = albums.map {|a| a.id }
      AlbumArtist.where(album_id: album_ids).delete_all
      AlbumTrack.where(album_id: album_ids).delete_all
      AlbumEntry.where(album_id: album_ids).delete_all
    end
    Playlist.where(title: "").find_in_batches do |playlists|
      playlist_ids = playlists.map {|p| p.id }
      PlaylistTrack.where(playlist_id: playlist_ids).delete_all
      PlaylistEntry.where(playlist_id: playlist_ids).delete_all
    end
  end

  desc "clear artists"
  task :clear_artists => :environment do
    items = Artist.where(provider: 'AppleMusic').select do |artist|
      artist.identifier == artist.name
    end

    artist_ids = items.map {|i| i.id }

    TrackArtist.where(artist_id: artist_ids).delete_all
    AlbumArtist.where(artist_id: artist_ids).delete_all
    Artist.where(id: artist_ids).delete_all
    puts "clear apple music artists"
  end

  desc "normalize artists"
  task :normalize_artists => :environment do
    count = 0
    TrackArtist.find_in_batches do |track_artists|
      wrong_items = track_artists.select do |track_artist|
        e = Track.find_by(id: track_artist.track_id).nil?
      end
      TrackArtist.where(id: wrong_items.map {|i| i.id }).delete_all
      count += wrong_items.count
      puts "track #{count}"
    end
    puts "clear invalid track_artist #{count}"
    count = 0
    AlbumArtist.find_in_batches do |album_artists|
      wrong_items = album_artists.select do |album_artist|
        Album.find_by(id: album_artist.album_id).nil?
      end
      AlbumArtist.where(id: wrong_items.map {|i| i.id }).delete_all
      count += wrong_items.count
      puts "album #{count}"
    end
    puts "clear invalid album_artist #{count}"
  end
end
