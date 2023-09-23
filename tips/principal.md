# 🪪 Principal

Internet Computerには、Principalという識別子があります。

★TODO★

[https://5n2bt-lqaaa-aaaae-aajfa-cai.raw.icp0.io/](https://5n2bt-lqaaa-aaaae-aajfa-cai.raw.icp0.io/)

>SHA-224: Used to derive principal IDs from DER-encoded public keys, and to derive account IDs from principal IDs.

>base32: Used to cook raw principal IDs to make them friendlier to humans.

## 秘密鍵

dfxコマンドで

```bash
$ cat ~/.config/dfx/identity/default/identity.pem 
```

## 公開鍵の導出

```bash
$ openssl ec -in ~/.config/dfx/identity/default/identity.pem -pubout
read EC key
writing EC key
-----BEGIN PUBLIC KEY-----
MFYwEAYHKoZIzj0CAQYFK4EEAAoDQgAEjY+D2FbXMd2Rboh7fPVrCM8PD2/pr7Tf
kRu50wo9Ugf9cp6/oqJOgB8ik8hKt0ip494rp6yvA5czH03jlDknuw==
-----END PUBLIC KEY-----
```

## 公開鍵の詳細

```bash
$ openssl ec -in ~/.config/dfx/identity/default/identity.pem -noout -text
```

```bash
openssl ec -in ~/.config/dfx/identity/default/identity.pem -pubout | openssl ec -pubin -noout -text
read EC key
read EC key
writing EC key
Public-Key: (256 bit)
pub:
    04:8d:8f:83:d8:56:d7:31:dd:91:6e:88:7b:7c:f5:
    6b:08:cf:0f:0f:6f:e9:af:b4:df:91:1b:b9:d3:0a:
    3d:52:07:fd:72:9e:bf:a2:a2:4e:80:1f:22:93:c8:
    4a:b7:48:a9:e3:de:2b:a7:ac:af:03:97:33:1f:4d:
    e3:94:39:27:bb
ASN1 OID: secp256k1
```

## Principalの導出

★TODO★
