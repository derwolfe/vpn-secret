# vpn-secret

This is a simple utility that grabs the vpn secret from a file and copies it to
your macOs clipboard.

For users other than me, edit
[this line](https://github.com/derwolfe/vpn-secret/blob/b38edc27303596f47af5e6bb4c82d2b01834ba5f/src/main.rs#L26) to
point to the location of your file.

## How do I use it?

if you have cargo installed you can

```
> cd /into/this/repo
# edit the file to point to the write location
> cargo install
# source your shell so that the binary can be found in your path
> vpn-secret
# this should result in the secret being present in your clipboard
```
