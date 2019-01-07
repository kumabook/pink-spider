FROM ruby:2.5.3

RUN curl -sL https://deb.nodesource.com/setup_10.x | bash

RUN apt-get update
RUN apt-get install -y build-essential nodejs
RUN apt-get install -y libpq-dev postgresql-client

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH $HOME/.cargo/bind

RUN npm install yarn --global

ENV INSTALL_PATH /app
RUN mkdir -p $INSTALL_PATH

WORKDIR $INSTALL_PATH

COPY Gemfile Gemfile
COPY Gemfile.lock Gemfile.lock
RUN gem install bundler && bundle install --deployment

COPY . .

RUN cargo build
RUN yarn


EXPOSE 8080
CMD ["cargo", "run", "--bin", "app"]
