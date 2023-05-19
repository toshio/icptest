## quill

quillを使う。

https://internetcomputer.org/docs/current/references/quill-cli-reference/

## インストール方法

以下のURLから最新モジュールをダウンロードする。

https://github.com/dfinity/quill/releases

|quill-arm_32|
|quill-linux-x86_64|
|quill-linux-x86_64-musl|
|quill-macos-x86_64|
|quill-windows-x86_64.exe|

LinuxなどのOSではchmodで実行権限を付与しておく。環境変数PATHが通っているディレクトリにバイナリを置いておくと良いでしょう。

## ウォレット作成

```
$ quill generate
Principal id: xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxxxx-xxx
Account id: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

seed.txtファイルが出力される。

