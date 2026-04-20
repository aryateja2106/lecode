Output demos/lecode-quickstart.gif

Set FontSize 14
Set Width 1200
Set Height 720
Set Theme "Dracula"
Set Padding 20
Set Shell "zsh"

# Banner
Type "# lecode — one terminal, every agent, every machine"
Sleep 1200ms
Enter
Type "clear"
Enter
Sleep 300ms

# 1. Start daemon in background
Type "# 1. Start the daemon (with test provider)"
Enter
Sleep 400ms
Type "./target/release/lesearch-daemon --addr 127.0.0.1:7070 &"
Enter
Sleep 1500ms

# 2. Spawn an agent, stream output
Type "# 2. Spawn a test agent — watch its output stream back"
Enter
Sleep 400ms
Type "./target/release/lesearch-cli run --addr ws://127.0.0.1:7070/ws --provider test"
Enter
Sleep 2500ms

# 3. List + status
Type "# 3. Inspect the daemon"
Enter
Sleep 400ms
Type "./target/release/lesearch-cli ls --addr ws://127.0.0.1:7070/ws"
Enter
Sleep 1500ms
Type "./target/release/lesearch-cli daemon status --addr ws://127.0.0.1:7070/ws"
Enter
Sleep 1500ms

# 4. Cleanup
Type "# 4. Stop daemon"
Enter
Sleep 400ms
Type "kill %1 2>/dev/null; wait 2>/dev/null"
Enter
Sleep 800ms

Type "# ✓ Apache-2.0 · 21 tests · self-hosted"
Enter
Sleep 1800ms
