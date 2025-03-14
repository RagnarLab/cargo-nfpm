//! Rust Triple manipulation and information query.

use anyhow::Context;

/// An LLVM triple in the format: `<arch><sub>-<vendor>-<sys>-<env>`.
pub struct LlvmTriple {
    inner: String,
}

impl LlvmTriple {
    /// Triple must follow the following format:
    ///
    /// `<arch><sub>-<vendor>-<sys>-<env>`.
    pub fn new<S>(triple: S) -> anyhow::Result<Self>
    where
        S: AsRef<str>,
    {
        let inner = String::from(triple.as_ref());

        let mut split = inner.split('-');
        let _arch = split.next().context("missing architecture component")?;
        let _vendor = split.next().context("missing vendor component")?;
        let _sys = split.next().context("missing sys component")?;
        let _env = split.next().context("missing env component")?;

        Ok(Self { inner })
    }

    pub fn arch(&self) -> &str {
        let (arch, _) = self.inner.split_once('-').expect("validation wrong");
        arch
    }

    pub fn to_rpm_arch(&self) -> anyhow::Result<&'static str> {
        let arch = self.arch();
        let ret = match arch {
            "aarch64" | "arm64" | "arm64e" => "aarch64",
            "i686" => "i386",
            "x86_64" => "x86_64",
            "arm" | "armv6" | "armv6k" => "armv6hl",
            "armv7" | "armv7k" | "armv7s" | "armv7a" => "armv7hl",
            arch => return Err(anyhow::anyhow!("unsupported arch: {arch}")),
        };

        Ok(ret)
    }

    /// Architecture for `DEB` files is defined here: https://wiki.debian.org/ArchitectureSpecificsMemo
    pub fn to_deb_arch(&self) -> anyhow::Result<&'static str> {
        let arch = self.arch();
        let ret = match arch {
            "aarch64" | "arm64" | "arm64e" => "arm64",
            "i686" => "i386",
            "x86_64" => "amd64",
            "arm" | "armv6" | "armv6k" | "armv7" | "armv7k" | "armv7s" | "armv7a" => "arm",
            arch => return Err(anyhow::anyhow!("unsupported arch: {arch}")),
        };

        Ok(ret)
    }

    pub fn to_ipk_arch(&self) -> anyhow::Result<&'static str> {
        let arch = self.arch();
        let ret = match arch {
            "aarch64" | "arm64" | "arm64e" => "arm64",
            "i686" => "i386",
            "x86_64" => "x86_64",
            "arm" | "armv6" | "armv6k" => "armhf",
            "armv7" | "armv7k" | "armv7s" | "armv7a" => "armhf",
            arch => return Err(anyhow::anyhow!("unsupported arch: {arch}")),
        };

        Ok(ret)
    }

    pub fn to_apk_arch(&self) -> anyhow::Result<&'static str> {
        let arch = self.arch();
        let ret = match arch {
            "aarch64" | "arm64" | "arm64e" => "aarch64",
            "i686" => "x86",
            "x86_64" => "x86_64",
            "arm" | "armv6" | "armv6k" => "armhf",
            "armv7" | "armv7k" | "armv7s" | "armv7a" => "armv7",
            arch => return Err(anyhow::anyhow!("unsupported arch: {arch}")),
        };

        Ok(ret)
    }

    pub fn to_archlinux_arch(&self) -> anyhow::Result<&'static str> {
        let arch = self.arch();
        let ret = match arch {
            "aarch64" | "arm64" | "arm64e" => "aarch64",
            "i686" => "i686",
            "x86_64" => "x86_64",
            "arm" | "armv6" | "armv6k" => "armv6h",
            "armv7" | "armv7k" | "armv7s" | "armv7a" => "armv7h",
            arch => return Err(anyhow::anyhow!("unsupported arch: {arch}")),
        };

        Ok(ret)
    }
}
