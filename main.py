import random

def main():
    s = [f"{i}s" for i in range(1, 10)]
    m = [f"{i}m" for i in range(1, 10)]
    p = [f"{i}p" for i in range(1, 10)]
    j = ["to", "na", "sy", "pe", "hk", "ht", "ty"]
    mountain = s * 4 + m * 4 + p * 4 + j * 4
    haipai = random.sample(mountain, 13)
    haipai.sort(key=lambda x: (x[1:].isdigit(), x[1], x[0] if x[0].isdigit() else float('inf')))
    print(haipai)

if __name__ == "__main__":
    main()
