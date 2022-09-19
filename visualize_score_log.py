import matplotlib.pyplot as plt

score_log_file = "tools/out/score_log.txt"

scores = []

with open(score_log_file, "r") as f:
    for line in f:
        scores.append(float(line))

plt.plot(scores)
plt.show()
plt.savefig("tools/out/score_log.png")
