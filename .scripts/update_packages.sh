#!/bin/bash

ROOT_DIR=".."

find "$ROOT_DIR" -name "Cargo.toml" -execdir sh -c '
    project_dir=$(pwd);

    echo "Updating $project_dir";

    #cargo update --manifest-path "$project_dir/Cargo.toml";
    #cargo upgrade --manifest-path "$project_dir/Cargo.toml";
' {} \;