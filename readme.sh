#!/bin/sh

cargo install --path .

echo -n > README.md
echo '# advent-of-code' >> README.md
echo >> README.md
echo '```' >> README.md
./target/release/advent-of-code >> README.md
echo '```' >> README.md
