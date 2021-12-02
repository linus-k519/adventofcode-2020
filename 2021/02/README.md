# Advent of Code 2021 Day 2 in PHP

Run PHP script in docker container. Adjust the file name to `...task1.php` or `...task2.php` corresponding to the program you want to run.

```
docker run --rm -it --user="$(id -u):$(id -g)" -w /usr/src/app -v "$PWD":/usr/src/app php:7.4-cli php advent_2021_02_task[1/2].php
```
