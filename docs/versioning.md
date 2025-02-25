# Versioning

Ruff uses a custom versioning scheme that uses the **minor** version number for breaking changes and the **patch** version number for bug fixes. Ruff does not yet have a stable API; once Ruff's API is stable, the **major** version number and semantic versioning will be used.

## Version changes

**Minor** version increases will occur when:

- A deprecated option or feature is removed
- Configuration changes in a backwards incompatible way
    - This _may_ occur in minor version changes until `1.0.0`, however it should generally be avoided.
- A rule is promoted to stable
- Support for a new file type is promoted to stable
- Support for an end-of-life Python version is dropped
- The behavior of a stable rule is changed
    - Does not include bug fixes that follow the original intent of the rule
- Stable rules are added to the default set
- Stable rules are removed from the default set
- A safe fix for a rule is promoted to stable

**Patch** version increases will occur when:

- Bugs are fixed, _including behavior changes that fix bugs_
- An unsafe fix for a rule is added
- A safe fix for a rule is added in preview
- A fix’s applicability is demoted
- A new configuration option is added
- A rule is added in preview
- The behavior of a preview rule is changed
- Support for a new Python version is added
- Support for a new file type is added in preview
- An option or feature is deprecated

## Preview mode

A preview mode is available to enable new, unstable rules and features e.g. support for a new file type.

The preview mode is intended to help us collect community feedback and gain confidence that changes are a net-benefit.

The preview mode is _not_ intended to gate access to work that is incomplete or features that we are _likely to remove._ However, **we reserve the right to make changes to _any_ behavior gated by the mode** including the removal of preview features or rules.

## Rule stabilization

When modifying or adding rules, we use the following guidelines:

- New rules are always be added in a preview mode
- New rules remain in preview mode for at least one minor release before being promoted to stable
    - If added in a patch release i.e. `0.6.1` then a rule are not be eligible for stability until `0.8.0`
- Stable rule behavior are not changed significantly in patch versions
- Promotion of rules to stable may be delayed in order to “batch” them into a single minor release
- Not all rules in preview need to be promoted in a given minor release

## Fix stabilization

Fixes have three applicability levels:

- **Display**: Never applied, just displayed.
- **Unsafe**: Can be applied with explicit opt-in.
- **Safe**: Can be applied automatically.

Fixes for rules may be introduced at a lower applicability then promoted to a higher applicability. Reducing the applicability of a fix is not a breaking change. The applicability of a given fix may change when the preview mode is enabled.
