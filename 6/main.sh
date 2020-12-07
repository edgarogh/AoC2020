#!/bin/bash

current_group=""

(
while read line
do
    if [ ! $line = "" ]
    then
        current_group=$current_group$line
    else
        questions=$(echo -n $current_group | grep -o . | sort -u | wc -l)
        echo $questions
        current_group=""
    fi
done
) < input.txt | paste -sd+ | bc
