## asym_enc

Encrypt/Decrypt files using public key cryptography.

### Build and usage

 - Install `rustc` and `cargo` from https://rustup.rs/
 - This relies on Open-SSL bindings, and so it needs `libssl-dev`
 - `cd` into the directory and run `cargo run -- -h` to compile and run the application.

### Note on usage

During encryption, this needs the public key (in PEM format). It encrypts the file using AES-256 (in CBC mode), encrypts the key with the given public key and writes both to the output.

``` bash
$ md5sum example/input
746308829575e17c3331bbcb00c0898b  example/input
$ cargo run -- -m enc -i example/input -o example/output -p example/public.pem -k example/key
   Compiling asym_enc v0.1.0 (file:///home/wafflespeanut/Desktop/surematics/asym_enc)
    Finished dev [unoptimized + debuginfo] target(s) in 2.77 secs
     Running `target/debug/asym_enc -m enc -i example/input -o example/output -p example/public.pem -k example/key`
```

This generates `<output-file>` and `<encrypted-aes-key>`

During decryption, it needs the private key (in PEM format). It decrypts the key, decrypts the file, and then writes the output.

``` bash
$ cargo run -- -m dec -i example/output -o example/input -p example/private.pem -k example/key
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/asym_enc -m dec -i example/output -o example/input -p example/private.pem -k example/key`
$ md5sum example/input
746308829575e17c3331bbcb00c0898b  example/input
```

Note that asymmetric encryption is used only for sharing the generated AES key, since encrypting/decrypting using the public/private keys is very expensive, and it's perceivable in large files.
