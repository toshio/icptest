# 4. ic_cdk::call()

あるCanisterから別のCanisterの機能を呼び出す方法について解説します。

以下の資料を参考にしています。

[https://internetcomputer.org/docs/current/developer-docs/backend/rust/intercanister](https://internetcomputer.org/docs/current/developer-docs/backend/rust/intercanister)

このサンプルでは、2つのCanister間のやりとりを、Publisher-Subscriberパターンを使っています。

送信側と受信側を結合せず、アプリケーションから関心を持っている複数のコンシューマーに対して非同期的にイベントを通知できるようにする仕組みです。

- メッセージを送信する側：Publisher
- メッセージを受け取る側：Subscriber

Publisherが送信したメッセージはトピックという送信先に送られます。トピックに送信先のCanister Idを紐づける「subscribe」を行うことで、送信したいSubscriberへと送ることができる仕組みです。

## 1. Rust Workspaceの作成

### (1) ディレクトリ構成

以下の2つのCanisterを作成するため、Rustの`cargo workspace`を使用します。

- publisher
- subscriber

```
icptest
 ├─ src
 │   ├─ publisher
 │   │   ├─ src/lib.rs
 │   │   └─ Cargo.toml
 │   └─ subscriber
 │       ├─ src/lib.rs
 │       └─ Cargo.toml
 ├─ Cargo.toml
```

```toml
[workspace]
members = [
    "src/publisher",
    "src/subscriber"
]
```

※個人的には、icptest直下のsrcディレクトリは不要として各プロジェクトディレクトリでもよいかと考えていますが、公式サンプルと同じ構成としています。

### (2) プロジェクトディレクトリ作成

```bash
$ mkdir icptest
$ cd icptest
```

### (3) [Cargo.toml](Cargo.toml)作成

```toml
[workspace]
members = [
    "src/publisher",
    "src/subscriber"
]
```

### (4) [dfx.json](dfx.json)作成

```json
{
  "version": 1,
  "canisters": {
    "publisher": {
      "package": "publisher",
      "candid": "src/publisher/src/publisher.did",
      "type": "rust"
    },
    "subscriber": {
      "package": "subscriber",
      "candid": "src/subscriber/src/subscriber.did",
      "type": "rust"
    }
  }
}
```

## 2. 'publisher' Canister作成

### (1) Rustプロジェクト作成

```bash
$ mkdir src
$ cd src
$ cargo new publisher --lib
$ cd publisher
```

### (2) [src/publisher/Cargo.toml](src/publisher/Cargo.toml)編集

#### a. crate-type追加

ライブラリセクションを追加して、crate-typeにcdylibを指定します。cdylib を指定することで最終成果物の .wasm ファイルを動的ライブラリにします。

```toml
[lib]
crate-type = ["cdylib"]
```

#### b. dependencies追加

```bash
$ cargo add candid ic-cdk ic-cdk-macros serde serde_derive
```

### (3) [src/publisher/src/publisher.did](src/publisher/src/publisher.did)作成

```
type Counter = record {
    topic:text;
    value:nat64;
};
type Subscriber = record {
    topic:text;
  };
service : {
     "subscribe": (subscriber:Subscriber) -> ();
     "publish": (counter : Counter) -> ();
}
```

### (4) [src/publisher/src/lib.rs](src/publisher/src/lib.rs)作成

## 3. 'subscriber' canister作成

### (1) Rustプロジェクト作成

```bash
$ cd src
$ cargo new subscriber --lib
```

### (2) [src/subscriber/Cargo.toml](src/publisher/Cargo.toml)編集

#### a. crate-type追加

ライブラリセクションを追加して、crate-typeにcdylibを指定します。cdylib を指定することで最終成果物の .wasm ファイルを動的ライブラリにします。

```toml
[lib]
crate-type = ["cdylib"]
```

#### b. dependencies追加

```bash
$ cargo add candid ic-cdk ic-cdk-macros serde serde_derive
```

### (3) [src/subscriber/src/subscriber.did](src/subscriber/src/subscriber.did)作成

```
type Counter = variant {
    topic : text;
    value:nat64;
};
type Subscriber = variant {
    topic:text;
  };
service : {
     "setup_subscribe": (publisher_id:principal,topic:text) -> ();
     "update_count": (counter : Counter) -> ();
     "get_count": () -> (nat64);
}
```

#### (4) [src/subscriber/src/lib.rs](src/subscriber/src/lib.rs)作成


### 4. サービス起動

```bash
$ dfx start --clean --background
```

### 5. deploy

dfx.jsonのあるディレクトリに移動した上で、以下を実行して下さい。

```bash
$ dfx deploy
Deploying all canisters.
Creating a wallet canister on the local network.
The wallet canister on the "local" network for user "default" is "bnz7o-iuaaa-aaaaa-qaaaa-cai"
Creating canisters...
Creating canister publisher...
︙
```

もし、Cargo.lockファイルがなければ、dfxコマンドがエラーとなりますので、以下で作成しておくとよいでしょう。

```bash
$ cargo generate-lockfile
```

### 6. 動作確認

#### (1) setup_subscribe呼び出し

第1引数にPublisherのCanister Id、第2引数にTopicを指定して、subscribeの設定を行います。

```bash
$ dfx canister call subscriber setup_subscribe "(principal \"$(dfx canister id publisher)\",\"ICP\")"
()
```

Publisherのcanister Idは、`dfx canister id publisher`で取得できますから、シェルスクリプトのCommand Substitution（コマンド置換）を使って第1引数に指定するとよいでしょう。それに合わせて、dfx call コマンドの引数は単一引用符(')から、二重引用符(")に変更し、引数内の文字列を括る二重引用符(")の前にエスケープ記号 (`\`)を指定するとよいでしょう。

topicは自由に設定できますが、ここでは `ICP` とします。

#### (2) publish呼び出し

publisherに対して、指定したトピックでpublishしてみます。

```bash
$ dfx canister call publisher publish '(record {topic="ICP"; value=1})'
()
```

#### (3) get_count呼び出し

subscriberに対して、指定したトピックのカウントを取得してみます。publishした値が反映されていることが確認できます。

```bash
$ dfx canister call subscriber get_count
(1 : nat64)
```
