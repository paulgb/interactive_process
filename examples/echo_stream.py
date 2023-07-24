#!/usr/bin/env python3


def main():
    while True:
        line = input()
        if line == "exit":
            break
        print(f"echo: {line}", flush=True)


if __name__ == "__main__":
    main()
