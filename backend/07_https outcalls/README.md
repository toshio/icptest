# HTTPS outcalls

Canister上から外部のWebサーバに対してhttpリクエストを発行することができます。

今回のサンプルでは、『[1. Hello](../01_hello/README.md)』と同じインタフェースを持つDappとし、CanisterからさらにWebサーバへリクエストし応答メッセージを呼び出し元へ返すようにしてみます。

## 0. リクエスト先外部Webサーバ準備

今回はテスト用にJSONを返すリクエスト先の外部Webサーバを用意することにします。

手軽にWebサーバを公開できる[Deno Deploy](https://deno.com/deploy)を使用し、サーバが返すJSONデータ内には、複数リクエストを行っても変わらない固定部分と、リクエストごとに異なる可変部分を含めることにしましょう。

##### [https://jsontest.deno.dev/?name=ICP](https://jsontest.deno.dev/?name=ICP)

```json
{
  "message": "Hello, ICP",
  "timestamp": "2023-11-25T00:50:16.252Z"
}
```

##### Deno Deploy用ソースコード(例)

```javascript
Deno.serve((req: Request) => {
  const url = new URL(req.url);
  const name = url.searchParams.get('name') || "world";
  const data = {
      "message": `Hello, ${name}`,
      "timestamp": new Date()
  };

  return new Response(
    JSON.stringify(data, null, 2), {
      status: 200,
      headers: {
        "content-type": "application/json",
      },
    });
});
```

## 1. プロジェクトの作成

今回もBackendのみですので、`dfx new`コマンドは使わず、Rustの一般的なライブラリプロジェクトからCanister用のDappをつくることにします。

「icptest」はプロジェクト名ですので、任意に設定して構いません。

## 1. プロジェクトの作成

```bash
$ cargo new icptest --lib
$ cd icptest
```

## 2. プロジェクト資材準備

### (1) dfx.jsonの作成

『[1. Hello](../01_hello/README.md)』と同じdfx.jsonを用意します。

###### [dfx.json](dfx.json)

```json
{
  "canisters": {
    "backend": {
      "candid": "./backend.did",
      "package": "icptest",
      "type": "rust"
    }
  },
  "defaults": {
    "build": {
      "args": "",
      "packtool": ""
    }
  },
  "output_env_file": ".env",
  "version": 1
}
```

### (2) didファイルの作成

『[1. Hello](../01_hello/README.md)』と同じbackend.didを用意します。

###### [backend.did](backend.did)

```
service : {
    "greet": (text) -> (text) query;
}
```

### (3) [Cargo.toml](Cargo.toml)編集

『[1. Hello](../01_hello//README.md)』と同じように、libセクションに`crate-type = ["cdylib"]`を追記して、依存ライブラリを追加します。

今回は`candid`, `ic-cdk`にくわえ、URLエンコードのために`urlencoding`も入れておきます。

```bash
$ cargo add candid ic-cdk
$ carg add urlencoding
$ cargo add serde --no-default-features --features derive
$ cargo add serde_json
```

```ini
[package]
name = "icptest"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
candid = "0.10.0"
ic-cdk = "0.12.0"
urlencoding = "2.1.3"
```
