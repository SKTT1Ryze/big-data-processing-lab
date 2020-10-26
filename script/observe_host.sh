#!/bin/bash
# Shell for observe /home/hadoop/tmp/dfs/name/, /home/hadoop/tmp/dfs/name/current, /home/hadoop/tmp/dfs/namesecondary/ and /home/hadoop/tmp/dfs/namesecondary/current

while [ 1 ]
do
    ls /home/hadoop/tmp/dfs/name/
    ls -lht /home/hadoop/tmp/dfs/name/current/
    ls /home/hadoop/tmp/dfs/namesecondary/
    ls -lht /home/hadoop/tmp/dfs/namesecondary/current/
    echo ""
    sleep 1s
done
