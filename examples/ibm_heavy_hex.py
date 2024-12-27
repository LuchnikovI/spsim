import os
from math import pi
import matplotlib.pyplot as plt
from spsim import SPSim

# ---------------------------------------------------
theta = 0.9
time_steps = 20
threshold = 1e-4
decay = 1.
# ---------------------------------------------------

dirpath = os.path.dirname(os.path.realpath(__file__))

sim = SPSim(127)
for i in range(127):
    sim.add_gate([('X', i)], -theta / 2)
for i in range(13):
    sim.add_gate([('Z', i), ('Z', i + 1)], pi / 4)
for i in range(18, 32):
    sim.add_gate([('Z', i), ('Z', i + 1)], pi / 4)
for i in range(37, 51):
    sim.add_gate([('Z', i), ('Z', i + 1)], pi / 4)
for i in range(56, 70):
    sim.add_gate([('Z', i), ('Z', i + 1)], pi / 4)
for i in range(75, 89):
    sim.add_gate([('Z', i), ('Z', i + 1)], pi / 4)
for i in range(94, 108):
    sim.add_gate([('Z', i), ('Z', i + 1)], pi / 4)
for i in range(113, 126):
    sim.add_gate([('Z', i), ('Z', i + 1)], pi / 4)
sim.add_gate([('Z', 0), ('Z', 14)], pi / 4)
sim.add_gate([('Z', 14), ('Z', 18)], pi / 4)
sim.add_gate([('Z', 4), ('Z', 15)], pi / 4)
sim.add_gate([('Z', 15), ('Z', 22)], pi / 4)
sim.add_gate([('Z', 8), ('Z', 16)], pi / 4)
sim.add_gate([('Z', 16), ('Z', 26)], pi / 4)
sim.add_gate([('Z', 12), ('Z', 17)], pi / 4)
sim.add_gate([('Z', 17), ('Z', 30)], pi / 4)
sim.add_gate([('Z', 20), ('Z', 33)], pi / 4)
sim.add_gate([('Z', 33), ('Z', 39)], pi / 4)
sim.add_gate([('Z', 24), ('Z', 34)], pi / 4)
sim.add_gate([('Z', 34), ('Z', 43)], pi / 4)
sim.add_gate([('Z', 28), ('Z', 35)], pi / 4)
sim.add_gate([('Z', 35), ('Z', 47)], pi / 4)
sim.add_gate([('Z', 32), ('Z', 36)], pi / 4)
sim.add_gate([('Z', 36), ('Z', 51)], pi / 4)
sim.add_gate([('Z', 37), ('Z', 52)], pi / 4)
sim.add_gate([('Z', 52), ('Z', 56)], pi / 4)
sim.add_gate([('Z', 41), ('Z', 53)], pi / 4)
sim.add_gate([('Z', 53), ('Z', 60)], pi / 4)
sim.add_gate([('Z', 45), ('Z', 54)], pi / 4)
sim.add_gate([('Z', 54), ('Z', 64)], pi / 4)
sim.add_gate([('Z', 49), ('Z', 55)], pi / 4)
sim.add_gate([('Z', 55), ('Z', 68)], pi / 4)
sim.add_gate([('Z', 58), ('Z', 71)], pi / 4)
sim.add_gate([('Z', 71), ('Z', 77)], pi / 4)
sim.add_gate([('Z', 62), ('Z', 72)], pi / 4)
sim.add_gate([('Z', 72), ('Z', 81)], pi / 4)
sim.add_gate([('Z', 66), ('Z', 73)], pi / 4)
sim.add_gate([('Z', 73), ('Z', 85)], pi / 4)
sim.add_gate([('Z', 70), ('Z', 74)], pi / 4)
sim.add_gate([('Z', 74), ('Z', 89)], pi / 4)
sim.add_gate([('Z', 75), ('Z', 90)], pi / 4)
sim.add_gate([('Z', 90), ('Z', 94)], pi / 4)
sim.add_gate([('Z', 79), ('Z', 91)], pi / 4)
sim.add_gate([('Z', 91), ('Z', 98)], pi / 4)
sim.add_gate([('Z', 83), ('Z', 92)], pi / 4)
sim.add_gate([('Z', 92), ('Z', 102)], pi / 4)
sim.add_gate([('Z', 87), ('Z', 93)], pi / 4)
sim.add_gate([('Z', 93), ('Z', 106)], pi / 4)
sim.add_gate([('Z', 96), ('Z', 109)], pi / 4)
sim.add_gate([('Z', 109), ('Z', 114)], pi / 4)
sim.add_gate([('Z', 100), ('Z', 110)], pi / 4)
sim.add_gate([('Z', 110), ('Z', 118)], pi / 4)
sim.add_gate([('Z', 104), ('Z', 111)], pi / 4)
sim.add_gate([('Z', 111), ('Z', 122)], pi / 4)
sim.add_gate([('Z', 108), ('Z', 112)], pi / 4)
sim.add_gate([('Z', 112), ('Z', 126)], pi / 4)

dyn = list(map(lambda x: x.real, sim.execute([('Z', 62)], time_steps, threshold, decay)))

plt.plot(dyn, 'b')
plt.xlabel("Time step")
plt.ylabel("<Z>")
plt.savefig(f"{dirpath}/ibm_heavy_hex.pdf")