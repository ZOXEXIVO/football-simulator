#!/bin/bash

ROOT_DIR="../src"

find "$ROOT_DIR" -name "Cargo.toml" -execdir sh -c '
    project_dir=$(pwd);

    cargo update --manifest-path "$project_dir/Cargo.toml";
    cargo upgrade --manifest-path "$project_dir/Cargo.toml";
' {} \;