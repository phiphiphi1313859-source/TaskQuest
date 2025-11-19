# TaskQuest - Quick Start Guide

## Installation

```bash
# Build the project
cargo build --release

# Install it
cargo install --path .
```

## Setup

### 1. Initialize TaskQuest

```bash
taskquest init
```

Answer the prompts:
- **Hero Name**: Your character's name (e.g., "Aria Shadowstep")
- **Class**: Choose 1-5:
  1. Rogue (DEX/CHA) - Cunning and agile
  2. Ranger (DEX/WIS) - Skilled and versatile
  3. Warrior (STR/CON) - Strong and durable
  4. Paladin (STR/CHA) - Righteous and balanced
  5. Monk (WIS/DEX) - Disciplined and wise
- **Primary Stat**: Main stat (gets +2 per task)
- **Secondary Stat**: Secondary stat (gets +1 per task)

### 2. Create Your First Quest

```bash
# Simple task (default challenge: 5)
task add "Complete the TaskQuest tutorial"

# Medium difficulty task
task add "Write project documentation" challenge:6

# Hard task with a deadline
task add "Refactor authentication system" challenge:8 due:tomorrow
```

### 3. Complete a Task

```bash
# Mark your first task as complete
task 1 done
```

You'll see something like:
```
╔═══════════════════════════════╗
║     ⚔️  QUEST COMPLETE! ⚔️      ║
╠═══════════════════════════════╣
║ +50 XP  │  +25 Gold            ║
╠═══════════════════════════════╣
║ Level: 1  │  XP: 50/121        ║
║ Gold: 25                       ║
╚═══════════════════════════════╝
```

### 4. Check Your Progress

```bash
# View your character status
taskquest status

# View detailed statistics
taskquest stats
```

## Understanding Challenge Levels

| Challenge | Difficulty | XP Range | Gold Range | When to Use |
|-----------|-----------|----------|------------|-------------|
| 1-2 | Trivial | 10-20 | 4-6 | Quick 5-min tasks |
| 3-4 | Easy | 30-40 | 12-16 | Simple 15-30 min tasks |
| 5-6 | Medium | 50-60 | 20-30 | Standard 1-2 hour tasks |
| 7 | Hard | 70+ | 28-35 | Complex 4+ hour tasks |
| 8 | Deadly | 80+ | 32-40 | Very difficult tasks |
| 9 | Legendary | 90+ | 36-45 | Major project milestones |
| 10 | Epic | 100+ | 40-60 | Epic achievements |

## Tips for Best Experience

1. **Set Realistic Challenge Levels** - Don't inflate difficulty just for XP
2. **Use Deadlines** - Tasks completed early get 1.3x XP bonus
3. **Grace Period** - Tasks <24hrs late still get 0.8x XP (no harsh penalties)
4. **Track Progress** - Use `taskquest status` regularly to see your growth
5. **Stat Strategy** - Choose primary/secondary stats that match your work style

## Character Classes Guide

### Rogue (DEX/CHA)
- **Best for**: Quick, agile task completion
- **Playstyle**: Many small tasks, social projects
- **Example**: Sprint planning, client communication, quick fixes

### Ranger (DEX/WIS)
- **Best for**: Versatile, balanced approach
- **Playstyle**: Mix of different task types
- **Example**: Full-stack development, mixed responsibilities

### Warrior (STR/CON)
- **Best for**: Heavy lifting, endurance work
- **Playstyle**: Large, difficult tasks
- **Example**: Backend development, infrastructure work

### Paladin (STR/CHA)
- **Best for**: Leadership and difficult challenges
- **Playstyle**: High-impact tasks, team coordination
- **Example**: Tech lead, project management

### Monk (WIS/DEX)
- **Best for**: Thoughtful, disciplined work
- **Playstyle**: Research, careful planning, implementation
- **Example**: Architecture, research, optimization

## Next Steps

1. Complete 5-10 tasks to get to Level 2
2. Experiment with different challenge levels
3. Try tasks with deadlines to earn bonus XP
4. View your stats growth with `taskquest stats`

## Troubleshooting

### Character not found error
```bash
# Re-run the init wizard
taskquest init
```

### Want to change character settings
```bash
# Change name
taskquest name "New Name"

# Change class
taskquest class Warrior

# Change stats
taskquest statpri STR
taskquest statsec CON
```

### View character data
```bash
# Check your character file
cat ~/.taskquest/character.json
```

---

**Ready to start your epic quest? Run `taskquest init` now!**
