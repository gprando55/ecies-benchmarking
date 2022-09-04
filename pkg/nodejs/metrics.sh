# !/bin/bash

for fileName in 1kb 10kb 100kb 1mb 10mb; do
  echo "$fileName"
  node main.js --file $fileName > node-$fileName.txt
done 