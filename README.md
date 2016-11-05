# Pink Spider

[![Build Status](https://travis-ci.org/kumabook/pink-spider.svg?branch=master)](https://travis-ci.org/kumabook/pink-spider)

[![Deploy](https://www.herokucdn.com/deploy/button.png)](https://heroku.com/deploy)

## How to build

```shell
    cargo build
```

##

```shell
    heroku buildpacks:set https://github.com/emk/heroku-buildpack-rust.git
    heroku buildpacks:set heroku/ruby
```
