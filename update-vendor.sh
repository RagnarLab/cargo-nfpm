#!/usr/bin/env bash
set -euo pipefail

VERSION="$1"
osname=$(uname)
osarch=$(uname -m)
tmpdir=$(mktemp -d)

curl -LfsS "https://github.com/goreleaser/nfpm/releases/download/v${VERSION}/checksums.txt" > nfpm_checksums.txt
sed -i "s/^\(const NFPM_VERSION: &str = \"\).\+\(\";\)$/\1${VERSION}\2/g" src/nfpm.rs

curl -LfsS -o "${tmpdir}/nfpm.tar.gz" \
    "https://github.com/goreleaser/nfpm/releases/download/v${VERSION}/nfpm_${VERSION}_${osname}_${osarch}.tar.gz"
tar xzf "${tmpdir}/nfpm.tar.gz" -C "$tmpdir"
"${tmpdir}/nfpm" jsonschema > ./extras/schema.json

rm -rf "$tmpdir"
