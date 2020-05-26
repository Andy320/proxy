#!/bin/sh

role=$1

echo "Start Redis as: "
if [ "$role" = "master" ]
then
    echo "master"
    redis-server /etc/redis.conf
elif [ "$role" = "slave" ]
then
    echo "slave"
    sed -i 's/# slaveof <masterip> <masterport>/slaveof redis-master 6379/g' /etc/redis.conf
    redis-server /etc/redis.conf
else
    echo "unknown role!"
fi

