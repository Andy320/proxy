FROM registry.cn-beijing.aliyuncs.com/andy320/dev-alpine3.11:v1.0.0

RUN apk add redis
RUN echo 'vm.overcommit_memory = 1' >> /etc/sysctl.conf

ADD redis.conf /etc/
ADD start_redis.sh /usr/local/bin/

EXPOSE 6379

CMD ["master"]

ENTRYPOINT ["sh","/usr/local/bin/start_redis.sh"]