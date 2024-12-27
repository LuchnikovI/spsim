import os
from math import pi
import numpy as np
import matplotlib.pyplot as plt
from spsim import SPSim

# ---------------------------------------------------
theta = pi / 2 - 0.6
time_steps = 40
threshold = 1e-6
decay = 2.
# ---------------------------------------------------

dirpath = os.path.dirname(os.path.realpath(__file__))

def get_hex_sim(size: int, mixing_time: float) -> SPSim:
    if size % 2 != 0:
        raise ValueError("Size must be multiple of 2")
    qubits_number = size * size
    sim = SPSim(qubits_number)
    for i in range(qubits_number):
        sim.add_gate([('X', i)], -mixing_time / 2)
    for y in range(size):
        shift = y * size
        for x in range(size):
            sim.add_gate([('Z', x + shift), ('Z', shift + (x + 1) % size)], pi / 4)
            if (x + y) % 2 == 0:
                sim.add_gate([('Z', x + shift), ('Z', x + ((y + 1) % size) * size)], pi / 4)
    return sim

sim = get_hex_sim(20, theta)

dyn = list(map(lambda x: x.real, sim.execute([('Z', 200)], time_steps, threshold, decay)))


np.save(f"{dirpath}/sp_dynamics.npy", np.array(dyn))
plt.plot(dyn, 'b')
plt.yscale('log')
plt.xlabel("Time step")
plt.ylabel("<Z>")
plt.savefig(f"{dirpath}/hex.pdf")