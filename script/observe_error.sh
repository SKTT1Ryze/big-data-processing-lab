#!/bin/bash
# Shell for observe DataNode error

while [ 1 ]
do
    echo "master"
    tree /home/hadoop/
    echo "slave-1"
    ssh root@cluster-slave-1 "tree /home/hadoop/"
    echo "slave-2"
    ssh root@cluster-slave-2 "tree /home/hadoop/"
    echo "slave-3"
    ssh root@cluster-slave-3 "tree /home/hadoop/"
    echo "hdfs"
    hadoop fs -la /input
    sleep 2s
    echo ""
done
