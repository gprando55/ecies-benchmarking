# !/bin/bash

for fileName in 1kb 10kb 100kb 1mb 10mb; do
  echo "$fileName"
  python3 main.py  --file $fileName > python-$fileName.txt
done 