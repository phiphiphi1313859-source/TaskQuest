# TaskQuest

**Transform your Taskwarrior tasks into an epic RPG adventure!**

TaskQuest is a gamification layer for Taskwarrior that turns completing tasks into an immersive RPG experience. Earn XP, gold, loot, and achievements as you conquer your to-do list.

## Features (Phase 1 - MVP)

- ⚔️ **RPG Character System** - Choose from 5 classes (Rogue, Ranger, Warrior, Paladin, Monk)
- 📊 **D&D 5e Stats** - STR, DEX, CON, INT, WIS, CHA that grow with task completion
- ⬆️ **Progressive Leveling** - WoW-inspired XP curve (100 * N^2.1)
- 💰 **Gold Rewards** - Earn gold based on task difficulty
- 🎯 **Smart XP System** - Rewards based on challenge, urgency, and timing
- ⏰ **Grace Period** - Late tasks still earn reduced XP (no harsh penalties)
- 🔄 **Automatic Integration** - Uses Taskwarrior hooks for seamless operation

## Installation

### From Source

```bash
# Clone the repository
git clone https://github.com/yourusername/taskquest
cd taskquest

# Build and install
cargo build --release
cargo install --path .
```

### Via Cargo

```bash
cargo install taskquest
```

## Quick Start

### 1. Initialize TaskQuest

```bash
taskquest init
```

This will:
- Configure Taskwarrior UDAs (User Defined Attributes)
- Run the setup wizard to create your character
- Set up data storage in `~/.taskquest/`

### 2. Create Tasks with Challenge Levels

```bash
# Simple task (default challenge: 5)
task add "Write documentation"

# Challenging task
task add "Refactor legacy code" challenge:8

# Epic task
task add "Launch new product" challenge:10 due:tomorrow
```

### 3. Complete Tasks and Earn Rewards

```bash
task 1 done
```

You'll see your rewards:
```
╔═══════════════════════════════╗
║     ⚔️  QUEST COMPLETE! ⚔️      ║
╠═══════════════════════════════╣
║ +85 XP  │  +45 Gold            ║
╠═══════════════════════════════╣
║ Level: 3  │  XP: 650/726       ║
║ Gold: 145                      ║
╚═══════════════════════════════╝
```

### 4. View Your Character

```bash
taskquest status
```

## CLI Commands

| Command | Description |
|---------|-------------|
| `taskquest init` | Initialize TaskQuest (setup wizard) |
| `taskquest status` | View character status |
| `taskquest stats` | View detailed statistics |
| `taskquest name "Hero Name"` | Set character name |
| `taskquest class Warrior` | Set character class |
| `taskquest statpri STR` | Set primary stat |
| `taskquest statsec CON` | Set secondary stat |

## Challenge Levels

| Level | Name | Color | XP Multiplier |
|-------|------|-------|---------------|
| 1-2 | Trivial | Gray | 10-20 XP |
| 3-4 | Easy | Light Gray | 30-40 XP |
| 5-6 | Medium | White | 50-60 XP |
| 7 | Hard | Yellow | 70 XP |
| 8 | Deadly | Orange | 80 XP |
| 9 | Legendary | Red | 90 XP |
| 10 | Epic | Magenta | 100 XP |

## XP Calculation

XP is calculated based on:
- **Base XP**: `challenge * 10`
- **Urgency Multiplier**: `1.0 + (urgency * 0.5)` (max 1.5x)
- **Timing Bonus/Penalty**:
  - Early (>24hrs before due): 1.3x
  - On Time (day of due date): 1.0x
  - Grace Period (<24hrs late): 0.8x
  - Late (>24hrs late): 0.5x
  - No due date: 1.0x

**Example**: Challenge 8 task, medium urgency, completed on time:
- Base: 80 XP
- Urgency: 80 * 1.25 = 100 XP
- Timing: 100 * 1.0 = **100 XP**

## Character Classes

| Class | Description | Suggested Stats |
|-------|-------------|-----------------|
| **Rogue** | Cunning and agile | DEX (primary), CHA (secondary) |
| **Ranger** | Skilled and versatile | DEX (primary), WIS (secondary) |
| **Warrior** | Strong and durable | STR (primary), CON (secondary) |
| **Paladin** | Righteous and balanced | STR (primary), CHA (secondary) |
| **Monk** | Disciplined and wise | WIS (primary), DEX (secondary) |

## Level Progression

| Level | Total XP Required | Tasks to Level (avg) |
|-------|-------------------|---------------------|
| 1 | 0 | Start |
| 2 | 121 | ~2 |
| 3 | 483 | ~5 |
| 5 | 2,023 | ~14 |
| 10 | 15,849 | ~82 |
| 20 | 105,737 | ~300 |
| 30 | 296,059 | ~600 |

## Data Storage

TaskQuest stores data in `~/.taskquest/` (configurable via `TASKQUEST_DATA`):

```
~/.taskquest/
├── character.json      # Your character data
├── character.json.bak  # Automatic backup
└── .git/              # Optional: git repository for sync
```

## Design Philosophy

TaskQuest follows these key principles:

1. **No Punishment** - Only rewards for completion
2. **Grace Period** - Late tasks still earn reduced XP
3. **Organic Progression** - Stats tied to actual task completion
4. **Non-Toxic Achievements** - No streak requirements (coming in Phase 2)
5. **Sustainable Motivation** - Challenging but not demotivating XP curve

## Roadmap

- [x] **Phase 1 (MVP)** - Core system, XP, gold, basic CLI
- [ ] **Phase 2** - Loot system, shop, achievements, ASCII avatars
- [ ] **Phase 3** - TUI interface, git sync, polish
- [ ] **Phase 4** - Advanced features, analytics, export/import

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT License - See [LICENSE](LICENSE) for details.

## Credits

- **Taskwarrior**: The foundation of this system
- **Habitica**: Inspiration (and lessons learned)
- **D&D 5e**: Stat system inspiration
- **World of Warcraft**: Level curve & loot system inspiration

---

*Start your quest today! Transform your tasks into an epic adventure with TaskQuest.*
