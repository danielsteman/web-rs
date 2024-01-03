#!/bin/bash

pandoc -s articles/blog11.md -o articles/blog11.html --standalone --highlight-style=pygments
awk '/<body>/,/<\/body>/ {print}' articles/blog11.html >articles/blog11_no_head.html
awk 'BEGIN { inStyle=0 } /<style>/ { inStyle=1 } inStyle { print } /<\/style>/ { inStyle=0 }' articles/blog11.html >>articles/blog11_no_head.html
