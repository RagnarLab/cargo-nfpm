//! Tests related to parsing the `Cargo.toml`.

use std::collections::HashMap;
use std::path::PathBuf;

use cargo_nfpm::cargo_schema::CargoManifest;
use cargo_nfpm::nfpm_schema::{
    Alternative, ApkScripts, ApkSignature, ApkSpecificSettings, ApplicationsInThePackage,
    ArchlinuxSpecificScripts, ArchlinuxSpecificSettings, Config, ContentElement, DebCompression,
    DebSpecificSettings, IpkSpecificSettings, MethodRole, MsixSignature, MsixSpecificSettings,
    OverrideValue, PackageCapabilities, PackageIdentity, PackageProperties, RpmCompression,
    RpmSignature, RpmSpecificScripts, RpmSpecificSettings, Scripts, ScriptsToExecute, Signature,
    SignerRole, TargetDeviceFamilies, TargetDeviceFamilyElement, Triggers, Type, VersionSchema,
    VisualElements,
};
use pretty_assertions::assert_eq;

#[test]
fn roundtrip() {
    let config = Config {
        name: Some("cargo-nfpm".to_owned()),
        arch: Some("amd64".to_owned()),
        version: Some("1.0.0".to_owned()),

        changelog: Some("changelog.yml".to_owned()),
        description: Some(
            "A simple and lightweight Cargo plugin for using nFPM from Any Rust project."
                .to_owned(),
        ),
        epoch: Some("1".to_owned()),
        homepage: Some("https://github.com/RagnarLab/cargo-nfpm/".to_owned()),
        license: Some("MIT OR Apache-2.0".to_owned()),
        maintainer: Some("Arvid Gerstmann <github@arvid.io>".to_owned()),
        mtime: Some("2009-11-10T23:00:00Z".to_owned()),
        platform: Some("linux".to_owned()),
        prerelease: Some("0".to_owned()),
        priority: Some("extra".to_owned()),
        release: Some("1".to_owned()),
        section: Some("default".to_owned()),
        vendor: Some("Arvid Gerstmann <github@arvid.io>".to_owned()),
        version_metadata: Some("version".to_owned()),
        disable_globbing: Some(false),
        umask: Some(0o002),
        version_schema: Some(VersionSchema::None),

        conflicts: Some(vec!["package-which-conflicts".to_owned()]),
        depends: Some(vec!["package-dependencies".to_owned()]),
        provides: Some(vec!["cargo-nfpm".to_owned()]),
        recommends: Some(vec!["recommended-package-to-install".to_owned()]),
        replaces: Some(vec!["package-that-it-replaces".to_owned()]),
        suggests: Some(vec!["suggested-package".to_owned()]),

        scripts: Some(ScriptsToExecute {
            postinstall: Some("postinstall-script.sh".to_owned()),
            postremove: Some("postremove-script.sh".to_owned()),
            preinstall: Some("preinstall-script.sh".to_owned()),
            preremove: Some("preremove-script.sh".to_owned()),
        }),

        overrides: Some(HashMap::from([(
            "umask".to_owned(),
            OverrideValue {
                umask: Some(0o002),
                ..Default::default()
            },
        )])),

        contents: Some(vec![ContentElement {
            dst: "/usr/bin/cargo-nfpm".to_owned(),
            expand: Some(false),
            file_info: None,
            packager: Some("deb".to_owned()),
            src: Some("/home/ubuntu/cargo-nfpm/target/release/cargo-nfpm".to_owned()),
            confi_type: Some(Type::Empty),
        }]),

        deb: Some(DebSpecificSettings {
            arch: Some("amd64".to_owned()),
            breaks: Some(vec![
                "installation-will-fail-if-this-is-installed".to_owned(),
            ]),
            compression: Some(DebCompression::Gzip),
            fields: Some(HashMap::from([(
                "Bugs".to_owned(),
                "https://github.com/RagnarLab/cargo-nfpm/issues".to_owned(),
            )])),
            predepends: Some(vec!["predepends".to_owned()]),
            scripts: Some(Scripts {
                config: Some("config-for-debconf".to_owned()),
                rules: Some("rules-script-for-debconf.sh".to_owned()),
                templates: Some("templates-file-for-debconf".to_owned()),
            }),
            signature: Some(Signature {
                key_file: Some("key.gpg".to_owned()),
                key_id: Some("bc8acdd415bd80b3".to_owned()),
                method: Some(MethodRole::Debsign),
                signer: Some("signer".to_owned()),
                signature_type: Some(SignerRole::Maint),
            }),
            triggers: Some(Triggers {
                activate: Some(vec!["activate-trigger-name".to_owned()]),
                activate_await: Some(vec!["activate_await-trigger-name".to_owned()]),
                activate_noawait: Some(vec!["activate_noawait-trigger-name".to_owned()]),
                interest: Some(vec!["interest-trigger-name".to_owned()]),
                interest_await: Some(vec!["interest_await-trigger-name".to_owned()]),
                interest_noawait: Some(vec!["interest_noawait-trigger-name".to_owned()]),
            }),
            arch_variant: Some("amd64v1".to_owned()),
        }),

        ipk: Some(IpkSpecificSettings {
            abi_version: Some("abi_version".to_owned()),
            alternatives: Some(vec![Alternative {
                link_name: Some("link_name".to_owned()),
                priority: Some(1),
                target: Some("target".to_owned()),
            }]),
            arch: Some("x86_64".to_owned()),
            auto_installed: Some(false),
            essential: Some(false),
            fields: Some(HashMap::from([("Foo".to_owned(), "Bar".to_owned())])),
            predepends: Some(vec!["cargo".to_owned()]),
            tags: Some(vec!["build-tools".to_owned()]),
        }),

        rpm: Some(RpmSpecificSettings {
            arch: Some("x86_64".to_owned()),
            buildhost: Some("x86_64".to_owned()),
            compression: Some(RpmCompression::Gzip),
            group: Some("Unspecified".to_owned()),
            packager: Some("Arvid Gerstmann <github@arvid.io>".to_owned()),
            prefixes: Some(vec!["/usr/bin".to_owned()]),
            scripts: Some(RpmSpecificScripts {
                posttrans: Some("./scripts/posttrans.sh".to_owned()),
                pretrans: Some("./scripts/pretrans.sh".to_owned()),
                verify: Some("./scripts/verify.sh".to_owned()),
            }),
            signature: Some(RpmSignature {
                key_file: Some("key.gpg".to_owned()),
                key_id: Some("012345ABCDEF".to_owned()),
            }),
            summary: Some(
                "Explicit summary for the package. Defaults to first line of description."
                    .to_owned(),
            ),
        }),

        apk: Some(ApkSpecificSettings {
            arch: Some("x86_64".to_owned()),
            scripts: Some(ApkScripts {
                postupgrade: Some("postupgrade-script.sh".to_owned()),
                preupgrade: Some("preupgrade-script.sh".to_owned()),
            }),
            signature: Some(ApkSignature {
                key_file: Some("key.gpg".to_owned()),
                key_id: Some("ignored".to_owned()),
                key_name: Some("origin".to_owned()),
            }),
        }),

        archlinux: Some(ArchlinuxSpecificSettings {
            arch: Some("x86_64".to_owned()),
            packager: Some("Arvid Gerstmann <github@arvid.io>".to_owned()),
            pkgbase: Some("cargo-nfpm".to_owned()),
            scripts: Some(ArchlinuxSpecificScripts {
                postupgrade: Some("./scripts/postupgrade.sh".to_owned()),
                preupgrade: Some("./scripts/preupgrade.sh".to_owned()),
            }),
        }),

        msix: Some(MsixSpecificSettings {
            applications: vec![ApplicationsInThePackage {
                id: "App".to_owned(),
                entry_point: Some("Windows.FullTrustApplication".to_owned()),
                executable: "app/myapp.exe".to_owned(),
                visual_elements: Some(VisualElements {
                    background_color: Some("transparent".to_owned()),
                    description: Some("My application description".to_owned()),
                    display_name: Some("My Application".to_owned()),
                    square150_x150_logo: Some("Assets/Square150x150Logo.png".to_owned()),
                    square44_x44_logo: Some("Assets/Square44x44Logo.png".to_owned()),
                }),
            }],
            arch: Some("x64".to_owned()),
            capabilities: Some(PackageCapabilities {
                capabilities: Some(vec!["internetClient".to_owned()]),
                device_capabilities: Some(vec!["microphone".to_owned()]),
                restricted: Some(vec!["broadFileSystemAccess".to_owned()]),
            }),
            dependencies: Some(TargetDeviceFamilies {
                target_device_families: Some(vec![TargetDeviceFamilyElement {
                    name: "Windows.Desktop".to_owned(),
                    max_version_tested: "10.0.22621.0".to_owned(),
                    min_version: "10.0.17763.0".to_owned(),
                }]),
            }),
            identity: Some(PackageIdentity {
                resource_id: Some("".to_owned()),
            }),
            properties: Some(PackageProperties {
                display_name: Some("My Application".to_owned()),
                logo: Some("Assets/logo.png".to_owned()),
                publisher_display_name: Some("My Company".to_owned()),
            }),
            publisher: "CN=MyCompany, O=MyCompany, C=US".to_owned(),
            signature: Some(MsixSignature {
                pfx_file: Some("certificate.pfx".to_owned()),
                // Passphrase is read from the NFPM_MSIX_PASSPHRASE environment variable
            }),
        }),
    };

    // let str = toml::to_string(&CargoManifest {
    //     package: CargoPackage {
    //         metadata: Some(CargoMetadata {
    //             nfpm: Some(config.clone()),
    //         }),
    //     },
    // })
    // .unwrap();
    // println!("{str}");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let alltoml =
        std::fs::read_to_string(PathBuf::from(manifest_dir).join("fixtures/all.toml")).unwrap();

    let parsed: CargoManifest = toml::from_str(&alltoml).unwrap();

    assert_eq!(
        &config,
        parsed.package.metadata.unwrap().nfpm.as_ref().unwrap()
    );
}
