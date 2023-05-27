# Student wall 🎨

2023年5月に開催された[Motoko Bootcamp Day 3](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-3/project/README.MD)のプロジェクトをRust言語で実装します。

TODO: 対応中

## 1. Rustプロジェクト作成

Rustのプロジェクト「day3」を作成します。`cargo new`コマンドを`--lib`オプションを付与して実行します。

```
$ cargo new day3 --lib
$ cd day3
```

生成されたファイルは以下の通りです。

```
day3
├── Cargo.toml
└── src
    └── lib.rs
```

## 2. Cargo.tomlの編集

### (1) IC関連ライブラリ追加

[ic-cdk](https://docs.rs/ic-cdk/latest/ic\_cdk/)と[ic-cdk-macros](https://docs.rs/ic-cdk-macros/latest/ic\_cdk\_macros/)ライブラリを使用します。最新バージョンでよいかと思いますので、以下のように実行ます。

```bash
$ cargo add candid ic-cdk ic-cdk-macros serde
```

### (2) crate-type設定

Canister上から関数が正しく呼び出させるようcrate-typeを`cdylib`にします。

```toml
[lib]
crate-type = ["cdylib"]
```

[**Cargo.toml**](Cargo.toml)


## 3. dfx.jsonの作成

Canisterの定義を行います。

[**dfx.json**](dfx.json)

```json
{
  "canisters": {
    "day3": {
      "candid": "./day3.did",
      "package": "day3",
      "type": "rust"
    }
  },
  "defaults": {
    "build": {
      "args": "",
      "packtool": ""
    }
  },
  "version": 1
}
```

## 4. candidの作成

dfx.jsonの [canisters] > [day3] > [candid]項目に指定したファイルに、Canisterに配置するDappが提供する関数のI/Fを定義します。

[Motoko Bootcamp Day2 📺 Interface](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-3/project/README.MD#-interface)に相当するcandidを用意します。

###### [day3.did](day2.did)

```
type Content = variant {
  Text: text;
  Image: blob;
  Video: blob;
};

type Message = record {
  content: Content;
  creator: principal;
  vote: int;
};

type Result = variant {
  Ok;
  Err: text;
};

type ResultMessage = variant {
  Ok: Message;
  Err: text;
};

service : {
  deleteMessage: (nat) -> (Result);
  downVote: (nat) -> (Result);
  getAllMessages: () -> (vec Message) query;
  getAllMessagesRanked: () -> (vec Message) query;
  getMessage: (nat) -> (ResultMessage) query;
  upVote: (nat) -> (Result);
  updateMessage: (nat, Content) -> (Result);
  writeMessage: (Content) -> (nat);
};
```
