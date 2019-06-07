# Hashing to BLS12-381

## Prerequisites

You will need the following:

- cmake version 3.9 or newer
- OpenSSL 1.1.0 or newer (development headers)
- GMPLib 5.x or newer (development headers)
- gcc >= 8 or clang >= 7
- make
- bash
- Optional: clang-format, clang-tidy, cppcheck
- To run tests: sagemath and a working version of `dd` (GNU coreutils or similar)

## Quickstart

To set up your build directory:

    $ mkdir -p /path/to/bls_hash/c-impl/build
    $ cd /path/to/bls_hash/c-impl/build
    $ cmake ..

All further commands should be run from the `build` directory.

To build:

    $ make

You can use the `-j` switch for parallel builds, e.g.,

    $ make -j$(nproc)

To test (once you've built):

    $ make test

You can run tests in parallel, too:

    $ ctest -j 4

## Build options

You can specify a different compiler when running cmake:

    $ CC=clang CXX=clang++ cmake ..

You can also specify `CLANG_TIDY=` or `CLANG_FORMAT=` (see "other useful targets," below).

The build system supports several build targets. By default, cmake chooses the `Release`
target, which enables the usual optimizations. The `Debug` target enables debugging and
reduces the optimization level. To choose this target,

    $ cmake .. -DCMAKE_BUILD_TYPE=Debug

The following targets are supported:

- `Release` - optimizations
- `Debug` - debug symbols and `-Og`
- `RelASan` - release build with [ASan](https://en.wikipedia.org/wiki/AddressSanitizer) and
  [UBSan](https://developers.redhat.com/blog/2014/10/16/gcc-undefined-behavior-sanitizer-ubsan/)
- `DebugASan` - debug build with ASan and UBSan

And of course you can combine the above, e.g.,

    $ CLANG_TIDY=/path/to/clang-tidy CC=/path/to/clang cmake .. -DCMAKE_BUILD_TYPE=Debug

**Note:** if you want to change `CC`, `CXX`, `CLANG_TIDY`, or `CLANG_FORMAT`, you need to remove
`build/CMakeCache.txt` and re-run cmake. (This isn't necessary for `CMAKE_BUILD_TYPE`.)

## Other useful targets

To lint (you'll need `clang-tidy`):

    $ make -j$(nproc) tidy

To run cppcheck (you'll need `cppcheck`):

    $ make cppcheck

To format (you'll need `clang-format`):

    $ make format

To see all available targets,

    $ make help

# license

(C) 2019 Riad S. Wahby <rsw@cs.stanford.edu>

See the license in the toplevel directory of this repository.
