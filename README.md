# RustQIP_ICE
An ICE example for cargo fix

To reproduce:
```
$ cargo fix --edition
    Checking qipice v0.1.0 (.../git/RustQIP_ICE)
   Migrating src/lib.rs from 2018 edition to 2021
error: overflow while checking whether `FeynmanState<P>` requires drop

error: could not compile `qipice` due to previous error
warning: build failed, waiting for other jobs to finish...
error: build failed
```

Using version:
```
$  cargo version --verbose
cargo 1.56.0 (4ed5d137b 2021-10-04)
release: 1.56.0
commit-hash: 4ed5d137baff5eccf1bae5a7b2ae4b57efad4a7d
commit-date: 2021-10-04
```
