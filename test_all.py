import subprocess

from termcolor import colored

days = {
    "day-01-1": "55607",
    "day-01-2": "55291",
    "day-02-1": "2632",
    "day-02-2": "69629",
    "day-03-1": "536202",
    "day-03-2": "78272573",
    "day-04-1": "21088",
    "day-04-2": "6874754",
    "day-05-1": "178159714",
    "day-05-2": "0",
}


def test(name: str, expected: str) -> bool:
    result = subprocess.run(["cargo", "run", "--bin", name, "--release", "--quiet"], capture_output=True)
    return result.stdout.decode("utf-8").strip() == expected


def main():
    global days
    for (name, expected) in days.items():
        if test(name, expected):
            print(f"{colored('', 'green')} {name} passed")
        else:
            print(f"{colored('', 'red')} {name} failed")


if __name__ == "__main__":
    main()
