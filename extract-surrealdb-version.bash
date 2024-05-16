#!/bin/bash

VERSION="$( cargo metadata --format-version=1 --no-deps | jq -c '.packages[0].dependencies[] | select( .name == "surrealdb" ) | .req | match("^\\^?(.*)"; "i") | .captures[0].string' | sed 's:^.\(.*\).$:\1:' )"

echo -n $VERSION > src/surrealdb-version
