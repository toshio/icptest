# quill

[https://internetcomputer.org/docs/current/references/quill-cli-reference/](https://internetcomputer.org/docs/current/references/quill-cli-reference/)

## インストール方法

以下のURLから最新モジュールをダウンロードします。

[https://github.com/dfinity/quill/releases](https://github.com/dfinity/quill/releases)

|モジュール              |
|:-----------------------|
|quill-arm_32            |
|quill-linux-x86_64      |
|quill-linux-x86_64-musl |
|quill-macos-x86_64      |
|quill-windows-x86_64.exe|

LinuxなどのOSではchmodコマンドで実行権限を付与しておく。環境変数PATHが通っているディレクトリにバイナリを置いておくと良いでしょう。

## ウォレット作成

```
$ quill generate
Principal id: xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxx
Account id: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

seed.txtファイルが出力されます。

## 残高確認

```bash
$ quill account-balance <Account id>
```


## Seed phrase からのpem 出力

Seed phraseからpemファイルを生成するコマンド

```bash
$ quill generate --phrase '…' --pem-file <出力ファイル名.pem>
```

引数で指定したSeed phraseから、seed.txtファイルと、`--pem-file`で指定した対応するpemファイルが出力されます。

## pemファイルのPrincipal、およびAccount Idの表示

```bash
$ quill public-ids --pem-file <pem-file>
Principal id: xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxx
Legacy account id: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

## SeedファイルのPrincipal、およびAccount Idの表示

`quill generate`で生成されたseed.txtファイルを引数に指定します。

```bash
$ quill public-ids --seed-file <seed-file>
Principal id: xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxx
Legacy account id: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

## Seed phraseからのPrincipal、およびAccount Idの表示

`quill public-ids`には、パラメータから直接Seed phraseを指定する方法がないのでSeedファイルを出力して引数に指定します。

bashの場合には、`<(…)`のようにプロセス置換 (process substitution)を使ってコマンドラインに指定することはできます。

```bash
$ quill public-ids --seed-file <(echo "xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx xxxxx")
```

## opensslを使った秘密鍵作成

以下に記載されている通り、『ECDSA on curve P-256 (secp256r1)』で秘密鍵が使用できますので、Seed phraseは生成されませんが、opensslコマンドを使うことも可能です。

[https://internetcomputer.org/docs/current/references/ic-interface-spec](https://internetcomputer.org/docs/current/references/ic-interface-spec)

>Ed25519 and ECDSA signatures
>Plain signatures are supported for the schemes
>Ed25519 or
>ECDSA on curve P-256 (also known as secp256r1), using SHA-256 as hash function, as well as on the Koblitz curve secp256k1.

### opensslでサポートを確認

```bash
$ openssl ecparam -list_curves | grep secp256k1
secp256k1 : SECG curve over a 256 bit prime field
```

### opensslによる秘密鍵生成とPrincipal、およびAccount Idの表示

```bash
$ openssl ecparam -name secp256k1 -genkey -noout -out <pem-file>
$ quill public-ids --pem-file <pem-file>
Principal id: xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxx
Legacy account id: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```
