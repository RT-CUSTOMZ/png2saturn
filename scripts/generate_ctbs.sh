#!/bin/bash


#Bottom side
if [ "$1" != '' ]
then
    echo "Generating bottom layer ctb for $1..."
    b_png=${1%.*}.png
    b_ctb=${1%.*}.ctb
    gerbv -D 508 -B 0 -b "#FFFFFF" -f "#000000FF" -x png -o $b_png $1

    convert $b_png -scale 200x125 -background white -gravity center -extent 200x125 p_small_$b_png
    convert $b_png -scale 400x300 -background white -gravity center -extent 400x300 p_large_$b_png

    png2saturn -x 50 -y 30 -c south-west -e 100 -s p_small_$b_png -l p_large_$b_png $b_png $b_ctb 
else
    echo "Missing input filename(s)"
fi

#Top side
if [ "$2" != '' ]
then
    echo "Generating top layer ctb for $2..."
    f_png=${2%.*}.png
    f_ctb=${2%.*}.ctb
    gerbv -D 508 -B 0 -b "#FFFFFF" -f "#000000FF" -m Y -x png -o $f_png $2

    convert $f_png -scale 200x125 -background white -gravity center -extent 200x125 p_small_$f_png
    convert $f_png -scale 400x300 -background white -gravity center -extent 400x300 p_large_$f_png

    png2saturn -x 50 -y 30 -c south-east -e 100 -s p_small_$f_png -l p_large_$f_png $f_png $f_ctb 
fi
