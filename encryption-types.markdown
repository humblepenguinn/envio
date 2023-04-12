---
layout: page
title: Encryption Types
permalink: /encryption-types/
---

## Encryption types in envio

`envio` provides two types of encryption for users to choose from based on their preferences and needs:

* Passphrase encryption using the [`age`](https://crates.io/crates/age) Rust crate.

* GPG support, implemented using the [`gpgme`](https://crates.io/crates/gpgme) Rust crate on `Unix` systems and the [`gpg4win`](https://www.gpg4win.org/) program on `Windows`.

## Passphrase

Passphrase encryption using age is the default encryption mechanism used by `envio`.

With passphrase encryption, users need to pass in a `key` while creating a [`profile`](/profiles/). It is important to remember this `key`, as it is required to perform any sort of operation on the profile, such as adding environment variables, deleting them, updating them, exporting, etc.

If the `key` is lost, the profile becomes inaccessible, and there is no way to recover the data stored in it. So, it is recommended to keep the key in a safe and secure place, or to use a password manager to store it.

## GPG

`envio` also supports GPG encryption, a widely used encryption tool that allows users to encrypt and sign their files using public-key cryptography.

On `Unix` systems, we use the `gpgme` Rust crate to provide GPG support. Users need to have the `libgpgme-dev` and `libgpg-error-dev` packages installed on their system. These package names are for Ubuntu, and users need to find the equivalent package names for their respective repo.

```bash
$ sudo apt-get -y update && sudo apt-get install -y libgpgme-dev libgpg-error-dev
```
On `Windows`, we use the `gpg4win` program to provide GPG support. Users need to have `gpg4win` installed on their system to use GPG encryption. They can install `gpg4win` using the [`chocolatey`](https://chocolatey.org/install) package manager

```pwsh
$ choco install gpg4win
```

On `macOS`, users need to have `gnupg` and `gpgme` installed. They can install these tools using [`Homebrew`](https://brew.sh/) by running the following commands in the terminal:

```bash
$ brew install gnupg gpgme
```

With `envio`'s support for both age and GPG, users can choose the encryption method that works best for them, based on their needs and preferences.