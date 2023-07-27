# Student wall 🎨

2023年5月に開催された[Motoko Bootcamp Day 3](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-3/project/README.MD)のプロジェクトをRust言語で実装します。

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

[Motoko Bootcamp Day3 📺 Interface](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-3/project/README.MD#-interface)に相当するcandidを用意します。

※MotokoのResult (ok, err)とRust標準のResult (Ok, Err)で大文字小文字に違いがあります。MotokoとI/Fを合わせたい場合には別途Result型を定義してください。

###### [day3.did](day3.did)

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
  writeMessage: (Content) -> (nat);
  getMessage: (nat) -> (ResultMessage) query;
  updateMessage: (nat, Content) -> (Result);
  deleteMessage: (nat) -> (Result);
  upVote: (nat) -> (Result);
  downVote: (nat) -> (Result);
  getAllMessages: () -> (vec Message) query;
  getAllMessagesRanked: () -> (vec Message) query;
};
```

## 5. lib.rsの編集

`cargo new`コマンドで生成されたlib.rsの中身をクリアして、day3用のプログラムを作成します。

[Motoko Bootcamp Day 3](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-3/project/README.MD)と同じように、以下の関数を実装します。

- writeMessage()
- getMessage()
- updateMessage()
- deleteMessage()
- upVote()
- downVote()
- getAllMessages()
- getAllMessagesRanked()

###### [lib.rs](src/lib.rs)

Rust言語仕様の理解が十分でないため、作成したソースコードは所有権まわりをはじめ最適化されていない可能性がありますのでご注意ください。もしも、おかしな実装等が見つかりましたらが、ご指摘いただけますとさいわいです。

### ソース説明

#### (a) 関数名

Canisterが提供する関数の名前が`camelCase`形式なのに対し、Rustは一般的に`Snake_case`形式を推奨しているため、コンパイル時に以下のような警告が出ます。

```
warning: variable `xxx` should have a snake case name
```

先頭行に以下を入れておくことで、警告を抑止することができます。

```rust
#![allow(non_snake_case)]
```

#### (b) Content列挙型

扱うコンテンツを列挙型 (enum)として定義しています。

Textm Image, Videoのいずれかの値をとり、それぞれ異なるデータ型のデータを持ちます。

```rust
enum Content {
    Text(String),
    Image(Vec<u8>),
    Video(Vec<u8>),
}
```

#### (c) Message構造体

Contentとvote、creatorから構成される構造体を定義します。

Principal型はICのPrincipal IDを示しており投稿者も記録します。

```rust
struct Message {
    content: Content,
    vote: i128,
    creator: Principal
}
```

#### (d) Canisterの保持データ

Canister内に保持するデータは以下の2種類です。

- メッセージ自動採番用
- メッセージIDをキー、Messageデータを値とするBTreeMap


以下のようにスレッドローカルデータで保持するのが作法のようです。

```rust
thread_local! {
    static MESSAGE_ID: RefCell<u128> = RefCell::new(0);
    static WALL: RefCell<BTreeMap<u128, Message>> = RefCell::new(BTreeMap::new());
}
```

※MotokoではHashMapが使われておりますが、ここではgetAllMessages()で順序性を維持できるようBTreeMapを使用しています。

## 6. Unitテスト

TODO: IC色のあるUnitテスト方法について後日整理する

[Day 1](../day1/README.md)や[Day 2](../day2/README.md)のようにUnitテストを記述して`cargo test`を実行したところ、ロジックにICのPrincipal型が含まれることが原因で、「xxxx should only be called inside canisters.」のようなエラーが出ました。

```rust
#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn writeMessage() {
    let id = crate::writeMessage(Content::Text(String::from("TEST")));
    ︙
  }
}
```

ソース中にIC色があるとUnitテストが上手く行えないようですので、以下のいずれかの方法でテストするとよいでしょう。

- Canisterに配置してテストを行う
- IC CDKのAPIを直接呼ばずに抽象化して、テスト時はスタブを使うようにする

後者の方法として、以下の記事が参考になりそうです。

### Test your canister code even in presence of system API calls

[https://internetcomputer.org/docs/current/developer-docs/security/rust-canister-development-security-best-practices#test-your-canister-code-even-in-presence-of-system-api-calls](https://internetcomputer.org/docs/current/developer-docs/security/rust-canister-development-security-best-practices#test-your-canister-code-even-in-presence-of-system-api-calls)

## 7. Local Canisterの起動

Local Canisterを起動します。

`--background`オプションでサービス常駐でき、`--clean`を付与すると真っ新な状態でLocal canisterを起動できます。

```bash
$ dfx start --background --clean
```

## 8. Local Canisterへの配備

```bash
$ dfx deploy
```

Cargo.lockがディレクトリに存在しない場合`dfx deploy`がエラーとなりますので、`cargo generate-lockfile`を実行するとよいでしょう。
