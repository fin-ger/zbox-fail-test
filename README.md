# zbox test for a failed machine

This repository contains a test that shows how zbox is (currently) unable to handle an unexpected machine failure while writing to disk.

## How to reproduce the test?

### Preparation phase

In order to run this test you need the following packages installed on your system:

 - `coreutils` (tested with v8.30)
 - `curl` (tested with 7.63.0)
 - `expect` (tested with 5.45.4)
 - `qemu` (tested with 3.1.0)
    - make sure you have the following binaries installed by the qemu package:
       - `qemu-img`
       - `qemu-system-x86_64`

When all dependencies are satisfied run the VM prepare step:

```
$ ./scripts/setup-vm.exp
```

This will setup an alpine linux vm inside the `./qemu` folder.

### Run the test

```
$ ./scripts/run-test.exp
```

This will run a statically linked binary of the rust project that is included in this repository. The `run` action of the rust program will be interrupted after 20 seconds, which will lead to an interrupted write action in the zbox repository.

### Check the repository for inconsistencies

```
$ ./scripts/run-check.exp
```

This will try to validate the repository contents against the expected data. Currently, this always fails.

## How to run the test on your local machine?

> This will not fail, a full machine failure is needed to reproduce this error

1. Create a reference file for the test:

```
$ dd if=/dev/urandom of=data bs=1K count=128
```

2. Run the test

```
$ cargo run -- --file data run
```

3. Check the repository

```
$ cargo run -- --file data check
```
