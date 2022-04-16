#!/bin/sh

set -e

bindgen src/metal_cpp_rs_bindings.cpp \
    --no-layout-tests \
    -o src/metal_cpp_rs_bindings.rs \
    --allowlist-type MTL::.* \
    --allowlist-function MTL::.* \
    --allowlist-type NS::.* \
    --allowlist-function NS::.* \
    --vtable-generation \
    --enable-cxx-namespaces \
    -- \
        -xc++ \
        -std=c++17 \
        -I./metal-cpp \
        -I./metal-cpp-extensions \
        -fno-objc-arc \
        -isysroot$(xcrun --sdk macosx --show-sdk-path)
