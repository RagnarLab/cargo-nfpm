//! Tests related to parsing the `Cargo.toml`.

use std::collections::HashMap;
use std::path::PathBuf;

use cargo_nfpm::cargo_schema::CargoManifest;
use cargo_nfpm::nfpm_schema::{
    Alternative, ApkScripts, ApkSignature, ApkSpecificSettings, ArchlinuxSpecificScripts,
    ArchlinuxSpecificSettings, Config, ContentElement, DebCompression, DebSpecificSettings,
    IpkSpecificSettings, MethodRole, OverrideValue, RpmCompression, RpmSignature,
    RpmSpecificScripts, RpmSpecificSettings, Scripts, ScriptsToExecute, Signature, SignerRole,
    Triggers, Type, VersionSchema,
};

#[test]
fn roundtrip() {
    let config = Config {
        name: "foo".to_owned(),
        arch: "amd64".to_owned(),
        version: "1.0.0".to_owned(),

        changelog: Some("changelog".to_owned()),
        description: Some("description".to_owned()),
        epoch: Some("epoch".to_owned()),
        homepage: Some("homepage".to_owned()),
        license: Some("license".to_owned()),
        maintainer: Some("maintainer".to_owned()),
        mtime: Some("mtime".to_owned()),
        platform: Some("platform".to_owned()),
        prerelease: Some("prerelease".to_owned()),
        priority: Some("priority".to_owned()),
        release: Some("release".to_owned()),
        section: Some("section".to_owned()),
        vendor: Some("vendor".to_owned()),
        version_metadata: Some("version_metadata".to_owned()),
        disable_globbing: Some(false),
        umask: Some(0o002),
        version_schema: Some(VersionSchema::None),

        conflicts: Some(vec!["conflicts".to_owned()]),
        depends: Some(vec!["depends".to_owned()]),
        provides: Some(vec!["provides".to_owned()]),
        recommends: Some(vec!["recommends".to_owned()]),
        replaces: Some(vec!["replaces".to_owned()]),
        suggests: Some(vec!["suggests".to_owned()]),

        scripts: Some(ScriptsToExecute {
            postinstall: Some("postinstall".to_owned()),
            postremove: Some("postremove".to_owned()),
            preinstall: Some("preinstall".to_owned()),
            preremove: Some("preremove".to_owned()),
        }),

        overrides: Some(HashMap::from([(
            "umask".to_owned(),
            OverrideValue {
                umask: Some(0o002),
                ..Default::default()
            },
        )])),

        contents: Some(vec![ContentElement {
            dst: "destination".to_owned(),
            expand: Some(false),
            file_info: None,
            packager: Some("me".to_owned()),
            src: Some("source".to_owned()),
            confi_type: Some(Type::Config),
        }]),

        deb: Some(DebSpecificSettings {
            arch: Some("amd64".to_owned()),
            breaks: Some(vec!["breaks".to_owned()]),
            compression: Some(DebCompression::Gzip),
            fields: Some(HashMap::from([("Foo".to_owned(), "Bar".to_owned())])),
            predepends: Some(vec!["predepends".to_owned()]),
            scripts: Some(Scripts {
                config: Some("config".to_owned()),
                rules: Some("rules".to_owned()),
                templates: Some("templates".to_owned()),
            }),
            signature: Some(Signature {
                key_file: Some("key_file".to_owned()),
                key_id: Some("key_id".to_owned()),
                method: Some(MethodRole::Debsign),
                signer: Some("signer".to_owned()),
                signature_type: Some(SignerRole::Maint),
            }),
            triggers: Some(Triggers {
                activate: Some(vec!["activate".to_owned()]),
                activate_await: Some(vec!["activate_await".to_owned()]),
                activate_noawait: Some(vec!["activate_noawait".to_owned()]),
                interest: Some(vec!["interest".to_owned()]),
                interest_await: Some(vec!["interest_await".to_owned()]),
                interest_noawait: Some(vec!["interest_noawait".to_owned()]),
            }),
        }),
        ipk: Some(IpkSpecificSettings {
            abi_version: Some("abi_version".to_owned()),
            alternatives: Some(vec![Alternative {
                link_name: Some("link_name".to_owned()),
                priority: Some(1),
                target: Some("target".to_owned()),
            }]),
            arch: Some("arch".to_owned()),
            auto_installed: Some(false),
            essential: Some(false),
            fields: Some(HashMap::from([("Foo".to_owned(), "Bar".to_owned())])),
            predepends: Some(vec!["predepends".to_owned()]),
            tags: Some(vec!["tags".to_owned()]),
        }),
        rpm: Some(RpmSpecificSettings {
            arch: Some("arch".to_owned()),
            compression: Some(RpmCompression::Gzip),
            group: Some("group".to_owned()),
            packager: Some("packager".to_owned()),
            prefixes: Some(vec!["prefixes".to_owned()]),
            scripts: Some(RpmSpecificScripts {
                posttrans: Some("posttrans".to_owned()),
                pretrans: Some("pretrans".to_owned()),
                verify: Some("verify".to_owned()),
            }),
            signature: Some(RpmSignature {
                key_file: Some("key_file".to_owned()),
                key_id: Some("key_id".to_owned()),
            }),
            summary: Some("summary".to_owned()),
        }),
        apk: Some(ApkSpecificSettings {
            arch: Some("amd64".to_owned()),
            scripts: Some(ApkScripts {
                postupgrade: Some("postupgrade".to_owned()),
                preupgrade: Some("preupgrade".to_owned()),
            }),
            signature: Some(ApkSignature {
                key_file: Some("key_file".to_owned()),
                key_id: Some("key_id".to_owned()),
                key_name: Some("key_name".to_owned()),
            }),
        }),
        archlinux: Some(ArchlinuxSpecificSettings {
            arch: Some("amd64".to_owned()),
            packager: Some("packager".to_owned()),
            pkgbase: Some("pkgbase".to_owned()),
            scripts: Some(ArchlinuxSpecificScripts {
                postupgrade: Some("postupgrade".to_owned()),
                preupgrade: Some("preupgrade".to_owned()),
            }),
        }),
    };

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let alltoml =
        std::fs::read_to_string(PathBuf::from(manifest_dir).join("fixtures/all.toml")).unwrap();

    let parsed: CargoManifest = toml::from_str(&alltoml).unwrap();

    assert_eq!(&config, parsed.package.metadata.nfpm.as_ref().unwrap());
}
