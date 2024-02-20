#!/bin/bash
LT_BUILD_UUID=$(uuidgen)-$(date -u +"%Y-%m-%d-%H-%M-%S")

echo "LT_BUILD_UUID=$LT_BUILD_UUID" 

echo "#pragma once" > src/rtc_base/uuid.h
echo "const char* build_uuid = \"$LT_BUILD_UUID\";" >> src/rtc_base/uuid.h

if [[ "$OSTYPE" == "darwin"* ]]; then
    ./build_macos.sh --arch arm64
    mkdir -p lib
    cp -r mac-arm64-release/* .
else
    ./build_linux.sh --arch x64
    mkdir -p lib
    cp -r linux-x64-release/* . > /dev/null 2>&1
fi
