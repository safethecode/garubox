# Garubox

A pixel-based falling sand game built with Rust and macroquad. Experience realistic physics simulations with various elements and watch them interact in a dynamic sandbox environment.

![Garubox](https://img.shields.io/badge/version-0.1.0-blue)
![Rust](https://img.shields.io/badge/rust-2024-orange)

## Features

- **Real-time Physics Simulation**: Watch elements interact with realistic physics
- **Multiple Elements**: Sand, Water, Stone, and AI-driven People
- **Interactive Drawing**: Paint elements with adjustable brush sizes
- **Smart AI**: People can walk, jump, climb, and even drown
- **Intuitive UI**: Button-based element selection with visual previews
- **Keyboard Shortcuts**: Quick element switching with number keys
- **Pause & Clear**: Control simulation flow with Space and C keys

## Building and Running

### Prerequisites

- Rust (2024 edition or later)
- Cargo

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run --release
```

## Architecture

```
src/
├── main.rs
├── grid.rs
├── ui.rs
├── types/
│   └── element.rs
├── elements/
│   ├── sand.rs
│   ├── water.rs
│   ├── stone.rs
│   ├── person.rs
│   └── air.rs
└── states/
    └── person_state.rs
```

## Roadmap: Planned Elements (50+)

### Heat & Fire

1. **Fire** - Spreads to flammable materials, produces smoke
2. **Lava** - Hot liquid that burns and melts elements
3. **Ice** - Freezes water, melts with heat
4. **Steam** - Hot gas that rises and condenses
5. **Smoke** - Rises and dissipates over time
6. **Magma** - Slower, denser lava variant
7. **Ember** - Glowing hot particles

### Explosives & Energy

8. **TNT** - Explosive material triggered by fire or pressure
9. **Gunpowder** - Fast-burning explosive trail
10. **C4** - Remote-detonated explosive
11. **Firework** - Colorful explosive effects
12. **Gas** - Flammable and explosive under pressure
13. **Electricity** - Conducts through metal, damages entities
14. **Lightning** - Random electrical discharge
15. **Nuclear** - Massive explosive with radiation

### Liquids

16. **Oil** - Flammable liquid, floats on water
17. **Acid** - Corrodes most materials
18. **Mud** - Mixture of water and dirt, slow-moving
19. **Blood** - Fluid released by damaged entities
20. **Poison** - Toxic liquid harmful to entities
21. **Honey** - Viscous, slow-flowing liquid
22. **Alcohol** - Highly flammable liquid
23. **Mercury** - Dense liquid metal

### Organic Materials

24. **Dirt** - Can grow plants, affected by water
25. **Grass** - Spreads on dirt with water
26. **Wood** - Burns slowly, structural material
27. **Plant** - Grows over time with water
28. **Tree** - Large plant structure
29. **Leaf** - Falls and decays
30. **Seed** - Grows into plants
31. **Fungus** - Spreads in darkness

### Minerals & Metals

32. **Iron** - Conductive, heavy metal
33. **Gold** - Dense, non-reactive metal
34. **Copper** - Highly conductive metal
35. **Coal** - Burnable fuel source
36. **Diamond** - Indestructible crystal
37. **Glass** - Transparent solid, breaks under pressure
38. **Obsidian** - Volcanic glass
39. **Salt** - Dissolves in water

### Gases

40. **Oxygen** - Supports fire, breathable
41. **Hydrogen** - Highly explosive gas
42. **Carbon Dioxide** - Heavy gas, suffocates
43. **Chlorine** - Toxic green gas
44. **Methane** - Flammable gas

### Advanced Materials

45. **Concrete** - Hardens over time when mixed
46. **Rubber** - Bouncy, insulating material
47. **Plastic** - Melts with heat
48. **Gel** - Semi-solid viscous material
49. **Foam** - Light, buoyant material
50. **Glowstone** - Emits light

### Interactive Elements

51. **Clone** - Duplicates adjacent elements
52. **Void** - Deletes anything it touches
53. **Teleporter** - Transports elements to paired teleporter
54. **Converter** - Changes elements to another type
55. **Gravity Well** - Attracts nearby elements
56. **Portal** - Links two areas of the canvas

### Entities & Life

57. **Animal** - Moves and eats plants
58. **Fish** - Swims in water
59. **Bird** - Flies through air
60. **Zombie** - Converts people to zombies
61. **Robot** - Mechanical entity

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.

**Note**: This is a work in progress. Many elements from the roadmap are not yet implemented but are planned for future releases.
