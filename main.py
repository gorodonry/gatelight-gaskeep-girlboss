import itertools

PARTICLES = ["gas", "light", "gate", "keep", "girl", "boss"]

if __name__ == "__main__":
    all_pairs = [p for p in itertools.permutations(PARTICLES, 2)]

    all_combinations = [c for c in itertools.product(all_pairs, repeat=3)]

    for c in all_combinations:
        print(f"{c[0][0]}{c[0][1]} {c[1][0]}{c[1][1]} {c[2][0]}{c[2][1]}")
