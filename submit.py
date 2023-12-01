#!/usr/bin/env python3

import argparse
import json
import os
import requests
import subprocess
import sys

from word2number import w2n

YEAR = 2023

def submit_answer(day: int, part: int, answer: str, dry_run: bool) -> str:
    with open(os.path.join(os.getenv('HOME'), '.config/aocd/token')) as f:
        session_key = f.read().strip()

    url = f'https://adventofcode.com/{YEAR}/day/{day}/answer'
    headers = {'cookie': f'session={session_key}'}
    body = {"answer": answer, "level": part}

    if dry_run:
        print(f"POST {url} {body}")
        return ""
    else:
        return requests.post(url, headers=headers, data=body).text

ALREADY_COMPLETE = "You don't seem to be solving the right level."
SUCCESS = "That's the right answer!"
FAIL = "That's not the right answer."
FAIL_HIGH = "That's not the right answer; your answer is too high."
FAIL_LOW = "That's not the right answer; your answer is too low."

MSGS = [ALREADY_COMPLETE, SUCCESS, FAIL, FAIL_HIGH, FAIL_LOW]

def submit_question(name: str, day: int, part: int, dry_run: bool) -> int:
    p = subprocess.Popen(
        ['cargo', 'run', '--bin', name, '--', '--json'],
        stdin=subprocess.PIPE,
        stdout=subprocess.PIPE
    )

    answers = {}
    with open(os.path.join(name, 'input.txt')) as f:
        stdout, stderr = p.communicate(input=f.read().encode())
        for line in stdout.splitlines():
            b = json.loads(line)
            answers[b['part']] = b['answer']

    if p.returncode != 0:
        return p.returncode

    reply = submit_answer(day, part, answers[part], dry_run)
    for m in MSGS:
        if m in reply:
            print("remote:", m)
            break
    else:
        print(reply)

def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("name")
    parser.add_argument("--part", type=int, required=True)
    parser.add_argument("--day", type=int, required=False)
    parser.add_argument("--dry-run", action='store_true', required=False)
    args = parser.parse_args()
    return submit_question(
        args.name,
        args.day or w2n.word_to_num(args.name),
        args.part,
        args.dry_run
    )


if __name__ == "__main__":
    sys.exit(main())
