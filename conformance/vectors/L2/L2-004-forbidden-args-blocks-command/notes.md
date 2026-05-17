`policies.forbidden_args` contains the exact argv list `["rm", "-rf", "/tmp/work"]`. The invocation `rm -rf /tmp/work` matches this literal argv list when the full invocation is checked.
Confirms forbidden_args operates on the full argv list (not just the tool name) and fires before tool-level checks.
Spec: `spec/02-toolsets-schema.md` §"Policy evaluation" step 2, `spec/01-protocol.md` §2.2.
