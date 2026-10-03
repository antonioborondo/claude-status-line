# claude-status-line

[![CI](https://github.com/antonioborondo/claude-status-line/actions/workflows/ci.yml/badge.svg)](https://github.com/antonioborondo/claude-status-line/actions/workflows/ci.yml)

## Description

Claude [status line](https://code.claude.com/docs/en/statusline) that displays
the model, effort, context, and cost:

```
🧠 Opus 💪 high 💭 42% 💰 $1.23
```

## Configuration

Replace `/path/to/claude-status-line` with the directory where you want the repo:

1. Clone the repo:

   ```
   git clone https://github.com/antonioborondo/claude-status-line.git /path/to/claude-status-line
   ```

2. Build the executable:

   ```
   cd /path/to/claude-status-line && cargo build --release
   ```

3. Add the status line to your Claude settings in `~/.claude/settings.json`:

   ```json
   {
     "statusLine": {
       "type": "command",
       "command": "/path/to/claude-status-line/target/release/claude-status-line"
     }
   }
   ```
