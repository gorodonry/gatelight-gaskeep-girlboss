import itertools

PARTICLES = ["gas", "light", "gate", "keep", "girl", "boss"]

if __name__ == "__main__":
    all_pairs = [p for p in itertools.permutations(PARTICLES, 2)]

    all_combinations = [c for c in itertools.permutations(all_pairs, 3)]

    for c in all_combinations:
        if len(set([p for ps in c for p in ps])) == 6:
            print(f"{c[0][0]}{c[0][1]} {c[1][0]}{c[1][1]} {c[2][0]}{c[2][1]}")
