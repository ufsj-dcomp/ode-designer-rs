# ODE-Designer

## How to install

It is recommended to use the executables available in *Releases* on GitHub. [This link](https://github.com/ufsj-dcomp/ode-designer-rs/releases/latest) can be used to always redirect to the latest version.

***

### Linux

The Linux distribution uses [AppImages](https://appimage.org/), which requires a runtime (`fuse2`) installable on all Linux-based distributions. It is commonly included by default by some distributions, or is already installed by another program.

> [!NOTE]
> Although AppImages can make binaries super portable in Linux, there's still a
> hard dependency on your system's GLIBC, which cannot be redistributed. As it
> stands, the **minimum supported version is 2.28**. You can check your system's
> version with this command:
> ```sh
> ldd --version
> ```
>
> If your system meets this requirement, but you still get errors related to
> GLIBC, please open an issue and we'll look into it.

<details>
<summary><b>🐧 For Debian/Ubuntu/Pop_OS!/ElementaryOS</b></summary>

```sh
$ sudo apt install libfuse2
```

</details>

<details>
<summary><b>🐧 For ArchLinux</b></summary>

```sh
$ sudo pacman -S fuse2
```

</details>

***

### Windows

The Windows distribution consists of a ZIP that can be extracted and its contents executed.

## Compilation

### Using Docker

You can use the provided Dockerfile to build your own AppImage with no build tools installed directly on your system. Simply run:

```sh
$ docker build -t ode-designer-appimage-builder --output=. .
```

After this execution, the AppImage should be available in `./ode-designer.AppImage`. This AppImage, as well as those found in *Releases*, include Python and the `scipy` and `matplotlib` dependencies for code generation, interactive simulation and PDF export.

### Manually

To compile and run the software outside of the AppImage, the Rust toolchain (nightly version) is required, as well as Python >= 3.11, and the dependencies listed in `requirements.txt`. Fulfilling these requirements, simply compile like any Rust project, running

```sh
$ cargo run
```

Or

```sh
$ cargo build --release
$ ./target/release/ode-designer-rs
```

If anything goes wrong, try

```sh
$ git submodule update --init --recursive
$ git submodule update --recursive --remote
```

Then, recompile and run.