#!/bin/bash
set -e

TQ="/home/phi/.cargo/bin/taskquest"
CHAR_FILE="$HOME/.taskquest/character.json"

echo "═══════════════════════════════════════════════════════════"
echo "  TaskQuest Comprehensive Test Suite"
echo "═══════════════════════════════════════════════════════════"
echo ""

# Function to extract JSON values
get_json_value() {
    cat "$CHAR_FILE" | grep "\"$1\"" | head -1 | awk -F': ' '{print $2}' | tr -d ',' | tr -d '"'
}

# Test 1: Initial State
echo "TEST 1: Verify Initial Character State"
echo "----------------------------------------"
$TQ status
level=$(get_json_value "level")
xp=$(get_json_value "total_xp")
gold=$(get_json_value "gold")
tasks=$(get_json_value "tasks_completed")
str=$(get_json_value "strength")

echo "Initial Stats:"
echo "  Level: $level (expected: 1)"
echo "  XP: $xp (expected: 0)"
echo "  Gold: $gold (expected: 0)"
echo "  Tasks: $tasks (expected: 0)"
echo "  STR: $str (expected: 10)"
echo ""

# Test 2: Create and complete tasks
echo "TEST 2: Creating Test Tasks"
echo "----------------------------------------"

# Clear any existing tasks
task rc.confirmation=off delete / 2>/dev/null || true
task rc.gc=on

# Create tasks with various challenge levels
echo "Creating tasks with challenge levels 1, 3, 5, 7, 10..."
task add "Trivial task - Check email" challenge:1
task add "Easy task - Update README" challenge:3
task add "Medium task - Fix bug" challenge:5
task add "Hard task - Implement feature" challenge:7
task add "Epic task - Launch product" challenge:10

echo "Tasks created:"
task list
echo ""

# Test 3: Character modification commands
echo "TEST 3: Testing Character Modification Commands"
echo "----------------------------------------"
echo "Current name: TestWarrior"
$TQ name "BattleMaster"
echo "Changed name to BattleMaster"

$TQ status | grep "BattleMaster"
echo "✓ Name change verified"
echo ""

# Test 4: Stats display
echo "TEST 4: Testing Stats Display"
echo "----------------------------------------"
$TQ stats
echo ""

# Test 5: View character JSON before completing tasks
echo "TEST 5: Character Data Before Task Completion"
echo "----------------------------------------"
cat "$CHAR_FILE" | jq '.'
echo ""

# Summary
echo "═══════════════════════════════════════════════════════════"
echo "  Phase 1 Tests Complete"
echo "═══════════════════════════════════════════════════════════"
echo ""
echo "NOTE: To test task completion integration with Taskwarrior,"
echo "we need to set up the hooks. This will be in Phase 2 testing."
echo ""
echo "Manual test: Complete a task with 'task 1 done' and check"
echo "if the character XP/gold/stats update."
echo ""
