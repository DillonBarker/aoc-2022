#!/bin/sh

echo "Enter day: "  
read input

cp ./templates/template_day.rs ./src/days/$input.rs

