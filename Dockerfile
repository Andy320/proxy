FROM alpine:3.11
RUN apk --no-cache add ca-certificates

ADD output/router /usr/local/bin/
ADD log4rs.yaml /etc/router/

WORKDIR /
