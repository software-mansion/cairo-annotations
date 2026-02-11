# `cairo-annotations` Maintenance

## Maintenance Procedure

To maintain `cairo-annotations` effectively, ensure compatibility with the latest versions of **Scarb** and **Starknet
Foundry**.

### Compatibility Checks

Historically, compatibility issues have rarely occurred due to updates in tools like **Scarb** and **Starknet Foundry**.
However, it's good practice to keep these tools updated to their latest versions to ensure that `cairo-annotations`
remains compatible with them.

For guidance on updating dependencies, refer to this pull request as an
example: [PR](https://github.com/software-mansion/cairo-annotations/pull/120/changes).

If compatibility issues arise with the latest versions of Scarb or Starknet Foundry, they should be addressed and a new
version of cairo-annotations should be released.

> Note: We do not currently guarantee backward compatibility; by default, we only support the latest versions of
> ecosystem tools. However, supporting older versions when possible would be nice.

## Release Procedure

To release a new version of `cairo-annotations`, follow these steps:

1. **Prepare a Pull Request (PR)**:
    - Ensure the correct version is updated in the following files:
        - `Cargo.toml`
        - `Cargo.lock`
        - `CHANGELOG.md`
    - As a reference, you can check this [PR](https://github.com/software-mansion/cairo-annotations/pull/84/changes).

2. **Run the Release Action**:
    - Trigger
      the [release workflow](https://github.com/software-mansion/cairo-annotations/actions/workflows/release.yml)
      from the branch you want to release.
    - It is recommended to run this from the `main` branch.
    - Ensure that the commit you are releasing from has passed CI checks, as this is not automatically verified.
    - The workflow will create a tag and generate a new GitHub release, and include the contents of
      `CHANGELOG.md`.

3. **Release to Crates.io**:
    - After the GitHub Release, publish the new version to crates.io.
    - Navigate to the `crates/cairo-annotations` directory.
    - Ensure you are logged into `crates.io` using `cargo login`.
    - Run `cargo publish` to upload the crate.
    - Confirm the new version appears on [crates.io](https://crates.io/crates/cairo-annotations).

4. **Announce the Release**:
    - Notify the community about the update on **Twitter, Telegram, and Discord**.