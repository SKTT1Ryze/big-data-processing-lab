#!/bin/bash
# Shell for observe hdfs

while [ 1 ]
do
    hadoop fs -la /input
    echo ""
    sleep 1s
done

