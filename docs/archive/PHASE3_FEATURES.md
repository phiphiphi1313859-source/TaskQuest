# Phase 3 Features - Progress Analytics & Sync

**Implementation Date**: November 19, 2025
**Status**: ✅ Complete

---

## Overview

Phase 3 adds three major enhancements to TaskQuest:
1. **Custom Reward Management** - Add/remove custom rewards
2. **Progress Analytics** - Visual progress bars for stats and XP
3. **Git-Based Sync** - Superior syncing with version control

---

## 1. Custom Reward Management

### New Commands

#### Add Custom Reward
```bash
taskquest add-reward "Spa Day" 800 "Relaxing spa treatment" --tier epic --cooldown 168
```

**Parameters**:
- `name` - Reward name (required)
- `cost` - Gold cost (required)
- `description` - What the reward is (required)
- `tier` - normal/heroic/epic/legendary (default: normal)
- `--cooldown` - Hours before can purchase again (default: 0)

**Example**:
```bash
# Quick reward, no cooldown
taskquest add-reward "Coffee" 50 "Nice coffee from favorite cafe"

# Epic reward with 1-week cooldown
taskquest add-reward "Day Off" 1000 "Take a guilt-free day off" --tier epic --cooldown 168

# Legendary reward with 1-month cooldown
taskquest add-reward "Weekend Trip" 5000 "Weekend getaway" --tier legendary --cooldown 720
```

#### Remove Reward
```bash
# By ID
taskquest remove-reward 11

# By name
taskquest remove-reward "Coffee"
```

**Safety**: Cannot remove default rewards (IDs 1-10)

### Use Cases

1. **Personalized Rewards** - Add rewards that motivate YOU
2. **Seasonal Rewards** - Add holiday-specific treats
3. **Progressive Rewards** - Add expensive long-term goals
4. **Clean Unused Rewards** - Remove rewards you never buy

### Output

**Add Reward**:
```
╔════════════════════════════════════════╗
║      ✨ REWARD ADDED! ✨              ║
╠════════════════════════════════════════╣
║ ID: 11                                 ║
║ Name: Spa Day                          ║
║ Cost: 800 Gold                         ║
║ Tier: epic                             ║
║ Cooldown: 168 hours                    ║
╚════════════════════════════════════════╝
```

**Remove Reward**:
```
╔════════════════════════════════════════╗
║      🗑️  REWARD REMOVED  🗑️            ║
╠════════════════════════════════════════╣
║ Removed: Spa Day                       ║
╚════════════════════════════════════════╝
```

---

## 2. Progress Analytics

### Enhanced Displays

#### XP Progress Bar (Already in status)
```
║ XP Progress: [████████████████████░░░░░░░░░░░░░░░░░░░░░░░░░░] ║
```

**Features**:
- 50-character width bar
- Shows progress to next level
- Green color for visual appeal
- Percentage display

#### Stat Progress Bars (New in `taskquest stats`)
```
╔══════════════════════════════════════════════════════╗
║              DETAILED STATISTICS                     ║
╠══════════════════════════════════════════════════════╣
║ ABILITY SCORES:                                      ║
║   Strength:     150 [████████████████████] ║
║   Dexterity:     80 [████████░░░░░░░░░░░░] ║
║   Constitution: 120 [██████████████░░░░░░] ║
║   Intelligence:  90 [█████████░░░░░░░░░░░] ║
║   Wisdom:        70 [███████░░░░░░░░░░░░░] ║
║   Charisma:      60 [██████░░░░░░░░░░░░░░] ║
╠══════════════════════════════════════════════════════╣
║ PROGRESSION:                                         ║
║   XP Progress:  [████████████████░░░░░░░░░░░░░░] 58.3% ║
╚══════════════════════════════════════════════════════╝
```

### Stat Bar Features

**Relative Scaling**:
- Bars scale relative to your highest stat
- Shows stat distribution at a glance
- Minimum scale of 100 for early game

**Color Coding**:
- 🟩 Green: At max (100% of highest)
- 🟨 Yellow: 50-99% of highest
- 🟥 Red: 0-49% of highest

**Benefits**:
- **Quick Overview**: See stat balance instantly
- **Identify Gaps**: Red bars = neglected areas
- **Track Growth**: Watch bars fill over time
- **Life Balance**: See which activities you're doing/avoiding

### XP Progress Analytics

Enhanced XP display in `taskquest stats`:
```
║   Total XP:     15849                                ║
║   Current XP:   15849                                ║
║   Next Level:   2450 XP                              ║
║   XP Progress:  [████████████████░░░░░░░░░░░░░░] 58.3% ║
```

**Improvements**:
- Visual progress bar (30 characters)
- Cyan color for distinction
- Percentage display
- Clear XP metrics

---

## 3. Git-Based Sync (Recommended)

### Why Git > Syncthing

See [SYNC_ANALYSIS.md](./SYNC_ANALYSIS.md) for full comparison.

**Summary**:
- ✅ **Version Control** - Full history of character
- ✅ **Rollback** - Restore any previous state
- ✅ **Better Conflicts** - Intelligent merge vs .sync-conflict files
- ✅ **Offline-First** - Commit locally, push later
- ✅ **Free Cloud** - GitHub/GitLab private repos
- ✅ **Technical Users** - Fits CLI-comfortable user base

### Setup

#### Initialize Repository
```bash
taskquest sync init
```

**What it does**:
- Creates `.git` directory in `~/.taskquest/`
- Creates `.gitignore` (excludes .bak, .tmp, .sync-conflict files)
- Makes initial commit with all character data
- Ready for local use immediately

#### Add Remote (Optional)
```bash
# With GitHub/GitLab URL
taskquest sync init --remote git@github.com:username/taskquest-data.git

# Or add later
cd ~/.taskquest
git remote add origin git@github.com:username/taskquest-data.git
```

### Commands

#### Push Changes
```bash
taskquest sync push
```

**What it does**:
1. Commits any pending changes
2. Pushes to remote (origin/main)
3. Includes timestamp in commit message

**Output**:
```
✓ Committed: TaskQuest update - 2025-11-19 14:23:45
Pushing to git@github.com:username/taskquest-data.git...
✓ Pushed successfully
```

#### Pull Changes
```bash
taskquest sync pull
```

**What it does**:
1. Fetches latest from remote
2. Fast-forward merges if possible
3. Alerts if manual merge needed

**Output**:
```
Pulling from remote...
✓ Fast-forwarded to latest
```

**If conflicts**:
```
⚠️  Manual merge required
Changes on both local and remote. To merge:
  cd ~/.taskquest
  git merge FETCH_HEAD
  # Resolve any conflicts
  taskquest sync push
```

#### Show Status
```bash
taskquest sync status
```

**Output**:
```
╔════════════════════════════════════════╗
║          SYNC STATUS                   ║
╠════════════════════════════════════════╣
║ Working tree: clean                    ║
║ Remote: git@github.com:user/repo.git   ║
║ Last commit: 2025-11-19 14:23          ║
║ Message: TaskQuest update - 2025-11... ║
╚════════════════════════════════════════╝
```

#### View History
```bash
# Show last 10 commits
taskquest sync history

# Show last 20 commits
taskquest sync history 20
```

**Output**:
```
╔════════════════════════════════════════════════════════════╗
║                  SYNC HISTORY                              ║
╚════════════════════════════════════════════════════════════╝

a1b2c3d 2025-11-19 14:23 TaskQuest update - 2025-11-19 14:23:45
e4f5g6h 2025-11-19 10:15 TaskQuest update - 2025-11-19 10:15:32
i7j8k9l 2025-11-18 20:45 TaskQuest update - 2025-11-18 20:45:12
...
```

### Workflow Examples

#### Daily Use (Local Only)
```bash
# Work on tasks
task add "Code feature" challenge:7 stat1:int stat2:wis
task 1 done

# Character auto-saves locally
# No need to sync if only one device
```

#### Multi-Device Sync
```bash
# On Device A (laptop)
task add "Morning workout" challenge:6 stat1:str stat2:con
task 1 done
taskquest sync push

# On Device B (phone via Termux)
taskquest sync pull
taskquest status  # See updated stats
```

#### Rollback Example
```bash
# Oops, bought wrong reward
taskquest sync history  # Find commit before purchase
cd ~/.taskquest
git checkout a1b2c3d -- character.json
taskquest status  # Back to previous state
taskquest sync push  # Save rollback
```

### Security

**Private Data**:
```bash
# Create private GitHub repository
gh repo create taskquest-data --private

# Initialize with remote
taskquest sync init --remote git@github.com:username/taskquest-data.git
```

**SSH Keys** (Recommended):
```bash
# Generate SSH key
ssh-keygen -t ed25519 -C "your_email@example.com"

# Add to GitHub
cat ~/.ssh/id_ed25519.pub
# Paste into GitHub Settings → SSH Keys
```

**Alternative: HTTPS with Token**:
```bash
# Generate Personal Access Token on GitHub
# Use token as password when pushing
```

---

## Benefits Summary

### Custom Rewards
- ✅ Personalization
- ✅ Unlimited rewards
- ✅ Cooldown control
- ✅ Tier customization

### Progress Analytics
- ✅ Visual feedback
- ✅ Quick stat overview
- ✅ Life balance awareness
- ✅ Motivation boost

### Git Sync
- ✅ Version control
- ✅ Rollback capability
- ✅ Better than Syncthing
- ✅ Free cloud backup
- ✅ Multi-device support
- ✅ Offline-first

---

## Updated Command Reference

### Reward Management
```bash
taskquest add-reward <name> <cost> <description> [--tier TIER] [--cooldown HOURS]
taskquest remove-reward <id|name>
```

### Sync Commands
```bash
taskquest sync init [--remote URL]
taskquest sync push
taskquest sync pull
taskquest sync status
taskquest sync history [LIMIT]
```

### Display Commands
```bash
taskquest status   # XP progress bar
taskquest stats    # Stat bars + XP bar
```

---

## Implementation Details

### Files Modified
- `src/display/cli.rs` - Added commands
- `src/display/formatter.rs` - Enhanced progress bars
- `src/shop/rewards.rs` - Added cooldown parameter

### Files Created
- `src/sync/mod.rs` - Sync module
- `src/sync/git_ops.rs` - Git operations
- `docs/SYNC_ANALYSIS.md` - Sync comparison
- `docs/PHASE3_FEATURES.md` - This file

### Dependencies Added
- `git2 = "0.18"` - Git operations in Rust

---

## Testing

### Test Custom Rewards
```bash
# Add reward
taskquest add-reward "Test" 100 "Testing" --tier heroic --cooldown 24

# View in shop
taskquest shop

# Remove reward
taskquest remove-reward "Test"
```

### Test Progress Bars
```bash
# View XP bar
taskquest status

# View stat bars
taskquest stats
```

### Test Git Sync
```bash
# Initialize
taskquest sync init

# Check status
taskquest sync status

# Make changes
task add "Test task" challenge:5 stat1:int stat2:wis
task 1 done

# Push changes
taskquest sync push

# View history
taskquest sync history
```

---

## Future Enhancements (Potential Phase 4)

1. **Auto-Sync** - Optional auto-push after task completion
2. **Smart Merge** - Automatically take higher XP/stats on conflict
3. **Statistics Dashboard** - Graphs from git history
4. **Achievement Dates** - Track when achievements unlocked
5. **Reward Packs** - Import/export custom reward sets
6. **Sync Branches** - Different strategies per device

---

**Phase 3 Complete! ✅**

All requested features implemented:
- ✅ Custom reward management
- ✅ Progress bars (XP + stats)
- ✅ Git-sync (superior to Syncthing)
- ✅ Comprehensive documentation
