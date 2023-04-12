---
layout: page
title: Quick Start
permalink: /quick-start/
---

# Quickstart Guide

Visit the [installation page](/installation/) to learn how to install `envio` on your system.


## Creating a Profile

Before you can start using `envio`, you need to create a profile. A profile is a collection of environment variables that you want to manage together.

To create a profile, run the following command:

```sh
$ envio create <profile_name>
```

Replace `<profile_name>` with the name you want to give to your profile. For example:

```sh
$ envio create myprofile
```

By default, `envio` uses passphrase encryption to protect your profile. After running the above command, `envio` will prompt you to enter a passphrase to encrypt your profile. Make sure to remember the passphrase as it will be required to decrypt your profile later.

If you want to use GPG encryption instead of passphrase encryption, use the following command:

```sh
$ envio create <profile_name> -g <gpg_key_fingerprint>
```

Replace `<gpg_key_fingerprint>` with the fingerprint of the GPG key you want to use. Or you can replace `<gpg_key_fingerprint>` with `select`, this will prompt you with a list of all the public `GPG` keys available on your system, and you can select the one you want to use.

## Adding Environment Variables to a Profile

Once you have created a profile, you can add environment variables to it using the following command:

```sh
$ envio add <profile_name> -e <key>=<value>
```

Replace `<profile_name>` with the name of the profile you want to add the environment variable to, `<key>` with the name of the environment variable you want to add, and `<value>` with the value you want to assign to the environment variable. For example:

```sh
$ envio add myprofile -e DATABASE_URL=postgres://localhost/mydb
```

You can also add multiple environment variables at once:

```sh
$ envio add myprofile -e 'DATABASE_URL=postgres://localhost/mydb MY_VERY_SECRETIVE_KEY=1234'
```

Make sure you include the environment variables you want to add in between quotes, separated by a space.

If you have environment variables stored in a file, you can import them using the following command:

```sh
$ envio create <profile_name> -f <file_name>
```

Replace `<file_name>` with the name of the file containing the environment variables you want to import.

## Updating Environment Variables in a Profile

To update an existing environment variable, use the following command:

```sh
$ envio update <profile_name> -e <key>=<new_value>
```

Replace `<profile_name>` with the name of the profile you want to update the environment variable in, `<key>` with the name of the environment variable you want to update, and `<new_value>` with the new value you want to assign to the environment variable. For example:

```sh
$ envio update myprofile -e DATABASE_URL=postgres://myhost/mydb
```

## Deleting Environment Variables from a Profile

To delete an environment variable from a profile, use the following command:

```sh
$ envio delete <profile_name> -e <key>
```

Replace `<profile_name>` with the name of the profile you want