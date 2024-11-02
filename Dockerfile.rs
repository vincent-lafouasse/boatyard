FROM alpine:latest

RUN apk update && apk upgrade \
    && apk add bash bash-doc bash-completion \
    && apk add vim \
    && apk add build-base git \
    && apk add curl \
    && curl https://sh.rustup.rs -sSf | sh -s -- -y \
    && echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
