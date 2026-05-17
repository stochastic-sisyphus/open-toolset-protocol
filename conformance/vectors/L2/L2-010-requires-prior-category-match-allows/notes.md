Same registry as L2-009 but `git-status` (category: `navigation`) is invoked first, satisfying `grep`'s `requires_prior: ["category:navigation"]` constraint.
Confirms the positive path: `category:` prefix matching in `requires_prior` resolves against the category of tools already in the current phase trace.
Spec: `spec/02-toolsets-schema.md` §"Policy evaluation" step 7, `schemas/toolsets.schema.json` `$defs.tool.properties.requires_prior`.
