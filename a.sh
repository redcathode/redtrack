#!/bin/bash

# Array of fieldnames
declare -a fieldnames=("psychomotor" "mood")

# Array of fieldtypes
declare -a fieldtypes=("rating_numerical" "rating_numerical")

# Array of fieldvals
declare -a fieldvals=("-5" "0" "-2.5" "2.5" "4")

# Loop through arrays
for i in "${!fieldnames[@]}"; do
  curl -X POST -H "Content-Type: application/json" \
  -d '[{
    "fieldname": "'"${fieldnames[$i]}"'",
    "fieldtype": "'"${fieldtypes[$i]}"'",
    "fieldval": "'"${fieldvals[$i]}"'"
  }]' \
  http://localhost:3000/v1/posts/submit
done