find test -name "*.json" -exec bash -c 'jq . {} > {}.tmp && mv {}.tmp {}' \;
