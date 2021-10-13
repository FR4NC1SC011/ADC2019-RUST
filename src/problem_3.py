#!/usr/bin/python3
import sys

with open(sys.argv[1], 'r') as f:
    wires = [x.strip().split(',') for x in f.readlines()]

print(wires)
