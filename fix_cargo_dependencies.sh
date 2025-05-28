#!/bin/bash

echo "🔍 Scanning Cargo.toml files for duplicate [dependencies]..."

find . -name "Cargo.toml" | while read -r file; do
    dep_lines=()
    output=()
    in_dependencies=0

    while IFS= read -r line; do
        if [[ $line == "[dependencies]" ]]; then
            if [[ $in_dependencies -eq 1 ]]; then
                continue  # skip duplicate headers
            fi
            in_dependencies=1
            output+=("[dependencies]")
            continue
        fi

        if [[ $line == * ]]; then
            in_dependencies=0
            output+=("$line")
            continue
        fi

        if [[ $in_dependencies -eq 1 && $line =~ ^[[:space:]]*[a-zA-Z0-9_-]+[[:space:]]*=.* ]]; then
            crate=$(echo "$line" | cut -d= -f1 | xargs)
            if [[ " ${dep_lines[*]} " != *" $crate "* ]]; then
                dep_lines+=("$crate")
                output+=("$line")
            fi
        else
            output+=("$line")
        fi
    done < "$file"

    echo "${output[@]}" > "$file"
    echo "✅ Fixed: $file"
done

echo "🎯 All Cargo.toml files processed."
