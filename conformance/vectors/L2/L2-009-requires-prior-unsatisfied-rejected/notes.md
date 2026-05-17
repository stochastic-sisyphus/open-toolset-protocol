`grep` declares `requires_prior: ["category:navigation"]`. The sequence invokes `grep` directly without first running any `navigation`-category tool.
Confirms the adapter rejects the invocation when no prior tool in the current phase trace satisfies the `requires_prior` constraint.
Spec: `spec/02-toolsets-schema.md` §"Policy evaluation" step 7, `schemas/toolsets.schema.json` `$defs.tool.properties.requires_prior`.
