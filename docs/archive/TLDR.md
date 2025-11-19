# TaskQuest - TLDR Quick Reference

**Quick command reference for TaskQuest and Taskwarrior**

---

## Quick Start

```bash
# Install (after building)
cargo install --path .

# Initialize
taskquest init

# Check status
taskquest status
```

---

## TaskQuest Commands

### Character

```bash
taskquest status              # View character overview
taskquest stats               # View detailed stats
taskquest name "Aragorn"      # Change name
taskquest class Ranger        # Change class (cosmetic only)
```

### Shop & Rewards

```bash
taskquest shop                # List available rewards
taskquest buy 1               # Buy by ID
taskquest buy "Coffee Break"  # Buy by name
```

### Achievements

```bash
taskquest achievements        # View unlocked achievements
```

---

## Taskwarrior Commands

### Adding Tasks

```bash
# Basic
task add "Write documentation"

# With challenge (1-10, affects XP/gold/stats)
task add "Fix critical bug" challenge:9

# With stats training (stat1 gets 66%, stat2 gets 33%)
task add "Code review" challenge:6 stat1:int stat2:wis
task add "Gym workout" challenge:5 stat1:str stat2:con
task add "Team meeting" challenge:4 stat1:cha stat2:wis

# With due date (affects XP multiplier)
task add "Submit report" due:tomorrow challenge:6 stat1:int stat2:wis

# With project
task add "Code review" project:Work challenge:5 stat1:int stat2:dex

# With tags
task add "Buy groceries" +personal +urgent challenge:3

# Everything together
task add "Deploy to production" project:DevOps challenge:10 stat1:int stat2:wis due:friday +urgent
```

**Challenge Guidelines:**
- 1-2: < 5 min, trivial
- 3-4: 5-15 min, simple
- 5-6: 15-60 min, standard
- 7-8: 1-3 hrs, complex
- 9-10: 3+ hrs, epic

**Stats Guide:**
- **STR** (Strength) - Physical tasks, heavy lifting, demanding work
- **DEX** (Dexterity) - Quick tasks, agility, adaptability
- **CON** (Constitution) - Endurance, long-term projects, resilience
- **INT** (Intelligence) - Mental tasks, problem-solving, learning
- **WIS** (Wisdom) - Planning, insight, strategic thinking
- **CHA** (Charisma) - Social tasks, communication, leadership

### Viewing Tasks

```bash
task                          # List pending tasks
task next                     # Next tasks by urgency
task list                     # Detailed list
task all                      # All tasks (including completed)
task completed                # Completed tasks only

# Filters
task project:Work             # By project
task +urgent                  # By tag
task due.before:tomorrow      # Due soon
task /keyword/                # Search
```

### Completing Tasks

```bash
task 1 done                   # Complete task 1
task 1,3,5 done              # Complete multiple
```

**→ This triggers TaskQuest XP/gold/stat/loot rewards! 🎉**

### Modifying Tasks

```bash
task 1 modify challenge:8           # Change challenge
task 1 modify stat1:int stat2:wis   # Change stats trained
task 1 modify due:friday            # Change due date
task 1 modify project:Personal      # Change project
task 1 modify +urgent               # Add tag
task 1 modify -urgent               # Remove tag
task 1 modify "New description"     # Change description
```

### Deleting & Undoing

```bash
task 1 delete                 # Delete task
task 1 delete rc.confirmation:no    # Delete without confirm
task undo                     # Undo last action
```

### Projects & Organization

```bash
task projects                 # List all projects
task summary                  # Project summary with counts
task tags                     # List all tags
```

### Advanced Queries

```bash
# Tasks due this week
task due.before:eow

# Overdue tasks
task +OVERDUE

# High priority in specific project
task project:Work priority:H

# Tasks without due date
task due:

# Tasks added today
task entry.after:today

# Completed this week
task end.after:sow status:completed
```

### Reports & Summaries

```bash
task summary                  # Overview by project
task burndown.daily          # Burndown chart
task calendar                # Calendar view
task stats                   # Statistics
```

---

## Task Examples by Activity Type

### Coding/Development

```bash
task add "Implement authentication" challenge:8 stat1:int stat2:wis
task add "Fix production bug" challenge:7 stat1:int stat2:dex due:today
task add "Code review" challenge:5 stat1:int stat2:wis +work
task add "Refactor legacy code" challenge:9 stat1:int stat2:con
task add "Write unit tests" challenge:6 stat1:int stat2:wis
```

### Physical/Fitness

```bash
task add "Gym session" challenge:6 stat1:str stat2:con
task add "Run 5K" challenge:5 stat1:con stat2:dex
task add "Yoga class" challenge:4 stat1:dex stat2:wis
task add "Rock climbing" challenge:7 stat1:str stat2:dex
task add "Heavy lifting day" challenge:8 stat1:str stat2:con
```

### Social/Communication

```bash
task add "Present to stakeholders" challenge:7 stat1:cha stat2:wis
task add "Difficult conversation" challenge:6 stat1:cha stat2:wis
task add "Team building event" challenge:4 stat1:cha stat2:dex
task add "Networking event" challenge:5 stat1:cha stat2:wis
task add "Lead team meeting" challenge:6 stat1:cha stat2:wis
```

### Learning/Study

```bash
task add "Read technical book chapter" challenge:5 stat1:int stat2:wis
task add "Complete online course module" challenge:6 stat1:int stat2:con
task add "Practice algorithm problems" challenge:7 stat1:int stat2:dex
task add "Study for certification" challenge:8 stat1:int stat2:wis due:friday
task add "Learn new framework" challenge:7 stat1:int stat2:con
```

### Planning/Strategy

```bash
task add "Plan Q4 roadmap" challenge:7 stat1:wis stat2:int
task add "Design system architecture" challenge:8 stat1:int stat2:wis
task add "Strategic planning session" challenge:6 stat1:wis stat2:cha
task add "Analyze project metrics" challenge:5 stat1:wis stat2:int
task add "Create process documentation" challenge:6 stat1:int stat2:wis
```

### Household/Life

```bash
task add "Deep clean house" challenge:7 stat1:str stat2:con
task add "Organize garage" challenge:8 stat1:str stat2:wis
task add "Meal prep for week" challenge:5 stat1:dex stat2:wis
task add "Handle insurance paperwork" challenge:4 stat1:int stat2:wis
task add "Fix broken appliance" challenge:6 stat1:dex stat2:int
```

---

## Quick Workflow Examples

### Daily Development Workflow

```bash
# Morning: Check what's up
task next

# Add today's tasks with appropriate stats
task add "Team standup" project:Work challenge:3 stat1:cha stat2:wis due:10am
task add "Implement feature X" project:Work challenge:8 stat1:int stat2:wis due:5pm
task add "Review PRs" project:Work challenge:5 stat1:int stat2:wis

# Throughout the day: Complete tasks
task 1 done    # ⚔️ QUEST COMPLETE! +30 XP, +15 Gold, +6 CHA, +3 WIS
task 2 done    # ⚔️ QUEST COMPLETE! +104 XP, +40 Gold, +16 INT, +8 WIS, 💎 Loot!
task 3 done    # ⚔️ QUEST COMPLETE! +65 XP, +25 Gold, +10 INT, +5 WIS

# Evening: Check progress
taskquest status    # See your stat growth!
taskquest achievements
```

### Balanced Life Example

```bash
# Add varied tasks to train all stats
task add "Code feature" challenge:7 stat1:int stat2:wis project:Work
task add "Gym workout" challenge:6 stat1:str stat2:con +health
task add "Team presentation" challenge:6 stat1:cha stat2:wis project:Work
task add "Read book chapter" challenge:4 stat1:int stat2:wis +personal
task add "Organize closet" challenge:5 stat1:dex stat2:str +home

# After a week, review your stats
taskquest stats
# You'll see which areas you've been working on (and which you've neglected!)
```

---

## Syncing with Syncthing (Quick Setup)

### Linux

```bash
# Install
sudo dnf install syncthing  # Fedora
sudo apt install syncthing  # Debian/Ubuntu

# Run
syncthing

# Access Web UI: http://localhost:8384
```

### Termux

```bash
# Install
pkg install syncthing

# Run
syncthing

# Access Web UI at displayed IP
```

### Sync These Directories

```bash
~/.taskquest/     # TaskQuest data (character, achievements, shop)
~/.task/          # Taskwarrior data (tasks, config, hooks)
```

**After syncing hooks, make executable:**
```bash
chmod +x ~/.task/hooks/on-*-taskquest
```

---

## Progression Cheat Sheet

### XP Formula
```
XP = challenge × 10 × urgency_mult × timing_mult

Timing multipliers:
  Early (>24h before due): 1.3×
  On-time (day of due):    1.0×
  Grace (<24h late):       0.8×
  Late (>24h late):        0.5×
  No due date:             1.0×
```

### Gold Formula
```
Gold = challenge × 5 ± 20%

Examples:
  Challenge 2: 8-12 gold
  Challenge 5: 20-30 gold
  Challenge 10: 40-60 gold
```

### Loot Drop Chance
```
Chance = 30% + (challenge × 2%)

Examples:
  Challenge 1: 32%
  Challenge 5: 40%
  Challenge 10: 50%
```

### Loot Types (Weighted)
- 70% - Bonus gold (10-50)
- 20% - Normal reward
- 8% - Heroic reward
- 2% - Epic reward

### Stat Growth (NEW!)
```
Total gain per task: challenge × 3

stat1 gets: challenge × 2 (66%)
stat2 gets: challenge × 1 (33%)

Example (challenge 6):
  Total: 18 stat points
  stat1: +12 points
  stat2: +6 points
```

### Level XP Requirements
```
Level 1:  0 XP
Level 2:  428 XP
Level 5:  2,936 XP
Level 10: 15,849 XP
Level 20: 82,299 XP
Level 30: 216,632 XP
Level 50: 1,119,134 XP
```

---

## Achievement Quick List

### Easy to Get
- ⚔️ First Steps - Complete 1 quest
- 🎯 The Journey Begins - Level 5
- ⚖️ Master of Balance - Complete 1 task of each difficulty (1-10)
- 🛒 Wise Spender - Buy 10 shop rewards

### Medium Term
- 🏆 Seasoned Adventurer - 100 quests
- 📅 Consistent Hero - Active 30 different days
- 💪 The Undaunted - 10 difficulty-10 quests
- 🗝️ Treasure Hunter - 50 loot drops

### Long Term
- 👑 Veteran Hero - 500 quests
- ⚡ Legendary Warrior - Level 30
- 🏃 Marathon Hero - Active 100 different days
- 💰 Gold Hoarder - 5000 gold

### Epic Goals
- 📜 Completionist - 1000 quests
- 💫 Living Legend - Level 50
- 🎭 Jack of All Trades - 20 tasks of each difficulty
- 🌟 Legendary Strength - 500 in any stat
- 👑 Transcendent Power - 1000 in any stat

---

## Shop Quick Reference

| ID | Reward            | Cost | Cooldown |
|----|-------------------|------|----------|
| 1  | Coffee Break      | 50   | -        |
| 2  | Gaming Session    | 100  | -        |
| 3  | Movie Night       | 150  | -        |
| 4  | Treat Meal        | 200  | -        |
| 5  | New Book          | 300  | -        |
| 6  | Day Off           | 500  | 1 week   |
| 7  | Hobby Supplies    | 400  | -        |
| 8  | Weekend Adventure | 750  | 2 weeks  |
| 9  | Major Purchase    | 1500 | -        |
| 10 | Epic Reward       | 3000 | 30 days  |

---

## Classes Quick Reference

**NOTE: Class is now cosmetic/roleplay only - it doesn't affect stat growth!**

| Class   | Description                    |
|---------|--------------------------------|
| Rogue   | Quick, versatile, cunning      |
| Ranger  | Tactical, nature-focused       |
| Warrior | Strong, resilient              |
| Paladin | Powerful, inspirational        |
| Monk    | Agile, disciplined, wise       |

**Your stats grow based on the tasks you complete, not your class!**

---

## Troubleshooting Quick Fixes

```bash
# Hooks not working
chmod +x ~/.task/hooks/on-*-taskquest
taskquest init --skip-wizard

# Command not found
export PATH="$HOME/.cargo/bin:$PATH"
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc

# View raw data
cat ~/.taskquest/character.json
cat ~/.taskquest/achievements.json

# Restore backup
cp ~/.taskquest/character.json.bak ~/.taskquest/character.json

# Reset character (destructive!)
rm -rf ~/.taskquest
taskquest init
```

---

## Useful Taskwarrior Aliases

Add to `~/.bashrc` or `~/.zshrc`:

```bash
# Quick task shortcuts
alias t='task'
alias tn='task next'
alias ta='task add'
alias td='task done'
alias ts='taskquest status'
alias tsh='taskquest shop'
alias tach='taskquest achievements'

# Project shortcuts
alias tw='task project:Work'
alias th='task project:Home'
alias tp='task project:Personal'

# Task templates with stats
tcode() { task add "$1" challenge:${2:-6} stat1:int stat2:wis; }
tgym() { task add "$1" challenge:${2:-5} stat1:str stat2:con; }
tsocial() { task add "$1" challenge:${2:-5} stat1:cha stat2:wis; }
tlearn() { task add "$1" challenge:${2:-6} stat1:int stat2:wis; }
```

**Usage:**
```bash
t                    # List tasks
ta "New task"        # Add task
td 1                 # Complete task 1
tcode "Implement API" 8     # Add coding task, challenge 8
tgym "Workout"              # Add gym task, default challenge 5
tsocial "Team meeting" 4    # Add social task, challenge 4
ts                   # TaskQuest status
```

---

## Pro Tips

### See Which Areas You're Improving

```bash
# Check your stats regularly
taskquest stats

# High INT/WIS? You're doing lots of mental work
# High STR/CON? You're doing physical tasks
# Low CHA? Maybe add more social/communication tasks
# Balanced stats? You're living a well-rounded life!
```

### Maximize XP
- Set due dates (get timing multipliers)
- Complete tasks early (1.3× bonus)
- Do high-challenge tasks (10× base XP)
- Let urgency build naturally

### Maximize Gold
- Focus on high challenge tasks
- Gold = challenge × 5 ± 20%
- Challenge 10 tasks give 40-60 gold each

### Maximize Loot
- Higher challenge = better drop rate
- Challenge 10 = 50% drop chance
- Epic loot is 2% of drops

### Strategic Stat Growth
- Want to improve INT? Do coding/learning tasks with stat1:int
- Want balanced stats? Vary your stat1/stat2 assignments
- Track weak stats and intentionally train them
- Your stats reflect your actual life activities!

---

## One-Liner Examples

```bash
# Add a full day of varied tasks
task add "Code feature" c:7 stat1:int stat2:wis && \
task add "Gym" c:5 stat1:str stat2:con && \
task add "Team meeting" c:4 stat1:cha stat2:wis && \
task add "Read chapter" c:5 stat1:int stat2:wis

# Complete all tasks for today
task due:today done

# View stats that increased most
taskquest stats | grep ":" | sort -t: -k2 -rn | head -3

# Backup all data
tar -czf taskquest_backup_$(date +%Y%m%d).tar.gz ~/.taskquest ~/.task

# View recently completed tasks with their stats
task end.after:today status:completed
```

---

**Happy Questing! ⚔️**

*Complete tasks. Train stats. Level up your life.*

---

## Key Differences from Old System

### ❌ OLD (Wrong)
- Character had fixed primary/secondary stats
- All tasks increased the same two stats
- Class determined stat growth
- No task-level stat specification

### ✅ NEW (Correct)
- Each task specifies stat1 and stat2
- stat1 gets 66% growth, stat2 gets 33%
- Stats reflect actual work you're doing
- Class is cosmetic/roleplay only
- See which life areas you're focusing on!
