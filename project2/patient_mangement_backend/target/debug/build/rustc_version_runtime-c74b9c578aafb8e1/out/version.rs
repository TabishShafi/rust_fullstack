
            /// Returns the `rustc` SemVer version and additional metadata
            /// like the git short hash and build date.
            pub fn version_meta() -> VersionMeta {
                VersionMeta {
                    semver: Version {
                        major: 1,
                        minor: 82,
                        patch: 0,
                        pre: Prerelease::new("").unwrap(),
                        build: BuildMetadata::new("").unwrap(),
                    },
                    host: "x86_64-apple-darwin".to_owned(),
                    short_version_string: "rustc 1.82.0 (f6e511eec 2024-10-15)".to_owned(),
                    commit_hash: Some("f6e511eec7342f59a25f7c0534f1dbea00d01b14".to_owned()),
                    commit_date: Some("2024-10-15".to_owned()),
                    build_date: None,
                    channel: Channel::Stable,
                    llvm_version: Some(LlvmVersion{ major: 19, minor: 1 }),
                }
            }
            