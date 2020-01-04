# Pink Spider

[![Build Status](https://travis-ci.org/kumabook/pink-spider.svg?branch=master)](https://travis-ci.org/kumabook/pink-spider)

[![Deploy](https://www.herokucdn.com/deploy/button.png)](https://heroku.com/deploy)

<img height="260" src="public/no_image.png">

## What's this?

Pink Spider is a music spider. Currently crawl YouTube and SoundCloud api
with request of scraping track from web page.


## Development

1. Install [asdf][] and run `asdf install`
2. Install [yarn][] and [bundler][]
3. Install dependencies

    ```shell
    cargo install
    yarn install
    bundle install
    ```

4. Install posgresql and setup database

    - Create "postgres" role (password: "pink_spider"):

        ```shell
        createuser -d -U your_name -P pink_spider -s`
        ```

    - Create database

        ```shell
        rake db:create
        rake db:migrate
        ```

5. Build and run backend

    ```shell
    cargo run --bin app
    ```

6. Build frontend

    ```shell
    npm start # on another shell
    ```

## Development on Docker

- Install `docker` and `docker-compose` and `docker-machine`
  - `brew install docker docker-compose docker-machine`
- Create container and prepare db
  - `docker-compose up`
  - `docker-compose run --rm web rake db:create`
  - `docker-compose run --rm web rake db:migrate`
- Restore database from backup
  - `cat latest.dump | docker exec -i `docker-compose ps -q db` pg_restore --verbose --clean -U postgres -d pink_spider_production`

## Testing

```shell
    cargo test
    npm run lint
    npm test
```

## Deploy on heroku

```shell
    heroku apps:create pink-spider
    heroku buildpacks:set https://github.com/emk/heroku-buildpack-rust.git
    heroku buildpacks:set heroku/ruby
    heroku buildpacks:set heroku/nodejs
    heroku addons:create heroku-postgresql:hobby-dev
    git push heroku master
    heroku run rake db:migrate
```

[asdf]:    https://github.com/asdf-vm/asdf
[yarn]:    https://yarnpkg.com/
[bundler]: http://bundler.io/
