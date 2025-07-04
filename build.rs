mod cargo_workspace {
    /// Verify that the cargo metadata workspace packages format matches that expected by
    /// [`set_cargo_workspace_members_env`] to set the `CARGO_WORKSPACE_MEMBERS` environment variable.
    ///
    /// This function should be typically called within build scripts, before the
    /// [`set_cargo_workspace_members_env`] function is called.
    ///
    /// # Panics
    ///
    /// Panics if running the `cargo metadata` command fails, or if the workspace member package names
    /// cannot be determined.
    pub fn verify_cargo_metadata_format() {
        #[allow(clippy::expect_used)]
        let metadata = cargo_metadata::MetadataCommand::new()
            .exec()
            .expect("Failed to obtain cargo metadata");

        assert!(
            metadata
                .workspace_packages()
                .iter()
                .any(|package| package.name == env!("CARGO_PKG_NAME")),
            "Unable to determine workspace member package names from `cargo metadata`"
        );
    }

    /// Sets the `CARGO_WORKSPACE_MEMBERS` environment variable to include a comma-separated list of
    /// names of all crates in the current cargo workspace.
    ///
    /// This function should be typically called within build scripts, so that the environment variable
    /// is available to the corresponding crate at compile time.
    ///
    /// # Panics
    ///
    ///
    /// Panics if running the `cargo metadata` command fails.
    #[allow(clippy::expect_used)]
    pub fn set_cargo_workspace_members_env() {
        use std::io::Write;

        let metadata = cargo_metadata::MetadataCommand::new()
            .exec()
            .expect("Failed to obtain cargo metadata");

        let workspace_members = metadata
            .workspace_packages()
            .iter()
            .map(|package| package.name.as_str())
            .collect::<Vec<_>>()
            .join(",");

        writeln!(
            &mut std::io::stdout(),
            "cargo:rustc-env=CARGO_WORKSPACE_MEMBERS={workspace_members}"
        )
        .expect("Failed to set `CARGO_WORKSPACE_MEMBERS` environment variable");
    }
}

fn main() {
    cargo_workspace::verify_cargo_metadata_format();
    cargo_workspace::set_cargo_workspace_members_env();
}
