# TaskQuest - Complete User Manual

**Version**: Phase 3 Complete
**Last Updated**: November 19, 2025

Transform your tasks into an epic RPG adventure!

TaskQuest is a gamification system for Taskwarrior that adds D&D-style RPG mechanics to your task management. Complete tasks to earn XP, level up your character, train stats, collect loot, unlock achievements, and purchase rewards.

---

## Table of Contents

1. [Installation](#installation)
   - [Linux Installation](#linux-installation)
   - [Termux (Android) Installation](#termux-android-installation)
2. [Initial Setup](#initial-setup)
3. [Synchronization](#synchronization)
   - [Git-Based Sync (Recommended)](#git-based-sync-recommended)
   - [Syncthing Alternative](#syncthing-alternative)
4. [How TaskQuest Works](#how-taskquest-works)
5. [Character System](#character-system)
6. [Progression Mechanics](#progression-mechanics)
7. [Shop & Rewards](#shop--rewards)
8. [Achievements](#achievements)
9. [Commands Reference](#commands-reference)
10. [Taskwarrior Integration](#taskwarrior-integration)
11. [Best Practices](#best-practices)
12. [Troubleshooting](#troubleshooting)
13. [Advanced Usage](#advanced-usage)

---

## Installation

### Linux Installation

#### Prerequisites

1. **Taskwarrior** - Install via your package manager:
   ```bash
   # Fedora/RHEL
   sudo dnf install task

   # Debian/Ubuntu
   sudo apt install taskwarrior

   # Arch Linux
   sudo pacman -S task
   ```

2. **Rust** (for building from source):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

#### Building TaskQuest

1. Clone or download the TaskQuest repository:
   ```bash
   cd ~/Downloads
   git clone <repository-url> TaskQuest
   cd TaskQuest
   ```

2. Build and install:
   ```bash
   cargo build --release
   cargo install --path .
   ```

3. Add Cargo bin to your PATH (if not already):
   ```bash
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

4. Verify installation:
   ```bash
   taskquest --help
   ```

---

### Termux (Android) Installation

Termux allows you to run Linux tools on Android - perfect for TaskQuest on-the-go!

#### Prerequisites

1. **Install Termux** from F-Droid (recommended) or Google Play Store

2. **Update packages**:
   ```bash
   pkg update && pkg upgrade
   ```

3. **Install required packages**:
   ```bash
   pkg install task rust git
   ```

#### Building TaskQuest on Termux

1. Clone the repository:
   ```bash
   cd ~
   git clone <repository-url> TaskQuest
   cd TaskQuest
   ```

2. Build and install:
   ```bash
   cargo build --release
   cargo install --path .
   ```

3. Verify installation:
   ```bash
   taskquest --help
   ```

**Note**: On Termux, verify `~/.cargo/bin` is in your PATH:
```bash
echo $PATH | grep cargo
```

---

## Initial Setup

After installation, initialize TaskQuest:

```bash
taskquest init
```

This will:
1. Configure Taskwarrior UDAs (User Defined Attributes)
2. Run the character creation wizard
3. Set up hook files in `~/.task/hooks/`

### Character Creation Wizard

The wizard guides you through:

1. **Hero Name** - Choose your character's name

2. **Class Selection** - Pick from 5 classes (purely cosmetic, for flavor):
   - **Rogue** - Cunning and agile
   - **Ranger** - Skilled and versatile
   - **Warrior** - Strong and durable
   - **Paladin** - Righteous and balanced
   - **Monk** - Disciplined and wise

**Important**: Classes are purely for roleplay flavor and avatar appearance. Your stats are determined by the tasks you complete, not your class!

Example:
```
What is your hero's name? Aragorn

Choose your class (for flavor and roleplay):
  1. Rogue - Cunning and agile
  2. Ranger - Skilled and versatile
  3. Warrior - Strong and durable
  4. Paladin - Righteous and balanced
  5. Monk - Disciplined and wise

Your choice (1-5): 2

✅ Character created successfully!

Your hero: Aragorn, the Ranger, Level 1

💡 Tip: Set stat1 and stat2 on tasks to train specific stats!
   Example: task add "Code review" challenge:6 stat1:int stat2:wis

Start completing tasks to earn XP, gold, and loot!
```

---

## Synchronization

TaskQuest supports two synchronization methods. **Git-based sync is recommended** for most users.

### Git-Based Sync (Recommended)

**New in Phase 3**: Superior syncing with version control, rollback, and better conflict resolution.

#### Why Git Sync?

✅ **Version Control** - Full history of your character progression
✅ **Rollback** - Restore to any previous state
✅ **Better Conflicts** - Intelligent merge instead of .sync-conflict files
✅ **Offline-First** - Commit locally, push when online
✅ **Free Cloud Hosting** - GitHub/GitLab private repositories
✅ **Technical User-Friendly** - Perfect for CLI-comfortable users

See [SYNC_ANALYSIS.md](./SYNC_ANALYSIS.md) for detailed comparison with Syncthing.

#### Quick Setup

1. **Initialize git repository**:
   ```bash
   taskquest sync init
   ```

2. **Create GitHub private repository** (recommended):
   ```bash
   # On GitHub.com:
   # 1. Click "New repository"
   # 2. Name it "taskquest-data"
   # 3. Make it PRIVATE
   # 4. Don't initialize with README

   # Then add as remote:
   cd ~/.taskquest
   git remote add origin git@github.com:username/taskquest-data.git
   ```

3. **Push initial data**:
   ```bash
   taskquest sync push
   ```

4. **On other devices** (after initial setup):
   ```bash
   # Clone the repository
   git clone git@github.com:username/taskquest-data.git ~/.taskquest

   # Install TaskQuest
   cd ~/TaskQuest  # Your TaskQuest source directory
   cargo install --path .

   # Use normally
   taskquest status
   ```

#### Daily Workflow

**Single Device**:
```bash
# Just use normally - changes are saved locally
task add "Task" challenge:5 stat1:int stat2:wis
task 1 done
# No sync needed for single device
```

**Multiple Devices**:
```bash
# Before starting work
taskquest sync pull

# Work on tasks
task add "Code feature" challenge:7 stat1:int stat2:wis
task 1 done

# When done
taskquest sync push
```

#### Sync Commands

- `taskquest sync init` - Initialize repository
- `taskquest sync init --remote <url>` - Initialize with remote
- `taskquest sync push` - Commit and push changes
- `taskquest sync pull` - Pull and merge changes
- `taskquest sync status` - Show sync status
- `taskquest sync history` - View commit history (last 10)
- `taskquest sync history 20` - View last 20 commits

#### SSH Key Setup (Recommended)

For passwordless push/pull:

```bash
# Generate SSH key
ssh-keygen -t ed25519 -C "your_email@example.com"

# Copy public key
cat ~/.ssh/id_ed25519.pub

# Add to GitHub:
# Settings → SSH and GPG keys → New SSH key
```

For comprehensive git-sync documentation, see [PHASE3_FEATURES.md](./PHASE3_FEATURES.md).

---

### Syncthing Alternative

Syncthing provides automatic peer-to-peer sync. Use this if you prefer automatic syncing without version control.

#### What to Sync

TaskQuest stores all data in `~/.taskquest/`:
- `character.json` - Your character data (level, stats, gold, XP)
- `achievements.json` - Achievement progress and unlocks
- `shop.json` - Reward store and purchase history

Taskwarrior data in `~/.task/`:
- `pending.data` - Pending tasks
- `completed.data` - Completed tasks
- `.taskrc` - Configuration (includes UDAs)
- `hooks/` - TaskQuest hooks

#### Syncthing Setup

1. **Install Syncthing**:
   ```bash
   # Fedora: sudo dnf install syncthing
   # Debian/Ubuntu: sudo apt install syncthing
   # Arch: sudo pacman -S syncthing
   # Termux: pkg install syncthing
   ```

2. **Start Syncthing**:
   ```bash
   syncthing
   # Access web UI at http://localhost:8384
   ```

3. **Add devices** - Pair your devices using device IDs

4. **Create folders** to sync `~/.taskquest/` and `~/.task/`

5. **Important**: After syncing, ensure hooks are executable:
   ```bash
   chmod +x ~/.task/hooks/on-*-taskquest
   ```

#### Syncthing Notes

⚠️ **Conflict Warning**: If you complete tasks on both devices while offline, Syncthing creates `.sync-conflict` files. You must manually resolve conflicts.

**Best Practices**:
- Always sync before using TaskQuest on a device
- Avoid using both devices offline simultaneously
- If conflicts occur, keep the newest version

---

## How TaskQuest Works

TaskQuest integrates with Taskwarrior through **hooks** - scripts that run automatically when you interact with tasks.

### The Flow

1. **Add a task** with Taskwarrior (include stat1 and stat2 to train specific abilities):
   ```bash
   task add "Write documentation" project:TaskQuest challenge:7 stat1:int stat2:wis due:tomorrow
   ```

2. **Complete the task**:
   ```bash
   task 1 done
   ```

3. **TaskQuest hook triggers automatically**:
   - Calculates XP based on challenge, urgency, and timing
   - Awards gold (5 × challenge ± 20% variance)
   - Rolls for loot drops (30% + challenge × 2%)
   - Increases character stats (stat1 gets 66%, stat2 gets 33% of gains)
   - Checks for level-ups
   - Checks for newly unlocked achievements
   - Displays beautiful reward notification

4. **Example output**:
   ```
   ╔════════════════════════════════════════╗
   ║       ⚔️  QUEST COMPLETE! ⚔️            ║
   ╠════════════════════════════════════════╣
   ║ +91 XP  │  +35 Gold (+ 15 bonus!)      ║
   ╠════════════════════════════════════════╣
   ║ 💎 LOOT DROP! Swift Boots [Heroic]     ║
   ╠════════════════════════════════════════╣
   ║ Level: 5  │  XP: 2485/2936             ║
   ║ Gold: 842                              ║
   ╚════════════════════════════════════════╝
   ```

---

## Character System

### The Six Stats (D&D 5e)

Each stat represents a different aspect of your productivity and grows based on **the tasks you complete**:

- **STR (Strength)** - Physical tasks, heavy lifting, demanding work (gym, moving, manual labor)
- **DEX (Dexterity)** - Quick tasks, agility, adaptability (typing, sports, coding sprints)
- **CON (Constitution)** - Endurance, long-term projects, resilience (marathons, long work sessions)
- **INT (Intelligence)** - Mental tasks, problem-solving, learning (coding, studying, research)
- **WIS (Wisdom)** - Planning, insight, strategic thinking (planning, reviewing, decision-making)
- **CHA (Charisma)** - Social tasks, communication, leadership (meetings, presentations, networking)

**Stats are trained by tasks, not by your class!** All stats start at 10. When you complete a task, you specify which stats it trains using `stat1` and `stat2`:

- **stat1** receives 66% of the gain (challenge × 2 points)
- **stat2** receives 33% of the gain (challenge × 1 point)

**Example**: Complete a challenge 6 coding task with `stat1:int stat2:wis`:
- INT +12 (6 × 2)
- WIS +6 (6 × 1)

This system lets you see which life areas you're improving in and which you're neglecting!

### Classes

Classes are **purely cosmetic** - they don't affect stats or gameplay, just roleplay flavor and your avatar appearance:

| Class   | Description                    | Avatar Style     |
|---------|--------------------------------|------------------|
| Rogue   | Cunning and agile              | Hooded figure    |
| Ranger  | Skilled and versatile          | Archer           |
| Warrior | Strong and durable             | Armored fighter  |
| Paladin | Righteous and balanced         | Holy knight      |
| Monk    | Disciplined and wise           | Martial artist   |

Your avatar evolves at levels 1, 10, 20, 30, and 40+ showing your character's growth!

### Levels

- Start at Level 1
- Max level: 50
- XP required follows WoW-inspired curve: `100 × level^2.1`
- Each level makes you feel more powerful

Example progression:
- Level 1: 0 XP
- Level 2: 428 XP
- Level 5: 2,936 XP
- Level 10: 15,849 XP
- Level 20: 82,299 XP
- Level 50: 1,119,134 XP

---

## Progression Mechanics

### XP (Experience Points)

**Base Formula:**
```
XP = challenge × 10 × urgency_multiplier × timing_multiplier
```

**Components:**

1. **Challenge (1-10)**: Set when creating a task
   ```bash
   task add "Easy task" challenge:3
   task add "Hard task" challenge:9
   ```

2. **Urgency Multiplier**: Based on Taskwarrior's urgency score
   - Range: 1.0 to 1.5
   - Calculated as: `1 + (urgency × 0.5, max 0.5)`

3. **Timing Multiplier**: Rewards punctuality
   - **Early** (>24hrs before due): 1.3× XP
   - **On-Time** (day of due date): 1.0× XP
   - **Grace Period** (<24hrs late): 0.8× XP
   - **Late** (>24hrs late): 0.5× XP
   - **No Due Date**: 1.0× XP

**Philosophy**: No punishment, only reduced rewards. Tasks completed late still give XP!

### Gold

Gold is awarded on task completion:

**Formula:**
```
Gold = (challenge × 5) ± 20% variance
```

- Challenge 1: 4-6 gold
- Challenge 5: 20-30 gold
- Challenge 10: 40-60 gold

Gold is used to purchase rewards from the shop.

### Stats Growth

Every completed task increases the stats you specify. Stats grow based on **task-level UDAs**, not character class:

**Formula:**
```
stat1: +(challenge × 2)  [66% of total gain]
stat2: +(challenge × 1)  [33% of total gain]
Total gain per task: challenge × 3 points
```

**Examples:**

Coding task (challenge 7):
```bash
task add "Implement auth" challenge:7 stat1:int stat2:wis
# On completion: INT +14, WIS +7
```

Gym session (challenge 5):
```bash
task add "Workout" challenge:5 stat1:str stat2:con
# On completion: STR +10, CON +5
```

Team meeting (challenge 4):
```bash
task add "Present roadmap" challenge:4 stat1:cha stat2:wis
# On completion: CHA +8, WIS +4
```

**Benefits:**
- See which areas of life you're improving
- Identify neglected stats (areas you're avoiding)
- Build a character that reflects your actual work
- No fixed class restrictions!

### Loot Drops

Loot drops occur randomly when completing tasks:

**Drop Chance:**
```
30% + (challenge × 2%)
```

- Challenge 1: 32% chance
- Challenge 5: 40% chance
- Challenge 10: 50% chance

**Loot Types** (weighted random):
- **70%** - Bonus gold (10-50 gold)
- **20%** - Normal tier reward
- **8%** - Heroic tier reward
- **2%** - Epic tier reward
- **0%** - Legendary (not droppable, shop-only)

---

## Shop & Rewards

### The Shop

View available rewards:
```bash
taskquest shop
```

Output shows:
- Your current gold
- All available rewards (not on cooldown)
- Affordability indicator (✓ or ✗)
- Reward tiers with color coding
- Cooldown timers

### Default Rewards

| ID | Reward              | Cost | Tier      | Cooldown |
|----|---------------------|------|-----------|----------|
| 1  | Coffee Break        | 50   | Normal    | None     |
| 2  | Gaming Session      | 100  | Normal    | None     |
| 3  | Movie Night         | 150  | Heroic    | None     |
| 4  | Treat Meal          | 200  | Heroic    | None     |
| 5  | New Book            | 300  | Heroic    | None     |
| 6  | Day Off             | 500  | Epic      | 1 week   |
| 7  | Hobby Supplies      | 400  | Epic      | None     |
| 8  | Weekend Adventure   | 750  | Epic      | 2 weeks  |
| 9  | Major Purchase      | 1500 | Legendary | None     |
| 10 | Epic Reward         | 3000 | Legendary | 30 days  |

### Purchasing Rewards

Buy by ID:
```bash
taskquest buy 1
```

Buy by name:
```bash
taskquest buy "Coffee Break"
```

Successful purchase shows:
- Reward name
- Cost deducted
- Remaining gold
- Any achievements unlocked

### Custom Rewards (Phase 3)

**Add custom rewards**:
```bash
# Basic syntax
taskquest add-reward "Name" <cost> "Description"

# With tier (normal/heroic/epic/legendary)
taskquest add-reward "Spa Day" 800 "Relaxing spa treatment" --tier epic

# With cooldown (in hours)
taskquest add-reward "Weekend Trip" 2000 "Weekend getaway" --tier epic --cooldown 168
```

**Examples:**
```bash
# Quick reward, no cooldown
taskquest add-reward "Coffee" 50 "Nice coffee from favorite cafe"

# Epic reward with 1-week cooldown
taskquest add-reward "Day Off" 1000 "Take a guilt-free day off" --tier epic --cooldown 168

# Legendary reward with 1-month cooldown
taskquest add-reward "Vacation" 5000 "One week vacation" --tier legendary --cooldown 720
```

**Remove custom rewards**:
```bash
# By ID
taskquest remove-reward 11

# By name
taskquest remove-reward "Spa Day"
```

**Safety**: Cannot remove default rewards (IDs 1-10)

---

## Achievements

TaskQuest has **29 built-in achievements** across 8 categories.

### View Achievements

```bash
taskquest achievements
```

Shows:
- All unlocked achievements grouped by tier
- Achievement icons and descriptions
- Overall completion percentage

### Achievement Categories

#### 1. Milestones
- **First Steps** - Complete your first quest
- **The Journey Begins** - Reach level 5
- **Seasoned Adventurer** - Complete 100 quests
- **Veteran Hero** - Complete 500 quests
- **Legendary Warrior** - Reach level 30
- **Living Legend** - Reach level 50
- **Completionist** - Complete 1000 quests

#### 2. Variety
- **Master of Balance** - Complete at least one task of each difficulty (1-10)
- **The Undaunted** - Complete 10 difficulty-10 quests
- **Jack of All Trades** - Complete at least 20 tasks of each difficulty

#### 3. Long-term Progress
- **Consistent Hero** - Be active for 30 different days
- **Marathon Hero** - Be active for 100 different days
- **Renaissance Soul** - Complete tasks in 10 different projects

#### 4. Comeback
- **Phoenix Rising** - Complete 5 quests after a 30+ day break

#### 5. Loot & Rewards
- **Epic Collector** - Receive an Epic-tier loot drop
- **Gold Hoarder** - Accumulate 5000 gold
- **Wise Spender** - Purchase 10 rewards from the shop
- **Treasure Hunter** - Receive 50 loot drops

#### 6. Timing
- **Early Riser** - Complete 50 tasks early (>24hrs before due)
- **Pressure Handler** - Complete 25 tasks within grace period
- **Time Master** - Complete 100 tasks with a due date set
- **Punctual Perfectionist** - Complete 100 tasks on time

#### 7. Power (Stat Milestones)
- **Strength Incarnate** - Reach 100 STR
- **Lightning Reflexes** - Reach 100 DEX
- **Iron Constitution** - Reach 100 CON
- **Brilliant Mind** - Reach 100 INT
- **Sage Wisdom** - Reach 100 WIS
- **Magnetic Personality** - Reach 100 CHA

#### 8. Legendary Stats
- **Legendary Strength** - Reach 500 in any stat
- **Transcendent Power** - Reach 1000 in any stat

### Achievement Tiers

- **Common** (white) - Easy to get, early-game
- **Uncommon** (green) - Regular play milestones
- **Rare** (blue) - Significant achievements
- **Epic** (magenta) - Long-term dedication
- **Legendary** (yellow) - Ultimate goals

---

## Commands Reference

### Character Management

```bash
# Initialize TaskQuest (run once)
taskquest init

# View character status (stats, level, gold, XP, avatar)
taskquest status

# View detailed stats (with progress bars)
taskquest stats

# Change character name
taskquest name "NewName"

# Change character class (cosmetic only, affects avatar)
taskquest class Ranger
```

### Shop & Rewards

```bash
# View shop
taskquest shop

# Buy reward by ID
taskquest buy 1

# Buy reward by name
taskquest buy "Coffee Break"

# Add custom reward
taskquest add-reward "Spa Day" 800 "Relaxing spa treatment" --tier epic --cooldown 168

# Remove custom reward (cannot remove IDs 1-10)
taskquest remove-reward 11
taskquest remove-reward "Spa Day"
```

### Achievements

```bash
# View achievements
taskquest achievements
```

### Sync Commands (Git-Based)

```bash
# Initialize git repository for syncing
taskquest sync init

# Initialize with remote repository
taskquest sync init --remote git@github.com:username/taskquest-data.git

# Push changes to remote
taskquest sync push

# Pull changes from remote
taskquest sync pull

# Show sync status
taskquest sync status

# View sync history
taskquest sync history
taskquest sync history 20  # Last 20 commits
```

### Help

```bash
# Show help
taskquest --help

# Show version
taskquest --version
```

---

## Taskwarrior Integration

TaskQuest enhances Taskwarrior with custom User Defined Attributes (UDAs).

### The Challenge UDA

When adding tasks, set the challenge level (1-10):

```bash
task add "Simple task" challenge:2
task add "Medium task" challenge:5
task add "Brutal task" challenge:10
```

**If you don't set challenge**, it defaults to 5.

### Challenge Guidelines

Think of challenge as "mental/physical effort required":

- **1-2**: Trivial tasks (< 5 minutes, minimal thought)
- **3-4**: Simple tasks (5-15 minutes, straightforward)
- **5-6**: Standard tasks (15-60 minutes, some complexity)
- **7-8**: Complex tasks (1-3 hours, significant effort)
- **9-10**: Epic tasks (3+ hours, major undertaking)

### The Stat Training UDAs (stat1 and stat2)

**IMPORTANT**: To train stats, you must specify `stat1` and `stat2` on your tasks!

```bash
task add "Code review" challenge:6 stat1:int stat2:wis
task add "Gym workout" challenge:5 stat1:str stat2:con
task add "Write proposal" challenge:7 stat1:int stat2:cha
```

**Valid stat values**: `str`, `dex`, `con`, `int`, `wis`, `cha` (case insensitive)

- **stat1** - Primary ability trained (receives 66% of gains: challenge × 2)
- **stat2** - Secondary ability trained (receives 33% of gains: challenge × 1)

**Choosing stats for tasks:**

- **Coding/Development**: `stat1:int stat2:wis` (or `dex` for speed-focused)
- **Physical/Fitness**: `stat1:str stat2:con` (or `dex` for agility work)
- **Social/Meetings**: `stat1:cha stat2:wis` (or `int` for technical discussions)
- **Planning/Strategy**: `stat1:wis stat2:int`
- **Creative Work**: `stat1:int stat2:cha` (or `dex` for execution)
- **Endurance/Long Projects**: `stat1:con stat2:int` (or relevant stat)

If you don't set stat1/stat2, the task won't train any stats (but still gives XP and gold)!

### Taskwarrior Commands (Quick Reference)

```bash
# Add a task (with TaskQuest UDAs)
task add "Task description"
task add "Code feature" challenge:7 stat1:int stat2:wis
task add "Gym session" challenge:5 stat1:str stat2:con project:Fitness
task add "Team meeting" challenge:4 stat1:cha stat2:wis due:tomorrow
task add "Write docs" challenge:6 stat1:int stat2:cha +urgent +work

# View tasks
task                    # List pending tasks
task next               # Show next tasks by urgency
task list               # Detailed list
task project:Work       # Filter by project
task +urgent            # Filter by tag

# Complete tasks
task 1 done            # Complete task 1
task 1,3,5 done        # Complete multiple tasks

# Modify tasks
task 1 modify challenge:8
task 1 modify stat1:wis stat2:int
task 1 modify due:friday
task 1 modify +urgent

# Delete/Undo
task 1 delete          # Delete task
task undo              # Undo last action

# Search
task /keyword/         # Search by keyword

# Projects
task projects          # List all projects

# Reports
task summary           # Project summary
task completed         # Show completed tasks
```

---

## Best Practices

### Task Creation

1. **Always set challenge** - It determines your XP, gold, and stat gains!
2. **Always set stat1 and stat2** - Otherwise you won't train any stats!
3. **Choose stats thoughtfully** - Think about what abilities the task actually uses
4. **Use projects** - Track tasks across different areas (Work, Personal, Fitness, etc.)
5. **Set due dates** - Get timing bonuses/penalties (early = 1.3× XP!)
6. **Use tags** - Organize with +urgent, +work, +home, etc.

### Character Management

7. **Review regularly** - Run `task next` to see what's most important
8. **Check your stats** - Run `taskquest stats` to see which areas you're neglecting
9. **View avatar evolution** - Your avatar changes at levels 10, 20, 30, 40+
10. **Track achievements** - Run `taskquest achievements` to see progress

### Shop Strategy

11. **Save for big rewards** - Epic/Legendary rewards feel more satisfying
12. **Use cooldowns wisely** - Add cooldowns to prevent overindulgence
13. **Customize rewards** - Add rewards that actually motivate YOU
14. **Remove unused rewards** - Keep shop focused on what you want

### Sync Workflow

15. **Sync regularly** - Use `taskquest sync push/pull` frequently
16. **Commit meaningful changes** - Each push includes a timestamp
17. **Check sync status** - Use `taskquest sync status` to verify
18. **View history** - Use `taskquest sync history` to see progression

### Example Workflow

```bash
# Morning planning
task add "Plan daily goals" challenge:4 stat1:wis stat2:int project:Personal

# Work tasks
task add "Fix critical bug" challenge:8 stat1:int stat2:dex project:Work due:today +urgent
task add "Code review" challenge:5 stat1:int stat2:wis project:Work

# Fitness
task add "Gym workout" challenge:6 stat1:str stat2:con project:Fitness

# Personal development
task add "Study Rust" challenge:7 stat1:int stat2:wis project:Learning due:friday

# Complete tasks throughout the day
task 1 done
task 2 done
# ... etc

# Evening review
taskquest status        # Check progress
taskquest shop          # See if you can afford rewards
taskquest sync push     # Sync to cloud

# Weekly review
taskquest stats         # Review stat distribution
taskquest achievements  # Check achievement progress
```

---

## Troubleshooting

### Hooks Not Running

**Problem**: Tasks complete but no XP/gold awarded

**Solutions**:

1. Check hooks exist:
   ```bash
   ls -la ~/.task/hooks/
   ```

2. Ensure hooks are executable:
   ```bash
   chmod +x ~/.task/hooks/on-*-taskquest
   ```

3. Re-run init:
   ```bash
   taskquest init --skip-wizard
   ```

### Git Sync Issues

**Problem**: Push/pull fails with authentication errors

**Solutions**:

1. Set up SSH keys (see [Git-Based Sync](#git-based-sync-recommended))

2. Or use HTTPS with personal access token:
   ```bash
   # Generate token on GitHub: Settings → Developer settings → Personal access tokens
   # Use token as password when pushing
   ```

3. Check remote is configured:
   ```bash
   cd ~/.taskquest
   git remote -v
   ```

### Syncthing Conflicts

**Problem**: Syncthing created `.sync-conflict` files

**Solution**:

1. Stop syncing temporarily
2. Check timestamps on conflicted files
3. Keep the newest version:
   ```bash
   cd ~/.taskquest
   # Check modification times
   ls -lt character.json*
   # Keep the newest, delete conflict
   rm character.json.sync-conflict-*
   ```
4. Resume syncing

### Character Data Corrupted

**Problem**: Character file won't load

**Solution**:

1. Check for backup:
   ```bash
   ls ~/.taskquest/character.json.bak
   ```

2. Restore backup:
   ```bash
   cp ~/.taskquest/character.json.bak ~/.taskquest/character.json
   ```

3. If using git sync, rollback:
   ```bash
   cd ~/.taskquest
   git log  # Find good commit
   git checkout <commit-hash> -- character.json
   ```

4. If no backup, recreate character:
   ```bash
   rm ~/.taskquest/character.json
   taskquest init
   ```

### Termux Permission Issues

**Problem**: Can't write to files

**Solution**:

1. Grant storage permissions:
   - Android Settings → Apps → Termux → Permissions → Storage → Allow

2. Check file permissions:
   ```bash
   ls -la ~/.taskquest/
   chmod 644 ~/.taskquest/*.json
   ```

### Command Not Found

**Problem**: `taskquest: command not found`

**Solution**:

1. Check if installed:
   ```bash
   ls ~/.cargo/bin/taskquest
   ```

2. Add to PATH:
   ```bash
   echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc
   source ~/.bashrc
   ```

3. Or use full path:
   ```bash
   ~/.cargo/bin/taskquest status
   ```

---

## Advanced Usage

### Custom Shop Rewards

You can create unlimited custom rewards:

```bash
# Seasonal rewards
taskquest add-reward "Holiday Break" 2000 "Week off for holidays" --tier legendary --cooldown 8760

# Progressive rewards
taskquest add-reward "Mini Vacation" 1500 "Long weekend trip" --tier epic --cooldown 720
taskquest add-reward "Spa Day" 800 "Relaxing spa treatment" --tier epic --cooldown 168
taskquest add-reward "Nice Dinner" 300 "Dinner at favorite restaurant" --tier heroic

# Quick rewards
taskquest add-reward "Coffee" 50 "Nice coffee" --tier normal
taskquest add-reward "Snack" 30 "Favorite snack" --tier normal
```

### Stats Analysis

View your character's raw stats:
```bash
cat ~/.taskquest/character.json | grep -A6 '"stats"'
```

Track your stat growth over time with git history:
```bash
cd ~/.taskquest
git log -p character.json | grep -A6 '"stats"'
```

### Achievement Progress Tracking

Check specific achievement progress:
```bash
cat ~/.taskquest/achievements.json | jq '.progress'
```

### Git History Analysis

View character progression over time:
```bash
cd ~/.taskquest
git log --oneline --all
git show <commit-hash>:character.json | jq '.level, .total_xp, .stats'
```

### Rollback Character State

Restore to previous state:
```bash
cd ~/.taskquest
git log  # Find desired commit
git checkout <commit-hash> -- character.json
taskquest status  # Verify
taskquest sync push  # Save rollback
```

### Multiple Characters (Advanced)

Maintain multiple characters by swapping directories:

```bash
# Backup current character
mv ~/.taskquest ~/.taskquest_warrior

# Create new character
taskquest init

# Switch back
mv ~/.taskquest ~/.taskquest_ranger
mv ~/.taskquest_warrior ~/.taskquest
```

---

## Design Philosophy

TaskQuest follows a **non-toxic gamification** approach:

### 1. No Punishment
- Tasks completed late still give XP (just reduced)
- Grace period (24 hours) for late tasks
- No stat loss or character damage
- No "streak breaking" penalties

### 2. Organic Progression
- Level curve feels natural (inspired by World of Warcraft)
- Stats grow based on what you actually do (task-based training)
- Your character reflects your real work and life balance
- Achievements reward long-term dedication, not daily grinding

### 3. Positive Reinforcement
- Beautiful notifications celebrate completion
- Loot drops create excitement
- Achievements recognize diverse playstyles
- Shop rewards encourage saving and spending

### 4. Flexibility
- No required daily tasks
- Can play on/offline
- Can sync or play standalone
- Stats reflect your actual work (not locked to a class)
- Purely cosmetic class choice for roleplay flavor

---

## Data Storage

All TaskQuest data is stored in plain JSON files for transparency and portability.

### File Locations

```
~/.taskquest/
├── character.json          # Character data
├── character.json.bak      # Automatic backup
├── achievements.json       # Achievement progress
├── achievements.json.bak   # Automatic backup
├── shop.json               # Shop and purchase history
├── shop.json.bak           # Automatic backup
└── .git/                   # Git repository (if using git sync)

~/.task/
├── pending.data            # Taskwarrior pending tasks
├── completed.data          # Taskwarrior completed tasks
├── undo.data               # Taskwarrior undo history
├── .taskrc                 # Taskwarrior config (includes UDAs)
└── hooks/
    ├── on-add-taskquest    # Task creation hook
    ├── on-modify-taskquest # Task completion hook (awards XP/gold)
    └── on-exit-taskquest   # Post-execution hook
```

### Backup Strategy

TaskQuest automatically creates `.bak` backups before writing. For additional safety:

```bash
# Manual backup
cp -r ~/.taskquest ~/.taskquest_backup_$(date +%Y%m%d)

# Restore from backup
cp -r ~/.taskquest_backup_20251119 ~/.taskquest
```

With git sync, you have full version history:
```bash
cd ~/.taskquest
git log --oneline  # View all commits
git checkout <commit> -- character.json  # Restore specific file
```

---

## What's New

### Phase 3 Features (Latest)

**Custom Reward Management**:
- Add unlimited custom rewards with `add-reward`
- Remove unwanted rewards with `remove-reward`
- Set custom tiers and cooldowns

**Progress Analytics**:
- Visual progress bars for all stats
- Color-coded stat bars (green/yellow/red)
- Enhanced XP progress display

**Git-Based Sync**:
- Superior to Syncthing (version control + rollback)
- Simple commands: `sync init/push/pull/status/history`
- Free cloud hosting with GitHub/GitLab
- Better conflict resolution

See [PHASE3_FEATURES.md](./PHASE3_FEATURES.md) for details.

### Phase 2 Features

**Loot System**: Random drops (30-50% chance)
**Shop System**: 10 default rewards + custom rewards
**Achievement System**: 29 achievements across 8 categories
**Avatar System**: ASCII art that evolves with level

### Phase 1 Features (MVP)

**Character System**: 6 D&D stats, task-based training
**Progression**: XP, levels, gold
**Taskwarrior Integration**: Hooks, UDAs
**Non-toxic Design**: No punishment, only reduced rewards

---

## Credits

TaskQuest is built with:
- **Rust** - Fast, safe systems programming
- **Taskwarrior** - Powerful task management
- **Serde** - JSON serialization
- **Colored** - Terminal colors
- **Chrono** - Date/time handling
- **Clap** - CLI argument parsing
- **Git2** - Git operations

Inspired by:
- Habitica (gamified task management)
- World of Warcraft (leveling curve)
- D&D 5e (stat system)

---

## License

MIT License - See LICENSE file for details

---

## Support & Contributing

### Found a Bug?

Report issues with:
- Your OS and version
- TaskQuest version (`taskquest --version`)
- Steps to reproduce
- Error messages

### Feature Requests

TaskQuest is designed to be extensible. Future features might include:
- Auto-sync after task completion
- Smart JSON merging
- Statistics dashboard
- Achievement unlock dates
- Reward pack sharing

---

**Happy Questing! ⚔️**

May your tasks be many and your XP plentiful!

For quick reference, see [TLDR.md](./TLDR.md)
