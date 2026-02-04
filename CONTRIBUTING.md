# How to contribute

#### **Did you find a bug?**

* **Do not open up a GitHub issue if the bug is a security vulnerability**,
  and instead to refer to our
  [security policy](https://github.com/astro/rust-protobuf-iter/security).

* **Ensure the bug was not already reported** by searching on GitHub
  under [Issues](https://github.com/astro/rust-protobuf-iter/issues).

* If you're unable to find an open issue addressing the problem,
  [open a new one](https://github.com/astro/rust-protobuf-iter/issues/new).
  Be sure to include a **title and clear description**, as much relevant
  information as possible, and a **code sample** or an **executable test
  case** demonstrating the expected behavior that is not occurring.


#### **Did you write a patch that fixes a bug?**

* Open a new GitHub pull request with your patch.

* Ensure the PR description clearly describes the problem and solution.
  Include the relevant issue number if applicable.

* Make sure your fix is covered by unit tests.


#### **Do you intend to add a new feature or change an existing one?**

* Suggest your change by creating a
  [new GitHub issue](https://github.com/astro/rust-protobuf-iter/issues/new)
  and start writing code.

* Make sure your change is covered by unit tests.


#### **Do you have questions about the source code?**

* Ask any question by filing a
  [new GitHub issue](https://github.com/astro/rust-protobuf-iter/issues/new).


#### **As a maintainer, do you want to cut a new release?**

* Decide on a version number according to [Semantic
  Versioning](https://semver.org/). If there’s been any breaking
  changes to our public API, increment the major version. If there’s
  been any non-breaking changes (such as new public functions),
  increment the minor version. Otherwise, increment the patch version.

* Send a pull request to update the version tag in `Cargo.toml` and
  get it merged into the `master` branch.

* Via the GitHub web interface, [create a
  release](https://github.com/astro/rust-protobuf-iter/releases/new)
  with the new tag. Click the button to generate release notes.
  Clarify and shorten the release notes like in this
  [example](https://github.com/astro/rust-protobuf-iter/releases/tag/v0.1.3),
  so downstream users can quickly understand what changed. Finally,
  click “Publish”.

* That’s all. GitHub will trigger an automated workflow to publish the
  new version on [crates.io](https://crates.io/crates/protobuf_iter).


#### **Do you want to join the team?**

This project is a volunteer effort. We encourage you to pitch in and join
[the team](https://github.com/astro/rust-protobuf-iter/graphs/contributors)!


Thanks! :heart: :heart: :heart:
