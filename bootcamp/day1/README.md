# Calculator ➕✖️➖➗

2023年5月に開催された[Motoko Bootcamp Day 1](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-1/project/README.MD)のプロジェクトをRust言語で試してみました。

## 1. Rustプロジェクト作成

Rustのプロジェクト「day1」を作成します。`cargo new`コマンドを`--lib`オプションを付与して実行します。

```
$ cd days
$ cargo new day1 --lib
$ cd day1
```

生成されたファイルは以下の通りです。

```
day1
├── Cargo.toml
└── src
    └── lib.rs
```

## 2. Cargo.tomlの編集

### (1) IC関連ライブラリ追加

[ic-cdk](https://docs.rs/ic-cdk/latest/ic\_cdk/)と[ic-cdk-macros](https://docs.rs/ic-cdk-macros/latest/ic\_cdk\_macros/)ライブラリを使用します。最新バージョンでよいかと思いますので、以下のように実行ます。

```bash
$ cargo add ic-cdk ic-cdk-macros
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
    "day1": {
      "candid": "./day1.did",
      "package": "day1",
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

## 3. candidの作成

dfx.jsonの`[canisters]` > `[day1]` > `[candid]`項目に指定したファイル名で、Canisterが提供する関数を定義します。

[Motoko Bootcamp Day1](./)のら、

[**day1.did**](day1.did)

```
service : {
    add: (float64) -> (float64);
    sub: (float64) -> (float64);
    mul: (float64) -> (float64);
    div: (float64) -> (opt float64);
    reset: () -> ();
    see: () -> (float64) query;
    power: (float64) -> (float64);
    sqrt: () -> (float64);
    floor: () -> (int);
}
```

参考：Candidリファレンス

{% embed url="https://internetcomputer.org/docs/current/references/candid-ref" %}

## 4. lib.rsの編集

`cargo new`コマンドで生成されたlib.rsの中身をクリアして、day1用のプログラムを作成します。

[Motoko Bootcamp Day1](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-1/project/README.MD)と同じように、以下の関数を実装します。

* add()
* sub()
* mul()
* div()
* reset()
* see()
* power()
* sqrt()
* floor()

[**lib.rs**](src/lib.rs)

## 4. Unitテスト

Rustではソース内にUnitテストコードを含めて記述することができます。

TODO: 今回の範囲ではロジックにIC色は無いため、Local canisterに配備せずそのまま実行する方法とします。

```bash
$ cargo test
```

## 6. Local Canisterの起動

Local Canisterを起動します。

`--background`オプションでサービス常駐でき、`--clean`を付与すると真っ新な状態でLocal canisterを起動できます。

```bash
$ dfx start --background --clean
```

## 7. Deploy

```bash
$ dfx deploy
```

### Cargo.lockが無い場合

Cargo.lockがディレクトリに存在しない場合、`dfx deploy`がエラーとなります。

その場合、たとえば、以下のように作成するとよいでしょう。

```bash
$ cargo generate-lockfile
```

### 作成資材

```
.
├── Cargo.lock
├── Cargo.toml
├── day1.did
├── dfx.json
└── src
    └── lib.rs
```

| File       |
| ---------- |
| Cargo.lock |
| Cargo.toml |
| day1.did   |
| dfx.json   |
| src/lib.rs |

* `dfx start`コマンドを実行すると`.dfx`ディレクトリが作成されます。
* `cargo test`コマンドを実行すると`target/debug`ディレクトリが作成されます。
* `dfx deploy`コマンドを実行すると`target/release`ディレクトリ、および`target/wasm32-unknown-unknown`が作成されます。
