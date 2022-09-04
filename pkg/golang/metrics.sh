# !/bin/bash

for fileName in 1kb 10kb 100kb 1mb 10mb; do
  echo "$fileName"
  go run main.go --file 1kb $fileName > go-$fileName.txt
done 