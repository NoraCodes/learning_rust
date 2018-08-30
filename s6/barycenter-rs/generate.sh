#!/bin/bash
randfloat () {
    printf "%d04.%d04\n" $RANDOM $RANDOM
}

echo "use super::Body;"
echo "pub fn get_values() -> Vec<Body> {"
printf "\tvec![\n"
for i in $(seq 1 $1); do
    printf "\t\tBody::new(%s,%s,%s,%s),\n" $(randfloat) $(randfloat) $(randfloat) $(randfloat)
done
printf "\t]\n}\n"