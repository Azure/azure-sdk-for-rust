#!/bin/bash

# avoid running out of disk space on GitHub Runners

set -eux -o pipefail

if [ -v CI ] && [ -v GITHUB_ACTION ] ; then
    # ref: https://github.com/actions/runner-images/issues/2840#issuecomment-790492173
    # ref: https://github.com/actions/runner-images/issues/2606#issuecomment-772683150
    # ref: https://github.com/apache/flink/blob/master/tools/azure-pipelines/free_disk_space.sh
    sudo rm -rf /opt/ghc
    sudo rm -rf /usr/local/graalvm
    sudo rm -rf /usr/local/lib/android
    sudo rm -rf /usr/local/share/boost
fi
