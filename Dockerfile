FROM ruby:2.5.3

RUN apt-get update
RUN curl -sL https://deb.nodesource.com/setup_11.x | bash -
RUN apt-get install -y build-essential nodejs
RUN apt-get install -y libpq-dev postgresql-client

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH $PATH:/root/.cargo/bin
RUN rustup install 1.26.2
RUN rustup default 1.26.2

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
