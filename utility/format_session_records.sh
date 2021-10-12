find . -wholename "*/SessionRecords/*.json" -exec bash -c 'jq . {} > {}.tmp && mv {}.tmp {}' \;
