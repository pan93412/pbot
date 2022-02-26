#!/usr/bin/env python3

import itertools, subprocess
from typing import Iterable

available_modules = ["fwdmod", "getinfomod", "addrankmod"]


def cargo_check(modules: Iterable[str]) -> bool:
    cmd = [
        "cargo", "check", "--no-default-features", "--features",
        " ".join(modules)
    ]
    result = subprocess.run(cmd)

    return result.returncode == 0


for i in range(0, len(available_modules)):
    for combination in itertools.combinations(available_modules, i + 1):
        human_readable_combination = ", ".join(combination)

        print(f"\x1b[1m-----\n"
              "Testing combination: {human_readable_combination}\n"
              "-----\x1b[0m")

        if cargo_check(combination):
            print(f"\x1b[1;32mSUCCESS: {human_readable_combination}\x1b[0m")
        else:
            print(f"\x1b[1;31mFAILED: {human_readable_combination}\x1b[0m")
            exit(1)
