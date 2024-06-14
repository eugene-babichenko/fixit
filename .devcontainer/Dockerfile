FROM alpine:latest
RUN apk add git cargo go npm shellcheck \
    && go install github.com/evilmartians/lefthook@latest \
    && npm install -g prettier
ENV PATH=${PATH}:/root/go/bin
