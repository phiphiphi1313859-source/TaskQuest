#!/bin/bash
set -e

echo "Installing TaskQuest hooks for Taskwarrior..."

# Get taskwarrior hooks directory
HOOK_DIR="$HOME/.task/hooks"
TQ_BINARY="/home/phi/.cargo/bin/taskquest"

# Create hooks directory if it doesn't exist
mkdir -p "$HOOK_DIR"

# Create symlinks for hooks
echo "Creating hook symlinks..."

# on-add hook
cat > "$HOOK_DIR/on-add-taskquest" << 'EOF'
#!/bin/bash
/home/phi/.cargo/bin/taskquest "$@"
EOF

# on-modify hook
cat > "$HOOK_DIR/on-modify-taskquest" << 'EOF'
#!/bin/bash
/home/phi/.cargo/bin/taskquest "$@"
EOF

# on-exit hook
cat > "$HOOK_DIR/on-exit-taskquest" << 'EOF'
#!/bin/bash
/home/phi/.cargo/bin/taskquest "$@"
EOF

# Make hooks executable
chmod +x "$HOOK_DIR/on-add-taskquest"
chmod +x "$HOOK_DIR/on-modify-taskquest"
chmod +x "$HOOK_DIR/on-exit-taskquest"

echo "✅ Hooks installed successfully!"
echo ""
echo "Hook files created:"
ls -l "$HOOK_DIR"/on-*-taskquest
echo ""
echo "TaskQuest will now automatically track task completions!"
