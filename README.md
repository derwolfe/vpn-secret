# vpn-secret

This is a simple utility that grabs the vpn secret from an encrypted file and copies it to
your macOs clipboard.

This can be configured to take the path of the file, if this path won't change, you can add a simple an alias to the full command, e.g. for zsh

```shell
alias vpn-password='vpn-secret -f ~/path/to/encrypted/vpn-file.gpg"`
```

## How do I get it?

If you trust me, just download the prebuilt binary from github.

If you don't (why should you), you can use cargo to build and install.

if you have cargo installed you can

```
> cd /into/this/repo
# edit the file to point to the write location
> cargo install
> vpn-secret -f ~/path/to/encrypted/vpn.gpg file
```
