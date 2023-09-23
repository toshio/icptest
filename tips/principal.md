# 🪪 Principal

Internet Computerには、Principalという識別子があります。

Principalによって、Canisterを呼び出したユーザーや別のCanisterを識別することができます。

Principalはユーザーの公開鍵（または秘密鍵）から導出されますが、どのように組み立てられているかアルゴリズムの解説サイトがありましたので、実際に検証してみることにします。

## 参照元情報

[https://5n2bt-lqaaa-aaaae-aajfa-cai.raw.icp0.io/](https://5n2bt-lqaaa-aaaae-aajfa-cai.raw.icp0.io/)

>We convert a DER public key to a principal IDs by computing its SHA224 hash then appending the byte 02, indicating a self-authenticating ID in Internet Computer parlance. Like other principal IDs, we can cook self-authenticating IDs by prepending a CRC32 checksum, encoding with base32, and inserting dashes. Some tools expect these

## Principalの導出

dfxコマンドを使っている方であれば、defaultのIdentityが以下に格納されているかと思います。

```bash
~/.config/dfx/identity/default/identity.pem
```

自分のテスト環境では、以下のPrincipalでしたので、この値を 秘密鍵 → 公開鍵 → Principal という流れで求めてみることにします。

```bash
$ dfx identity get-principal
rjfvh-5kjv6-svpk2-vf3sl-h24gi-qwmfe-22o2e-jcxqg-im5h6-2apbx-hqe
```

### (1) 秘密鍵(PEM形式)から公開鍵(DER形式)を出力

```bash
$ openssl ec -in ~/.config/dfx/identity/default/identity.pem -pubout -outform DER
```

バイナリですので、中身を確認したい場合は`| hexdump -C`のようにパイプでつなげるとよいでしょう。

### (2) sha224sum

公開鍵(DER形式)のsha224ハッシュを求めます。

```bash
$ openssl ec -in ~/.config/dfx/identity/default/identity.pem -pubout -outform DER | sha224sum | cut -d ' ' -f 1
49afa557ab552ee4b3eb86442cc2935a7688915e06433a7f680f0dcf
```

### (3) 末尾に'02'追加 (Principal ID raw)

```bash
$ openssl ec -in ~/.config/dfx/identity/default/identity.pem -pubout -outform DER | sha224sum | cut -d ' ' -f 1 | sed s/$/02/
49afa557ab552ee4b3eb86442cc2935a7688915e06433a7f680f0dcf02
```

末尾に付与するbyteは以下のようです。

|byte|Description                        |
|:---|:----------------------------------|
|0x01|canister                           |
|0x02|public key (self-authenticating ID)|
|0x04|anonymous                          |

### (4) CRC32計算

Principal ID (raw)のCRC32を計算します。

```
$ echo -n 49afa557ab552ee4b3eb86442cc2935a7688915e06433a7f680f0dcf02 | xxd -r -p |python3 -c 'import sys;import zlib;print("{:x}".format(zlib.crc32(sys.stdin.buffer.read())%(1<<32)))' 
8a4b53f5
```

### (5) 先頭にCRC32を追加してBase32

CRC32値 (`8a4b53f`)をPrincipal ID (raw)の先頭に追加した値をBase32エンコードします。

```bash
echo -n 8a4b53f549afa557ab552ee4b3eb86442cc2935a7688915e06433a7f680f0dcf02 | xxd -r -p | base32
RJFVH5KJV6SVPK2VF3SLH24GIQWMFE22O2EJCXQGIM5H62APBXHQE===
```

### (6) 小文字化して、5桁ずつ区切る

Base32エンコード値を見やすいように整形します。

```bash
$ echo -n 8a4b53f549afa557ab552ee4b3eb86442cc2935a7688915e06433a7f680f0dcf02 | xxd -r -p | base32 | tr A-Z a-z | tr -d "=" | sed -E 's/(.{5})/\1-/g'
rjfvh-5kjv6-svpk2-vf3sl-h24gi-qwmfe-22o2e-jcxqg-im5h6-2apbx-hqe
```

## Principal導出スクリプト

上記の内容をまとめたスクリプトを用意してみました。

[derivePrincipal.sh](derivePrincipal.sh)

### 使用方法

```bash
$ bash derivePrincipal.sh <ECDSA private pem file>
```

あくまでも学習目的で用意したものにすぎませんので内容は保証しません。

実際の運用では、公式の`dfx identity get-principal`コマンドなどをお使いください。
