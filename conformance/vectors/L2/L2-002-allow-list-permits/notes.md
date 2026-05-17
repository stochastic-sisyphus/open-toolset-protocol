`default_action` is `"deny"` but `policies.allow` explicitly permits `"ls"`. Sequence invokes ls in the reconnaissance phase.
Confirms that an explicit allow list entry overrides default_deny and the call proceeds to execution.
Spec: `spec/02-toolsets-schema.md` §"Policy evaluation" steps 3–4.
