# CLI Design

* Offer a TUI (or web?) "tutorial" experience to quickly build workflows from possible features.
* Plugins with Extism?

The Punchcard CLI offers commands to validate and generate GitHub Actions
workflows.

Subcommands:

* (Aspirational) `generate`: create a Workflow scaffold based on user provided
use cases. For example, `punchcard generate on-commit` for CI workflows, and
`punchcard generate issue-comment` for automated community management.

* `punchcard apply {file}.dhall`: convert the Dhall specification of the
Workflow into a YAML specification, generating warnings and errors for
based on opinionated lints.

TODO: Support specific language ecosystems? e.g. Pulumi, Rust, Nix:
If `setup-rust` is installed but there's no Rust caching, consider
automatically adding it on the behalf of the user.
  * Check for missing items. Example:
```dhall
let GithubActions =
      https://regadas.dev/github-actions-dhall/package.dhall
        sha256:feee9bf55dbdd94352e835a7c07b0850c6530b7ddf1eea9ae959b41464fddd5a
in GithubActions.Workflow::{
    , name = "Greeting"
    , on = GithubActions.On::{  }
    }
```
Dhall will let you write workflows with an empty list of `on` events. There's
no way to specify the list must be non-empty. Similarly, all fields on 
product types are optional, when they really mean "one or more of these fields
must be set". There's no semantic validation.
  * Walk the YAML data structure looking for "setup-*" and then check for
  caching usage after.
  * Perhaps: analyze cache for hits vs. misses?
  * Analyze artifact usage? E.g. you've built a release binary but you aren't
  storing the release as an artifact.
