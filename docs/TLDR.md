# TaskQuest - TL;DR Quick Reference

**Version**: Phase 3 Complete
**Quick Start Guide** - For full manual see [MANUAL.md](./MANUAL.md)

---

## Installation

```bash
# Install Taskwarrior
sudo dnf install task  # Fedora/RHEL
sudo apt install taskwarrior  # Debian/Ubuntu
sudo pacman -S task  # Arch

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Build TaskQuest
git clone <repo-url> TaskQuest
cd TaskQuest
cargo install --path .

# Initialize
taskquest init
```

**Termux (Android)**:
```bash
pkg install task rust git
# Then same build process
```

---

## Quick Start

```bash
# Create character
taskquest init

# Add task with stats
task add "Code feature" challenge:7 stat1:int stat2:wis due:tomorrow

# Complete task
task 1 done
# → Gets XP, gold, maybe loot, trains INT +14 and WIS +7

# Check status
taskquest status

# View detailed stats
taskquest stats

# Shop for rewards
taskquest shop
taskquest buy 1
```

---

## Core Commands

### Character
```bash
taskquest status              # View character overview
taskquest stats               # Detailed stats with progress bars
taskquest name "NewName"      # Change name
taskquest class Warrior       # Change class (cosmetic)
```

### Shop & Rewards
```bash
taskquest shop                                    # View available rewards
taskquest buy <id|name>                           # Purchase reward
taskquest add-reward "Name" 500 "Desc" --tier epic --cooldown 168
taskquest remove-reward <id|name>                 # Remove custom reward
```

### Achievements
```bash
taskquest achievements        # View unlocked achievements
```

### Sync (Git-Based - Recommended)
```bash
taskquest sync init                               # Initialize git repo
taskquest sync init --remote git@github.com:user/repo.git
taskquest sync push                               # Save to cloud
taskquest sync pull                               # Get latest
taskquest sync status                             # Check status
taskquest sync history                            # View commits
```

---

## Taskwarrior Integration

### Challenge UDA (Required)

```bash
task add "Task" challenge:5   # Default: 5 if not set
```

**Challenge Scale**:
- 1-2: Trivial (< 5 min)
- 3-4: Simple (5-15 min)
- 5-6: Standard (15-60 min)
- 7-8: Complex (1-3 hours)
- 9-10: Epic (3+ hours)

### Stat Training UDAs (Important!)

```bash
task add "Task" challenge:6 stat1:int stat2:wis
```

**Valid Stats**: `str`, `dex`, `con`, `int`, `wis`, `cha`

**Stat Gains**:
- `stat1` gets 66% (challenge × 2)
- `stat2` gets 33% (challenge × 1)

If you don't set stat1/stat2, **no stats will be trained**!

---

## Task Examples by Activity

### Coding / Development
```bash
task add "Implement auth" challenge:8 stat1:int stat2:wis project:Work due:friday
task add "Fix bug" challenge:6 stat1:int stat2:dex project:Work +urgent
task add "Code review" challenge:5 stat1:int stat2:wis project:Work
task add "Refactor module" challenge:7 stat1:int stat2:wis project:Work
```

**Why**: INT for problem-solving, WIS for planning, DEX for speed

### Physical / Fitness
```bash
task add "Gym workout" challenge:6 stat1:str stat2:con project:Fitness
task add "Run 5K" challenge:5 stat1:con stat2:dex project:Fitness
task add "Yoga session" challenge:4 stat1:dex stat2:wis project:Fitness
task add "Heavy lifting" challenge:7 stat1:str stat2:con project:Fitness
```

**Why**: STR for strength, CON for endurance, DEX for agility

### Social / Meetings
```bash
task add "Team meeting" challenge:4 stat1:cha stat2:wis project:Work
task add "Presentation" challenge:7 stat1:cha stat2:int project:Work
task add "Client call" challenge:6 stat1:cha stat2:wis project:Work
task add "Networking event" challenge:5 stat1:cha stat2:dex project:Personal
```

**Why**: CHA for communication, WIS for insight, INT for technical discussions

### Learning / Study
```bash
task add "Study Rust" challenge:7 stat1:int stat2:wis project:Learning
task add "Read tech article" challenge:4 stat1:int stat2:wis project:Learning
task add "Watch tutorial" challenge:5 stat1:int stat2:wis project:Learning
task add "Practice algorithms" challenge:8 stat1:int stat2:dex project:Learning
```

**Why**: INT for learning, WIS for understanding, DEX for practice

### Planning / Strategy
```bash
task add "Plan sprint" challenge:6 stat1:wis stat2:int project:Work
task add "Design architecture" challenge:8 stat1:int stat2:wis project:Work
task add "Review metrics" challenge:5 stat1:wis stat2:int project:Work
task add "Quarterly planning" challenge:7 stat1:wis stat2:cha project:Work
```

**Why**: WIS for planning, INT for analysis, CHA for alignment

### Household / Chores
```bash
task add "Clean apartment" challenge:4 stat1:str stat2:con project:Home
task add "Grocery shopping" challenge:3 stat1:dex stat2:wis project:Home
task add "Cook dinner" challenge:4 stat1:dex stat2:int project:Home
task add "Organize closet" challenge:5 stat1:str stat2:wis project:Home
```

**Why**: STR for physical work, DEX for coordination, CON for stamina

### Creative / Writing
```bash
task add "Write blog post" challenge:6 stat1:int stat2:cha project:Writing
task add "Edit article" challenge:5 stat1:int stat2:wis project:Writing
task add "Design graphics" challenge:7 stat1:int stat2:dex project:Creative
task add "Brainstorm ideas" challenge:4 stat1:wis stat2:int project:Creative
```

**Why**: INT for creativity, CHA for expression, WIS for editing

### Administrative
```bash
task add "File taxes" challenge:7 stat1:int stat2:wis project:Admin
task add "Pay bills" challenge:2 stat1:wis stat2:dex project:Admin
task add "Schedule appointments" challenge:3 stat1:wis stat2:cha project:Admin
task add "Organize documents" challenge:4 stat1:wis stat2:dex project:Admin
```

**Why**: WIS for organization, INT for complexity, DEX for quick tasks

---

## Progression Formulas

### XP Formula
```
XP = challenge × 10 × urgency × timing

Urgency: 1.0 to 1.5 (based on Taskwarrior urgency)
Timing:
  Early (>24h before due): 1.3×
  On-time (day of due):    1.0×
  Grace (<24h late):       0.8×
  Late (>24h late):        0.5×
  No due date:             1.0×
```

### Gold Formula
```
Gold = (challenge × 5) ± 20% variance

Challenge 1:  4-6 gold
Challenge 5:  20-30 gold
Challenge 10: 40-60 gold
```

### Stat Growth Formula
```
stat1 = challenge × 2  (66% of total)
stat2 = challenge × 1  (33% of total)
Total = challenge × 3 points per task

Examples:
  Challenge 5: stat1 +10, stat2 +5
  Challenge 7: stat1 +14, stat2 +7
  Challenge 10: stat1 +20, stat2 +10
```

### Loot Drop Formula
```
Drop Chance = 30% + (challenge × 2%)

Challenge 1:  32%
Challenge 5:  40%
Challenge 10: 50%

Loot Types:
  70% - Bonus gold (10-50)
  20% - Normal reward
  8%  - Heroic reward
  2%  - Epic reward
  0%  - Legendary (shop only)
```

### Level Progression
```
XP Required = 100 × level^2.1

Level 1:  0 XP
Level 2:  428 XP
Level 5:  2,936 XP
Level 10: 15,849 XP
Level 20: 82,299 XP
Level 50: 1,119,134 XP
```

---

## Stats Guide

| Stat | Meaning | Use For | Examples |
|------|---------|---------|----------|
| **STR** | Strength | Physical tasks, heavy work | Gym, moving, manual labor |
| **DEX** | Dexterity | Quick tasks, agility | Typing, sports, speed work |
| **CON** | Constitution | Endurance, long projects | Marathons, long sessions |
| **INT** | Intelligence | Mental tasks, learning | Coding, studying, problem-solving |
| **WIS** | Wisdom | Planning, insight | Strategy, reviewing, organizing |
| **CHA** | Charisma | Social tasks, communication | Meetings, presentations, networking |

**Stat Milestones**:
- 100: Achievement unlock
- 500: Legendary achievement
- 1000: Transcendent achievement

---

## Classes (Cosmetic Only)

| Class | Description | Avatar Style |
|-------|-------------|--------------|
| Rogue | Cunning and agile | Hooded figure |
| Ranger | Skilled and versatile | Archer |
| Warrior | Strong and durable | Armored fighter |
| Paladin | Righteous and balanced | Holy knight |
| Monk | Disciplined and wise | Martial artist |

**Avatar Evolution**: Levels 1, 10, 20, 30, 40+

---

## Shop Rewards

### Default Rewards (IDs 1-10)

| ID | Name | Cost | Tier | Cooldown |
|----|------|------|------|----------|
| 1 | Coffee Break | 50 | Normal | None |
| 2 | Gaming Session | 100 | Normal | None |
| 3 | Movie Night | 150 | Heroic | None |
| 4 | Treat Meal | 200 | Heroic | None |
| 5 | New Book | 300 | Heroic | None |
| 6 | Day Off | 500 | Epic | 1 week |
| 7 | Hobby Supplies | 400 | Epic | None |
| 8 | Weekend Adventure | 750 | Epic | 2 weeks |
| 9 | Major Purchase | 1500 | Legendary | None |
| 10 | Epic Reward | 3000 | Legendary | 30 days |

### Custom Rewards (Phase 3)

```bash
# Add custom reward
taskquest add-reward "Spa Day" 800 "Relaxing spa" --tier epic --cooldown 168

# Examples
taskquest add-reward "Coffee" 50 "Nice coffee"  # Normal, no cooldown
taskquest add-reward "Vacation" 5000 "Week off" --tier legendary --cooldown 720

# Remove (cannot remove IDs 1-10)
taskquest remove-reward 11
taskquest remove-reward "Spa Day"
```

---

## Sync Comparison

### Git-Based (Recommended)

**Pros**:
- ✅ Version control (full history)
- ✅ Rollback to any point
- ✅ Better conflict resolution
- ✅ Offline-first
- ✅ Free cloud (GitHub/GitLab)
- ✅ Technical user-friendly

**Setup**:
```bash
taskquest sync init --remote git@github.com:user/taskquest-data.git
taskquest sync push
```

**Daily Use**:
```bash
taskquest sync pull   # Before work
# ... complete tasks ...
taskquest sync push   # After work
```

### Syncthing (Alternative)

**Pros**:
- ✅ Automatic sync
- ✅ Peer-to-peer
- ✅ Simple setup
- ✅ Real-time

**Cons**:
- ❌ No version history
- ❌ .sync-conflict files
- ❌ Can't rollback

**Setup**: See MANUAL.md

**Recommendation**: Use git-sync for technical users, Syncthing for automatic-only preference.

---

## Achievements Quick List

### By Tier

**Legendary** (2):
- Living Legend (Level 50)
- Transcendent Power (1000 in any stat)

**Epic** (8):
- Legendary Warrior (Level 30)
- Completionist (1000 quests)
- Marathon Hero (100 active days)
- Gold Hoarder (5000 gold)
- Punctual Perfectionist (100 on-time tasks)
- 6 Legendary stat achievements (500 in each stat)

**Rare** (7):
- Veteran Hero (500 quests)
- Renaissance Soul (10 projects)
- Wise Spender (10 reward purchases)
- Treasure Hunter (50 loot drops)
- Time Master (100 tasks with due dates)
- Early Riser (50 early tasks)
- Jack of All Trades (20 of each difficulty)

**Uncommon** (8):
- The Journey Begins (Level 5)
- Seasoned Adventurer (100 quests)
- Consistent Hero (30 active days)
- The Undaunted (10 difficulty-10 quests)
- Epic Collector (Epic loot drop)
- Pressure Handler (25 grace period tasks)
- Master of Balance (One of each difficulty)
- 6 stat achievements (100 in each stat)

**Common** (4):
- First Steps (First quest)
- Phoenix Rising (5 quests after 30-day break)

---

## Workflow Examples

### Single Device
```bash
# Just use normally
task add "Code" challenge:7 stat1:int stat2:wis
task 1 done
taskquest status
```

### Multi-Device (Git Sync)
```bash
# Device A (morning)
taskquest sync pull
task add "Task" challenge:6 stat1:int stat2:wis
task 1 done
taskquest sync push

# Device B (evening)
taskquest sync pull  # Gets changes from Device A
taskquest status     # See updated character
task add "Task2" challenge:5 stat1:str stat2:con
task 1 done
taskquest sync push

# Device A (next day)
taskquest sync pull  # Gets Task2 completion
```

### Weekly Review
```bash
# Check progress
taskquest stats          # See stat distribution
taskquest achievements   # Check unlocks
taskquest sync history   # Review activity

# Identify gaps
# If DEX is red → Do more quick/agile tasks
# If CHA is red → Do more social tasks
# If STR is red → Do more physical tasks

# Adjust tasks accordingly
task add "Gym" challenge:6 stat1:str stat2:con
task add "Meeting" challenge:5 stat1:cha stat2:wis
```

### Reward Strategy
```bash
# Save up for big reward
taskquest shop  # Check what's affordable

# When you have enough gold
taskquest buy "Day Off"

# Add seasonal rewards
taskquest add-reward "Holiday Trip" 3000 "Winter vacation" --tier legendary --cooldown 8760
```

---

## Common Patterns

### Morning Routine
```bash
taskquest sync pull                    # Get latest
task next                              # See priorities
task add "Plan day" challenge:4 stat1:wis stat2:int
```

### Task Completion
```bash
task 1 done
# Automatic: XP, gold, maybe loot, stat training
taskquest status  # Check progress
```

### Evening Wrap-Up
```bash
taskquest stats              # Review day
taskquest shop               # Check rewards
taskquest sync push          # Save progress
```

### Weekly Planning
```bash
task project:Work            # Review work tasks
task project:Personal        # Review personal tasks
task add "Sprint planning" challenge:6 stat1:wis stat2:int project:Work
taskquest achievements       # Check progress
```

---

## Troubleshooting

### Hooks Not Working
```bash
chmod +x ~/.task/hooks/on-*-taskquest
taskquest init --skip-wizard
```

### Git Sync Fails
```bash
# Set up SSH keys
ssh-keygen -t ed25519
cat ~/.ssh/id_ed25519.pub  # Add to GitHub

# Or check remote
cd ~/.taskquest
git remote -v
```

### Stats Not Training
```bash
# Make sure you set stat1 and stat2!
task add "Task" challenge:5 stat1:int stat2:wis
```

### Character Corrupted
```bash
# Restore from backup
cp ~/.taskquest/character.json.bak ~/.taskquest/character.json

# Or restore from git
cd ~/.taskquest
git log
git checkout <commit> -- character.json
```

---

## Best Practices

1. **Always set challenge** - Determines rewards
2. **Always set stat1/stat2** - Trains stats
3. **Use due dates** - Get timing bonuses (1.3× for early!)
4. **Sync regularly** - `sync push/pull` frequently
5. **Review weekly** - Check stat distribution
6. **Customize rewards** - Add what motivates YOU
7. **Use projects** - Organize by area (Work/Personal/Fitness)
8. **Check achievements** - Track long-term progress

---

## Quick Command Reference

```bash
# Character
taskquest status              # Overview
taskquest stats               # Detailed + bars
taskquest name "Name"         # Rename
taskquest class Warrior       # Change class

# Shop
taskquest shop                # View
taskquest buy <id|name>       # Purchase
taskquest add-reward "X" 500 "Y" --tier epic --cooldown 168
taskquest remove-reward <id|name>

# Achievements
taskquest achievements        # View

# Sync
taskquest sync init [--remote URL]
taskquest sync push           # Save
taskquest sync pull           # Get latest
taskquest sync status         # Check
taskquest sync history        # View commits

# Tasks
task add "X" challenge:N stat1:S1 stat2:S2 [project:P] [due:D] [+tag]
task <id> done                # Complete
task next                     # Priorities
task projects                 # List projects
```

---

## File Locations

```
~/.taskquest/
  ├── character.json        # Your character
  ├── achievements.json     # Progress
  ├── shop.json             # Rewards
  └── .git/                 # Version history

~/.task/
  ├── pending.data          # Tasks
  ├── completed.data        # History
  ├── .taskrc               # Config
  └── hooks/                # TaskQuest hooks
```

---

## Key Differences from Old System

| Feature | OLD | NEW (Phase 3) |
|---------|-----|---------------|
| Stats | Fixed by class | Task-based training |
| Rewards | 10 fixed | Unlimited custom |
| Stat Display | Numbers only | Progress bars + colors |
| Sync | Syncthing only | Git (recommended) + Syncthing |
| Version Control | None | Full git history |
| Rollback | Impossible | Any commit |
| Customization | Limited | Extensive |

---

## Links

- **Full Manual**: [MANUAL.md](./MANUAL.md)
- **Sync Analysis**: [SYNC_ANALYSIS.md](./SYNC_ANALYSIS.md)
- **Phase 3 Features**: [PHASE3_FEATURES.md](./PHASE3_FEATURES.md)
- **Audit Report**: [report.md](./report.md)

---

**That's it! You're ready to quest! ⚔️**

Start with: `taskquest init` → Add tasks with stats → Complete them → Enjoy the dopamine! 🎮
