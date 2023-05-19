# dfx

TODO

## identity

```bash
$ dfx identity list
anonymous
default *
```

```bash
$ dfx identity create <my-secure-identity-name> # creates a password protected identity
$ dfx identity use <my-secure-identity-name> # uses this identity by default
```

## pem file

```bash
$ cat $HOME/.config/dfx/identity/default/identity.pem
```
