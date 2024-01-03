#!/bin/bash
# Script runtime dependencies: grep , ldd , mkdir , cp , appimagetool , echo , date
# Variables local to this script are lowercase, while the uppercase ones are
# external.

# Fail on any error
set -e

# Setup common variables

script_path=$(dirname "$(realpath -s "${BASH_SOURCE[0]}")")

# On Cargo.toml: name = "..."
# This regex extracts the `...` part
application_name=$(grep -Po '(?<=name\s=\s")[\w-]+' Cargo.toml)

appdir_target=./target/${application_name}.AppDir

appimage_target_dir=./target/appimage/
appimage_target=${appimage_target_dir}/${application_name}.AppImage

release_app=./target/release/$application_name

# ------------------------------------------------------------------------------

function log() {
    local color=""
    local type=${1^^}
    local message=${@:2}

    local blue=34
    local purple=35
    local yellow=33
    local red=31

    local dimmed_color=2
    local unset_style=0

    case $type in
        INFO) color=$blue ;;
        DEBUG) color=$purple ;;
        WARN) color=$yellow ;;
        ERROR) color=$red ;;
    esac

    local curr_time=$(date +"%T")

    echo -e "\033[${dimmed_color}m${curr_time} \033[${unset_style};${color}m${type}\033[m ${message}"
}

function copy_python_base_appimage() {
    # This function expects that the appimage has already been extracted and
    # that the our use-case dependencies have already been installed with PIP

    # External variable PYTHON_BASE_APPIMAGE_PATH
    cp -r $PYTHON_BASE_APPIMAGE_PATH -T $appdir_target

}

function copy_dependencies() {
    # Returned by `ldd`: dynlibname => dynlib_full_path
    # This regex extracts `dynlib_full_path`
    local dependencies=$(ldd $release_app | grep -Po "(?<==>\s)[\w\d/\.\+-]+")

    # Excludes partial matches from this file
    local final_dependencies=$(grep -vf $script_path/skip-dependencies.txt <<< "$dependencies")

    for dynlib in $final_dependencies
    do
        mkdir -p $appdir_target/$(dirname $dynlib)
        cp $dynlib $appdir_target/$dynlib
    done

}

function package_appdir() {
    # Copy rust binary
    mkdir -p $appdir_target/bin
    cp $release_app -t $appdir_target/bin

    # Remove leftover .desktop files/AppRun/icons from base AppImage
    rm $appdir_target/*.desktop
    rm $appdir_target/*.png
    rm $appdir_target/AppRun

    # Copy extra AppImage files
    cp -r appimage/merge -T $appdir_target

    mkdir -p $appimage_target_dir
    appimagetool $appdir_target $appimage_target
}

# ------------------------------------------------------------------------------

function main() {
    log INFO building app
    cargo build --release
    log INFO done building app

    log INFO copying base python appimage
    copy_python_base_appimage
    log INFO done copying base python appimage

    log INFO copying runtime library dependencies
    copy_dependencies
    log INFO done copying runtime library dependencies

    log INFO packaging appdir into appimage
    package_appdir
    log INFO done packaging appdir into appimage
}

main
