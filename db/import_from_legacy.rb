require 'uri'
require 'pg'

namespace :db do
  def get_connection
    url = URI.parse(ENV['DATABASE_URL'])
    PG::connect(host: url.host,
                user: url.user,
                password: url.password,
                dbname: url.path[1..-1])
  end

  def create_entry_if_not_exist(connection, entry)
    maybe_entry = connection.exec("SELECT id, url " +
                                  "FROM entries WHERE url = '#{entry['url']}'")
    entry_id = nil
    if maybe_entry.count == 0
      entry_id = SecureRandom::uuid()
      connection.exec("INSERT INTO entries (id, url) " +
                      "VALUES ('#{entry_id}', '#{entry['url']}')")
      puts "entry: url as #{entry['id']}"
    else
      entry_id = maybe_entry.first['id']
    end
    { 'id' =>  entry_id, 'url' =>  entry['url'] }
  end

  def create_track_if_not_exist(connection, track, entry_id)
    maybe_track = connection.exec("SELECT id, provider, title, url, identifier " +
                                  "FROM tracks WHERE provider = '#{track['provider']}' " +
                                  "AND identifier = '#{track['identifier']}'")
    track_id = nil
    if maybe_track.count == 0
      track_id = SecureRandom::uuid()
      puts "track: #{track_id['id']} -> #{track_id}"
      connection.prepare("insert-#{track_id}",
                         "INSERT INTO tracks (id, provider, title, url, identifier) " +
                         "VALUES ($1, $2, $3, $4, $5) RETURNING id")
      connection.exec_prepared("insert-#{track_id}",
                               [track_id,
                                track['provider'],
                                track['title'],
                                track['url'],
                                track['identifier']])
    else
      track_id = maybe_track.first['id']
    end
    track['id'] = track_id
    track
  end

  def create_track_entry_if_not_exist(connection, track_id, entry_id)
    maybe_track_entry = connection.exec("SELECT track_id, entry_id FROM track_entries" +
                                        " WHERE track_id = '#{track_id}' AND" +
                                        " entry_id = '#{entry_id}'")
    if maybe_track_entry.count == 0
      connection.exec("INSERT INTO track_entries (track_id, entry_id) " +
                      "VALUES ('#{track_id}', '#{entry_id}')");
      puts "relation entry #{entry_id} <-> track #{track_id}"
    end
  end

  task :import_from_legacy do
    connection = get_connection
    entries = connection.exec("SELECT id, url FROM entry")
    entries.each do |e|
      entry = create_entry_if_not_exist(connection, e)
      tracks = connection.exec("SELECT t.id, t.provider, t.title, t.url, t.identifier " +
                               "FROM track t LEFT JOIN track_entry te
                                 ON t.id = te.track_id
                                 WHERE te.entry_id = " + e['id'])
      tracks.each do |t|
        track = create_track_if_not_exist(connection, t, entry['id'])
        create_track_entry_if_not_exist(connection, track['id'], entry['id'])
      end
    end
    connection.exec("DROP TABLE IF EXISTS track_entry")
    connection.exec("DROP TABLE IF EXISTS track")
    connection.exec("DROP TABLE IF EXISTS entry")
  end
end
