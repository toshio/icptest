# dfx

TODO

## identity

### 登録済identity一覧

```bash
$ dfx identity list
anonymous
default *
```

### identity登録

新たにIdentityを作成します。

```bash
$ dfx identity new <identity-name>
Please enter a passphrase for your identity: ※passphraesを入力
Encryption complete.
Your seed phrase for identity 'hello': <seed-phrase>
This can be used to reconstruct your key in case of emergency, so write it down in a safe place.
Created identity: <identity-name>.
```

Identityの実体は秘密鍵で、pem形式をpassphraseで暗号化した以下のファイルが生成されます。

- ~/.config/dfx/identity/<identity-name>/identity.pem.encrypted 

passphraseで暗号化されているため、dfxコマンドの実行時にidentityを使用する際は復号化のためpassphraseの入力が求められるため、セキュリティ面で安心といえるでしょう。

### 非暗号化pemファイル

passphraseで暗号化されていないそのままのpemファイルを生成したい場合には、上記コマンドに`--disable-encryption`を付与することで、暗号化されていないpemファイルを生成することもできます。

### seed phase

コマンド実行結果にはseed phraseも表示されていますので、漏洩や紛失の無いようしっかり保管しておきましょう。seed phaseがあれば、pemファイルを復元することができます。

## 使用するidentityの切り替え

dfxコマンドでは、引数に`--identity <identity-name>`と指定すれば、コマンド毎にidentityを切り替えることができます。毎回identityを指定するのが手間な場合には、以下のように実行することで、`--identity`省略時に使用するidentityを切り替えることができます。

```bash
$ dfx identity use <identity-name>
```
