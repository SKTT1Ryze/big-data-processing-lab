#!/bin/bash
# Shell for record top
sleep_sec=$1
print_times=$2
log_file=$3

top -d $sleep_sec -n $print_times -b > $log_file