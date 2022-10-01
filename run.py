import multiprocessing
import pipes
import subprocess

CASE = 100
TL = 6.0


def execute_case(seed):
    input_file_path = f"tools/in/{seed:04}.txt"
    output_file_path = f"tools/out/{seed:04}.txt"
    with open(input_file_path) as f:
        n, m = map(int, f.readline().split())

    with open(input_file_path) as fin:
        with open(output_file_path, "w") as fout:
            subprocess.run(
                ["target/release/ahc014"],
                stdin=fin,
                stdout=fout,
                stderr=subprocess.PIPE,
                timeout=TL,
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
    return seed, output, n, m


def main():
    scores = []
    count = 0
    total = 0

    div_scores = [[0] * 10 for _ in range(6)]
    div_counts = [[0] * 10 for _ in range(6)]

    subprocess.run("cargo build --release", shell=True)
    with multiprocessing.Pool(max(1, multiprocessing.cpu_count() - 2)) as pool:
        for seed, score, n, m in pool.imap_unordered(execute_case, range(CASE)):
            count += 1
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

            print(
                f"case {seed:3}: (score: {scores[-1][0]:7}, current ave: {total / count:10.2f})",
                flush=True,
            )

            div_scores[min(5, (n - 31) // 5)][(m - 30) // 30] += scores[-1][0]
            div_counts[min(5, (n - 31) // 5)][(m - 30) // 30] += 1

    print()
    scores.sort()
    ave = total / CASE
    print(f"total: {total}")
    for i in range(10):
        print(f"{scores[-(i+1)]}")
    for i in range(10):
        print(f"{scores[i]}")
    print(f"ave: {ave}")

    div = 30
    cnt = [0] * div
    base = 500000
    step = 50000
    for s in scores:
        cnt[min(len(cnt) - 1, (s[0] - base) // step)] += 1

    for i in range(div):
        score = base + i * step
        print(f"{score:7} ~ {score + step - 1:7}: " + "o" * (cnt[i] * 100 // CASE))

    print(" " * 5 + " |", end=" ")
    for j in range(10):
        print(f"{j * 30 + 30}~{(j+1) * 30 + 30 - 1}".rjust(7, " "), end=" ")
    print()
    print("-" * (10 * 8 + 7))

    for i in range(6):
        print(f"{i * 5 + 31}~{(i+1) * 5 + 31 - 1} |", end=" ")
        for j in range(10):
            if div_counts[i][j] != 0:
                print(f"{div_scores[i][j] // div_counts[i][j]:7}", end=" ")
            else:
                print(f"    nan", end=" ")
        print()


if __name__ == "__main__":
    main()
