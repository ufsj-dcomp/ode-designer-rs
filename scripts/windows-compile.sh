#!/usr/bin/env bash
set -e

die() {
    echo Error: $@
    exit 1
}

# build information
buildtype=debug

# Parse args
for arg in "$@"; do
    case $arg in
        --release)
            buildtype=release
            ;;
    esac
done

if [[ ! -f Cargo.toml ]]; then
    die "Can only run in the same directory as project root."
fi

if ! which rustc > /dev/null; then
    die "Rust appears to not be instaled"
fi

target='x86_64-pc-windows-gnu'

echo "Rust version: $(rustc --version)"
echo "Build type: $buildtype"
echo "Target: $target"

args=()

if [[ $buildtype == release ]]; then
    args+=(--release)
fi

cargo build ${args[@]}

files=()

push-file() {
    if [[ -f "$1" ]]; then
        files+=("$1")
    else
        die "Failed to find file $1"
    fi
}

push-file "target/$target/$buildtype/ode-designer-rs.exe"

push-file "/usr/x86_64-w64-mingw32/bin/libstdc++-6.dll"

echo ${files[@]}

zip -j windows-build.zip "${files[@]}"
