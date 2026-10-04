# claude-status-line

[![CI](https://github.com/antonioborondo/claude-status-line/actions/workflows/ci.yml/badge.svg)](https://github.com/antonioborondo/claude-status-line/actions/workflows/ci.yml)

## Description

Claude [status line](https://code.claude.com/docs/en/statusline) that displays
the model, effort, context, and cost:

```
🧠 Opus 💪 high 💭 42% 💰 $1.23
```

## Configuration

1. Install the binary:

   ```
   cargo install claude-status-line
   ```

1. Add the status line to your Claude settings in `~/.claude/settings.json`:

   ```json
   {
     "statusLine": {
       "type": "command",
       "command": "claude-status-line"
     }
   }
   ```
