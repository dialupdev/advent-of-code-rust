if [ -z "$1" ]; then
  echo "Error: Please provide year (e.g. make 2022 01)"
  exit 1
fi

if [ -z "$2" ]; then
  echo "Error: Please provide day (e.g. make 2022 01)"
  exit 1
fi

cargo run --package "advent-of-code-$1" --bin "$2"
