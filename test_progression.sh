#!/bin/bash
# Test script to demonstrate new stat progression with quadratic diminishing returns

echo "===== New Stat Progression System ====="
echo ""
echo "Formula:"
echo "  Base gain = challenge / 20.0"
echo "  Difficulty multiplier = 1.0 + ((current_stat - 10) / 89)^2"
echo "  Actual gain = base_gain / difficulty_multiplier"
echo ""
echo "Progression at different stat levels (Challenge 5 task):"
echo ""

# Function to calculate difficulty multiplier
calc_multiplier() {
    local stat=$1
    echo "scale=4; 1.0 + ((($stat - 10.0) / 89.0) * (($stat - 10.0) / 89.0))" | bc
}

# Function to calculate actual gain
calc_gain() {
    local stat=$1
    local challenge=5
    local base_gain=$(echo "scale=6; $challenge / 20.0" | bc)
    local multiplier=$(calc_multiplier $stat)
    echo "scale=6; $base_gain / $multiplier" | bc
}

# Function to calculate tasks needed for 1 point
calc_tasks() {
    local stat=$1
    local gain=$(calc_gain $stat)
    echo "scale=2; 1.0 / $gain" | bc
}

printf "%-15s %-20s %-20s %-20s\n" "Current Stat" "Difficulty Mult." "Gain per Task" "Tasks for +1"
printf "%s\n" "--------------------------------------------------------------------------------"

for stat in 10 20 30 40 50 60 70 80 90 99; do
    mult=$(calc_multiplier $stat)
    gain=$(calc_gain $stat)
    tasks=$(calc_tasks $stat)
    printf "%-15s %-20s %-20s %-20s\n" "$stat" "$mult" "$gain" "$tasks"
done

echo ""
echo "Summary (Challenge 5 tasks):"
echo "  - Early game (stat 10-30): ~4-5 tasks per stat point"
echo "  - Mid game (stat 40-60): ~5-7 tasks per stat point"
echo "  - Late game (stat 70-90): ~7-11 tasks per stat point"
echo "  - Endgame (stat 90-99): ~11-13 tasks per stat point"
echo ""
echo "Challenge scaling:"
echo "  - Challenge 1: 20 tasks/point (early), 40 tasks/point (late)"
echo "  - Challenge 5: 4 tasks/point (early), 8 tasks/point (late)"
echo "  - Challenge 10: 2 tasks/point (early), 4 tasks/point (late)"
echo ""
echo "Higher challenge tasks give significantly more stat gains!"
