`default_action` is `"deny"` and `ls` is not in `policies.allow` (no allow list present). Sequence invokes ls.
Confirms the adapter falls through to the default_deny branch for any tool not matched by an explicit allow entry.
Spec: `spec/02-toolsets-schema.md` §"Policy evaluation" step 5, `spec/01-protocol.md` §2.2.
