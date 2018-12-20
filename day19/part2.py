r0 = 0
r4 = 10551293
r3 = 0
r2 = 1
r1 = int(r4 / r2)
while True:
    if r2 * r1 == r4:
        r0 += r2
    r1 += 1
    if r1 > r4 or r2 * r1 > r4:
        r2 += 1
        if r2 > r4:
            break
        else:
            r1 = int(r4 / r2)

print(r0, r1, r2, r3, r4)
