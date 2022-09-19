rm -r out/
mkdir out

cargo run < tools/in/0011.txt

python3 visualize_annealing.py

# rm -r out/
