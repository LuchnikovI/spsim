import os
from math import pi
import numpy as np
import matplotlib.pyplot as plt
from spsim import SPSim

# ---------------------------------------------------
theta = 0.3
time_steps = 40
threshold = 1e-5
decay = 1.
# ---------------------------------------------------

dirpath = os.path.dirname(os.path.realpath(__file__))

def get_diamond_sim(size: int, mixing_time: float) -> SPSim:
    if size % 2 != 0:
        raise ValueError("Size must be multiple of 2")
    qubits_number = size * size * size
    sim = SPSim(qubits_number)
    for i in range(qubits_number):
        sim.add_gate([('X', i)], -mixing_time / 2)
    for z in range(size):
        z_shift = size * size * z
        for y in range(size):
            y_shift = y * size
            for x in range(size):
                sim.add_gate([('Z', x + z_shift + y_shift), ('Z', z_shift + y_shift + (x + 1) % size)], pi / 4)
                if (x + y + z) % 2 == 0:
                    sim.add_gate([
                        ('Z', x + z_shift + y_shift),
                        ('Z', x + z_shift + ((y + 1) % size) * size),
                    ], pi / 4)
                    sim.add_gate([
                        ('Z', x + z_shift + y_shift),
                        ('Z', x + y_shift + ((z + 1) % size) * size * size),
                    ], pi / 4)
    return sim

sim = get_diamond_sim(10, theta)

dyn = list(map(lambda x: x.real, sim.execute([('Z', 500)], time_steps, threshold, decay)))


np.save(f"{dirpath}/sp_dynamics.npy", np.array(dyn))
plt.plot(dyn, 'b')
plt.yscale('log')
plt.xlabel("Time step")
plt.ylabel("<Z>")
plt.savefig(f"{dirpath}/diamond.pdf")