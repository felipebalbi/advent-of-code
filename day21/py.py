from collections import deque

import numpy as np


def zxcv(fname, n):
    with open(fname) as f:
        dat1 = f.read().strip().split("\n")

    if n > 64:
        data = []
        for i in range(5):
            for line in dat1:
                data.append(5 * line.replace("S", "."))
    else:
        data = dat1

    width = len(data[0])
    height = len(data)

    q = deque()

    sx, sy = width // 2, height // 2

    q.append((sx, sy, 0))
    s64 = set()
    visited = set()

    while q:
        x, y, steps = q.popleft()
        if (x, y, steps) in visited:
            continue
        visited.add((x, y, steps))
        if steps == n:
            s64.add((x, y))
        else:
            if x >= 0:
                if data[y][x - 1] != "#":
                    q.append((x - 1, y, steps + 1))
            if x < width - 1:
                if data[y][x + 1] != "#":
                    q.append((x + 1, y, steps + 1))
            if y >= 0:
                if data[y - 1][x] != "#":
                    q.append((x, y - 1, steps + 1))
            if y < height - 1:
                if data[y + 1][x] != "#":
                    q.append((x, y + 1, steps + 1))

    if n == 64:
        for y, line in enumerate(data):
            ll = list(line)
            for sx, sy in s64:
                if sy == y:
                    ll[sx] = "O"
            print("".join(ll))

    return len(s64)


# polynomial extrapolation
a0 = zxcv("input.txt", 65)
a1 = zxcv("input.txt", 65 + 131)
a2 = zxcv("input.txt", 65 + 2 * 131)

print(a0);
print(a1);
print(a2);

vandermonde = np.matrix([[0, 0, 1], [1, 1, 1], [4, 2, 1]])
b = np.array([a0, a1, a2])
x = np.linalg.solve(vandermonde, b).astype(np.int64)

# note that 26501365 = 202300 * 131 + 65 where 131 is the dimension of the grid
n = 202300
print("part 2:", x[0] * n * n + x[1] * n + x[2])
