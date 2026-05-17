`policies.banned_patterns` contains `"rm\\s+-rf"`. The invocation `rm -rf /tmp/work` matches this regex when the full command line is checked.
Confirms banned_patterns operates on the full command string (not just the tool name) and fires before tool-level checks.
Spec: `spec/02-toolsets-schema.md` §"Policy evaluation" step 2, `spec/01-protocol.md` §2.2.
