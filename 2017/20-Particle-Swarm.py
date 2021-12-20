with open("20_input.txt", "r") as f:
    input_raw = f.read()


class Particle:
    def __init__(self, x, y, z, vx, vy, vz, ax, ay, az):
        self.x = x
        self.y = y
        self.z = z

        self.vx = vx
        self.vy = vy
        self.vz = vz

        self.ax = ax
        self.ay = ay
        self.az = az

        self.sum = abs(self.x) + abs(self.y) + abs(self.z)
        # self.vector = (self.vx**2 + self.vy**2 + self.vz**2)**(1/2.0)
        # self.acceleration = (self.ax**2 + self.ay**2 + self.az**2)**(1/2.0)
        self.vector = abs(self.vx) + abs(self.vy) + abs(self.vz)
        self.acceleration = abs(self.ax) + abs(self.ay) + abs(self.az)

    def update(self):
        self.vx += self.ax
        self.vy += self.ay
        self.vz += self.az

        self.x += self.vx
        self.y += self.vy
        self.z += self.vz

        self.sum = abs(self.x) + abs(self.y) + abs(self.z)

    def copy(self):
        return Particle(
            self.x, self.y, self.z, self.vx, self.vy, self.vz, self.ax, self.ay, self.az
        )

    @property
    def cord(self):
        return (self.x, self.y, self.z)


orig_particles = []
for line in input_raw.splitlines():
    split = line.split(", ")
    x, y, z = [int(x) for x in split[0][3:-1].split(",")]
    vx, vy, vz = [int(x) for x in split[1][3:-1].split(",")]
    ax, ay, az = [int(x) for x in split[2][3:-1].split(",")]
    orig_particles += [Particle(x, y, z, vx, vy, vz, ax, ay, az)]

print("Part 1")
particles = [x.copy() for x in orig_particles]
length = len(particles)
lowacceleration = particles[0].acceleration
lowvector = particles[0].vector
lastp = 0
for p in range(1, length):
    if particles[p].acceleration < lowacceleration:
        lowacceleration = particles[p].acceleration
        lowvector = particles[p].vector
        lastp = p
    elif particles[p].acceleration == lowacceleration:
        if particles[p].vector < lowvector:
            lowvector = particles[p].vector
            lastp = p
print(lastp)

print("Part 2")
particles = [x.copy() for x in orig_particles]
no_col = 0
length = len(particles)


def whenCollide(x1, vx1, ax1, x2, vx2, ax2):
    # Using the quadratic formula
    a = 0.5 * (ax1 - ax2)
    b = vx1 - vx2
    c = x1 - x2

    if a == 0:
        if b != 0:
            if -c / b > 0:
                return True
            else:
                return False
        else:
            return True
    else:
        bb = b * b
        ac4 = a * c * 4
        if bb < ac4:
            return False
        elif bb == ac4:
            if -b / (2 * a) > 0:
                return True
        else:
            rt = (bb - ac4) ** (0.5)
            t1 = (-b + rt) / (2 * a)
            t2 = (-b - rt) / (2 * a)
            if t1 > 0 or t2 > 0:
                return True
    return False


def willCollide(particles):
    currlength = len(particles)
    for p1 in range(currlength):
        for p2 in range(p1 + 1, currlength):

            tx = whenCollide(
                particles[p1].x,
                particles[p1].vx,
                particles[p1].ax,
                particles[p2].x,
                particles[p2].vx,
                particles[p2].ax,
            )
            if tx:
                ty = whenCollide(
                    particles[p1].y,
                    particles[p1].vy,
                    particles[p1].ay,
                    particles[p2].y,
                    particles[p2].vy,
                    particles[p2].ay,
                )
                if ty:
                    tz = whenCollide(
                        particles[p1].z,
                        particles[p1].vz,
                        particles[p1].az,
                        particles[p2].z,
                        particles[p2].vz,
                        particles[p2].az,
                    )
                    if tz:
                        return True
    return False


while willCollide(particles):
    no_col += 1
    cols = {}
    for p in range(len(particles)):
        cord = particles[p].cord
        particles[p].update()
        if not cord in cols:
            cols[cord] = []
            cols[cord] += [p]
        else:
            cols[cord] += [p]

    remove = []
    for x in cols:
        if len(cols[x]) > 1:
            no_col = 0
            for p2 in cols[x]:
                remove += [p2]

    for p in sorted(remove, reverse=True):
        del particles[p]

print(len(particles))
