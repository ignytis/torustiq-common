#!/usr/bin/env bash

# This command builds C++ headers for dynamic linking

cbindgen --output ../torustiq-modules/torustiq_common_typedefs.hpp --lang c++ --crate torustiq-common --config cbindgen_cpp.toml