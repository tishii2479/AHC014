import subprocess

using_testcase_num = 200
total_testcase_num = 10000
n_trials = 400


def f(args):
    i, a, b, c = args
    cmd = "target/release/ahc014 {} {} {} < tools/in/{:04}.txt".format(a, b, c, i)
    res = subprocess.run(
        [cmd], universal_newlines=True, shell=True, stdout=subprocess.PIPE
    )
    # print(cmd, res.stdout, end="")
    return float(res.stdout)
