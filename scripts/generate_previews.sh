#!/bin/bash

convert $1 -scale 200x125 -background white -gravity center -extent 200x125 p_small_$1
convert $1 -scale 400x200 -background white -gravity center -extent 400x200 p_large_$1
