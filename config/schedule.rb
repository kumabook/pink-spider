DIR = "/var/app/current/pink-spider"

every :hour do
  runner "rsscrawler"
  command "cd #{DIR} && source .envrc && ./target/release/rss_crawler"
end

every 3.hours do
  runner "playlistcrawler"
  command "cd #{DIR} && source .envrc && ./target/release/playlist_crawler"
end
