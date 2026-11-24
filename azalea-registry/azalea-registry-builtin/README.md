# `azalea-registry-builtin`

A subset of `azalea-registry` which only contains the base builtin registries.

The entire contents of this crate are re-exported by `azalea-registry`, which
you should probably be using instead.

This exists for when you want some types from `azalea-registry`, without having
to pull in the whole crate. This is mostly meant to be used interally within
Azalea to avoid cyclic dependencies.
