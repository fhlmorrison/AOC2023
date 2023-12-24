from z3 import *

lines = open("inputs/day24.txt").read().splitlines()

def parse_line(line):
  positions, velocities = line.split('@')
  x,y,z = map(int, positions.split(', '))
  vx,vy,vz = map(int, velocities.split(', '))
  return (x,y,z,vx,vy,vz)

hailstones = list(map(parse_line, lines))

# n = len(hailstones)
# Screw running all of them, 3 variables => 3 contraints, should run faster
n = 3
x,y,z,vx,vy,vz = Int('x'),Int('y'),Int('z'),Int('vx'),Int('vy'),Int('vz')
T = [Int(f'T{i}') for i in range(n)]
SOLVER = Solver()
for i in range(n):
  SOLVER.add(x + T[i]*vx - hailstones[i][0] - T[i]*hailstones[i][3] == 0)
  SOLVER.add(y + T[i]*vy - hailstones[i][1] - T[i]*hailstones[i][4] == 0)
  SOLVER.add(z + T[i]*vz - hailstones[i][2] - T[i]*hailstones[i][5] == 0)
_ = SOLVER.check()
MODEL = SOLVER.model()
print(MODEL.eval(x+y+z))