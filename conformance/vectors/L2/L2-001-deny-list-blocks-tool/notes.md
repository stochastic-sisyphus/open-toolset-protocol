`policies.deny` lists `"grep"`. `default_action` is `"allow"` so only the denylist is exercised.
Confirms that the deny list wins absolutely - even though default_action would permit the call, the denylist overrides it ("Deny always wins").
Spec: `spec/02-toolsets-schema.md` §"Policy evaluation" step 1.
