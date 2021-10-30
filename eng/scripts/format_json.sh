find test -name "*.json" -exec bash -c 'jq -S . {} > {}.tmp && mv {}.tmp {}' \;
