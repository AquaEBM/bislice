# `bislice`

Simple data types that allow treating a pair of slices as logically contiguous, as if they were one slice.

This crate basically provides data types that contain two slices, and equips those types with methods mimicking Rust's native slice API as closely as possible.

Typically for use with ring buffers, since reading chunks returns two slices, because of wrap-around.
