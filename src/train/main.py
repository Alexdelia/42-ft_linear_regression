from pathlib import Path


def main():
    mileage = [10, 20, 30, 40, 50]
    prices = [5, 10, 15, 20, 25]

    theta0 = 0
    theta1 = 0

    learning_rate = 0.0001

    m = len(mileage)

    def estimate_price(mileage, theta0, theta1):
        return theta0 + (theta1 * mileage)

    for t in range(10000):
        tmp_theta0 = 0
        tmp_theta1 = 0
        for i in range(m):
            tmp_theta0 += estimate_price(mileage[i], theta0, theta1) - prices[i]
            tmp_theta1 += (
                estimate_price(mileage[i], theta0, theta1) - prices[i]
            ) * mileage[i]
        tmp_theta0 *= learning_rate / m
        tmp_theta1 *= learning_rate / m
        theta0 -= tmp_theta0
        theta1 -= tmp_theta1

        if t % 1000 == 0:
            print(f"""{t}:
theta0: {theta0}
theta1: {theta1}""")

            for i in range(m):
                print(
                    f"{mileage[i]}km\t{prices[i]}\t-> {round(estimate_price(mileage[i], theta0, theta1), 2)}"
                )
            print()

    print("Learned theta0:", theta0)
    print("Learned theta1:", theta1)

    output = Path(".cache/model.toml")

    output.parent.mkdir(parents=True, exist_ok=True)

    with output.open("w") as f:
        f.write(f"theta0 = {theta0}\n")
        f.write(f"theta1 = {theta1}\n")
