#!/bin/bash

result=0

while IFS= read -r line || [ -n "$line" ]
do
  digits=$(echo -n "${line}" | tr -d -c 0-9)
  first=$(echo -n "${digits}" | head -c 1)
  last=$(echo -n "${digits}" | tail -c 1)

  result=$((result + "$first$last"))
done < "${1}"

echo $result
