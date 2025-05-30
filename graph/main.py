#!/usr/bin/env python3
import subprocess
import matplotlib.pyplot as plt
import numpy as np


def run_simulations_for_size(grid_size, densities, simulations, burn_pattern):
    """Run simulations for different densities and return average burned percentages"""
    executable = "../target/release/project_forest_fire.exe"
    averages = []

    for density in densities:
        cmd = [
            executable,
            "--graphics-off",
            "--quiet",
            "-d", str(density),
            "-c", str(simulations),
            "-s", str(grid_size),
            "-b", burn_pattern,
        ]

        result = subprocess.run(cmd, capture_output=True, text=True)
        if result.returncode != 0:
            print(f"Error for size {grid_size}, density {density}: {result.stderr}")
            averages.append(None)
            continue

        try:
            avg = float(result.stdout.strip())
            averages.append(avg)
        except ValueError:
            print(f"Parse error for size {grid_size}, density {density}: {result.stdout}")
            averages.append(None)

    return averages


def main():
    # Configuration

    # sizes = np.unique(np.round(np.geomspace(20, 10_000, num=15))).astype(int).tolist()
    # sizes = [20, 30, 40, 60, 80, 100]
    sizes = [20, 40, 80, 160, 320, 640, 1280] #, 2560, 5120, 10240 ] # , 20480, 40960, 81920, 163840]
    sizes = [ 1280 ] #, 2560, 5120, 10240] # only for close up

    # densities = [d / 100.0 for d in range(0, 101, 5)]  # From 0.1 to 0.95 in steps of 0.05  for general plot
    # densities = [d / 100.0 for d in range(30, 51, 1)]  # From 0.3 to 0.5 in steps of 0.01     for moore - close up plot
    densities = [d / 100.0 for d in range(0, 101, 1)]  # From 0.01 to 0.99 in steps of 0.01     for vonneumann - close up plot
    simulations = 100
    burn_pattern = "moore" # "vonneumann" # "moore" - default
    # 0.3 - 0.5 for moore
    # 0.5 - 0.7 for vonneumann

    # Run sims
    results = {}
    for size in sizes:
        print(f"Running simulations for grid size {size}...")
        results[size] = run_simulations_for_size(size, densities, simulations, burn_pattern)

    # Plotting
    plt.figure(figsize=(10, 6))

    for size, averages in results.items():
        # filter out failures
        x = [d for d, a in zip(densities, averages) if a is not None]
        y = [a for a in averages if a is not None]
        plt.plot(x, y, marker='o', markersize=3, linewidth=1, label=rf'${size}^2$') # for general plot
        # plt.plot(x, y, linewidth=1, label=rf'${size}^2$') # for the close up plot

    # ticks every 0.05 on x (i.e. 5% density) and every 5% on y
    # Ticks: major at 0.10/10%, minor at 0.05/5%

    # plt.xticks(np.arange(0, 1.0001, 0.10)) # for general plot
    # plt.xticks(np.arange(0.3, 0.5001, 0.01)) # for close up plot moore
    plt.xticks(np.arange(0, 1.0001, 0.01)) # for close up plot vonneumann
    plt.yticks(np.arange(0, 105, 10))
    plt.minorticks_on()
    plt.gca().xaxis.set_major_locator(plt.MultipleLocator(0.1)) # for close up plot
    # plt.gca().xaxis.set_minor_locator(plt.MultipleLocator(0.05)) # for general plot
    plt.gca().yaxis.set_minor_locator(plt.MultipleLocator(5))

    plt.grid(which='minor', linestyle=':', linewidth=0.5, color='gray')
    plt.grid(which='major', linestyle='-', linewidth=1, color='gray') # lightcoral

    # axes labels & ticks
    plt.xlabel('Tree Density (fraction)')
    plt.ylabel('Average Burned Trees (%)')
    plt.title('Impact of Tree Density and Forest Size on Burned Area')

    # grid, legend
    plt.grid(True)
    plt.legend(title="Grid Size", fontsize='small', ncol=2, frameon=True)

    # y-limit with a little extra top margin
    plt.ylim(0, 105)

    plt.tight_layout()
    plt.show()


if __name__ == "__main__":
    main()
