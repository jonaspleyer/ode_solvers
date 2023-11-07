#/usr/bin/env python3

import matplotlib.pyplot as plt
from matplotlib.collections import LineCollection
from matplotlib.colors import ListedColormap, BoundaryNorm
import matplotlib.transforms as mtransforms
from matplotlib.patches import FancyBboxPatch
import numpy as np
from scipy.integrate import odeint


def lorenz_rhs(y, t, p):
    X = y[0]
    Y = y[1]
    Z = y[2]

    dX = p[0] * (Y - X)
    dY = X * (p[1] - Z) - Y
    dZ = X * Y - p[2] * Z

    return [dX, dY, dZ]


def get_segments(x, y):
    points = np.array([x, y]).T.reshape(-1, 1, 2)
    segments = np.concatenate([points[:-1], points[1:]], axis=1)
    return segments


if __name__ == "__main__":
    # Definen initial values, parameters and time series
    y = np.array([1, 1, 10])
    p = np.array([10, 28, 8/3])

    t_series = np.linspace(0, 50, 10000)

    # Solve the ODE
    res = odeint(lorenz_rhs, y, t_series, args=(p,))

    # Transform the results such that they can be used in a linecollection
    x = res[:,0]
    y = res[:,2]
    dydx = t_series

    # Get segments from result
    segments = get_segments(x, y)

    # Plot with matplotlib
    fig = plt.figure(figsize=(8,8))
    ax = fig.add_axes((0.1, 0.1, 0.8, 0.8))

    # Create a continuous norm to map from data points to colors
    norm = plt.Normalize(dydx.min(), dydx.max())
    lc = LineCollection(segments, color="#fde725", alpha=0.4)
    
    # Set the values used for colormapping
    lc.set_array(dydx)
    lc.set_linewidth(3)
    
    line = ax.add_collection(lc)

    # Set dimensions of plot
    ax.set_xlim(x.min(), x.max())
    ax.set_ylim(y.min(), y.max())
    
    # Disable axis
    ax.axis('off')

    # Create the blue box under the plot
    bb = mtransforms.Bbox([[0, 0], [1, 1]])
    p_bbox = FancyBboxPatch((bb.xmin, bb.ymin),
        abs(bb.width), abs(bb.height),
        boxstyle="round,pad=0,rounding_size=0.1",
        ec="k", zorder=0., transform=fig.transFigure, fill=True, facecolor="#31688e"
    )
    ax.add_patch(p_bbox)
    
    # Save the figure to svg file
    fig.savefig("lorenz.svg", transparent=True)

    # Open file and delete lines responsible for wrongly clipping the image
    with open(r"lorenz.svg", 'r') as fp:
        lines = fp.readlines()

    with open(r"lorenz.svg", 'w') as fp:
        for i, line in enumerate(lines[:-6]):
            fp.write(line)
        fp.write("</svg>\n")