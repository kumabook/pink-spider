FROM rust:1.21.0
RUN curl -sL https://deb.nodesource.com/setup_6.x | bash
RUN apt-get update && \
    apt-get install -qq -y build-essential nodejs ruby ruby-dev libpq-dev postgresql-client --fix-missing --no-install-recommends

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
