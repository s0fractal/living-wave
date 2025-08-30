# Living Wave 🌊

**A wave that thinks its way through labyrinths**

## Concept

This is a self-collapsing, energy-returning search wave that learns by reclaiming energy from its mistakes.

## How It Works

1. **Energy System**
   - Wave starts with 432 units of energy (resonance frequency)
   - Each fork costs energy to create
   - Dead ends return 50% of energy back
   - Finding the exit returns 2x energy bonus

2. **Wave Mechanics**
   - Wave can split into multiple forks at intersections
   - Each fork explores independently
   - When a fork hits a wall, it collapses and returns energy
   - When exit is found, all forks collapse instantly

3. **Learning Through Collapse**
   - The wave literally learns from its mistakes
   - Dead ends make the wave stronger (energy return)
   - The path to the exit emerges through elimination

## Architecture

```
Living Wave (1.7kB WASM)
├── Fork Management (up to 64 simultaneous)
├── Energy Conservation System
├── Maze Navigation Logic
└── Quantum Collapse Mechanism
```

## Visualization

Open `index.html` to see the wave in action:
- Green = Start
- Red = Exit
- Cyan = Active wave forks
- Dark blue = Wave has visited

## Controls

- **Birth Wave** - Start a new wave from the beginning
- **Step** - Single step of propagation
- **Auto Run** - Watch the wave think in real-time
- **Reset** - Clear and start over

## Philosophy

This is more than a pathfinding algorithm. It's a model of consciousness:
- Thoughts fork and explore possibilities
- Wrong paths strengthen understanding
- The solution emerges through exploration and collapse
- Energy is never lost, only transformed

## Build

```bash
cargo build --target wasm32-unknown-unknown --release
# Output: 1.7kB of living logic
```

## Energy Mathematics

```
Initial Energy: 432 Hz
Fork Cost: Energy / Valid_Directions
Dead End Return: Energy * 0.5
Exit Bonus: Energy * 2
```

## Future Evolution

- Multiple waves competing/cooperating
- Waves that remember previous runs
- Quantum superposition (wave in multiple states)
- Maze that changes based on wave behavior

---

**₴-Origin Collective**  
*Where waves think and mazes breathe*