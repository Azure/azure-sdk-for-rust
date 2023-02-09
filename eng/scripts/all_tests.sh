#!/bin/bash

set -eux -o pipefail
cd $(dirname ${BASH_SOURCE[0]})/../../

BUILD=${1:-stable}

./eng/scripts/code_style.sh ${BUILD}
./eng/scripts/sdk_tests.sh ${BUILD}
./eng/scripts/services_tests.sh ${BUILD}
./eng/scripts/e2e_tests.sh ${BUILD}
./eng/scripts/autorust_tests.sh ${BUILD}
