import multiprocessing
import pipes
import subprocess

CASE = 100
TL = 6.0


def execute_case(seed):
    input_file_path = f"tools/in/{seed:04}.txt"
    output_file_path = f"tools/out/{seed:04}.txt"
    with open(input_file_path) as fin:
        with open(output_file_path, "w") as fout:
            subprocess.run(
                ["target/release/ahc014"], stdin=fin, stdout=fout, timeout=TL
            )
            pipefile = f"tools/out/pipefile_{seed:04}"
            with pipes.Template().open(pipefile, "w") as p:
                subprocess.run(
                    ["tools/target/release/vis", input_file_path, output_file_path],
                    stdout=p,
                    timeout=TL,
                )
            output = open(pipefile).read()
            assert output
    return seed, output


def main():
    scores = []
    count = 0
    total = 0
    subprocess.run("cargo build --release", shell=True)
    with multiprocessing.Pool(max(1, multiprocessing.cpu_count() - 2)) as pool:
        for seed, score in pool.imap_unordered(execute_case, range(CASE)):
            if count > 0 and count % 10 == 0:
                print(f"case: {count}, current ave: {total / count}", flush=True)
            try:
                scores.append((int(score.split()[2]), f"{seed:04}"))
                total += scores[-1][0]
            except ValueError:
                print(seed, "ValueError", flush=True)
                print(score, flush=True)
                exit()
            except IndexError:
                print(seed, "IndexError", flush=True)
                print(f"error: {score}", flush=True)
                exit()
            count += 1
    print()
    scores.sort()
    ave = total / CASE
    print(f"total: {total}")
    print(f"max: {scores[-1]}")
    print(f"ave: {ave}")
    print(f"min: {scores[0]}")

    div = 30
    cnt = [0] * div
    base = 500000
    step = 50000
    for s in scores:
        cnt[(s[0] - base) // step] += 1

    for i in range(div):
        score = base + i * step
        print(f"{score:7} ~ {score + step - 1:7}: " + "o" * (cnt[i] * 100 // CASE))


if __name__ == "__main__":
    main()
