#!/bin/bash


set -eoux pipefail

#release-1.0
VERSION="release-1.0"

APIS=(
    advancedcronjobs
    broadcastjobs
    statefulsets
    containerrecreaterequests
    daemonsets
    ephemeraljobs
    imagepulljobs
    nodeimages
    resourcedistributions
    sidecarsets
    uniteddeployments
    workloadspreads
    podunavailablebudgets
)

rm -rf src/apis/

mkdir -p src/apis/

echo "// WARNING! generated file do not edit" > src/apis/mod.rs

for API in "${APIS[@]}"
do
    echo "generating  api ${API}"
    curl -sSL "https://raw.githubusercontent.com/openkruise/kruise/release-1.0/config/crd/bases/apps.kruise.io_${API}.yaml"  | kopium  -f - > src/apis/${API}.rs
    echo "pub mod ${API};" >> src/apis/mod.rs
done
