# Calculator ➕✖️➖➗

2023年5月に開催された[Motoko Bootcamp Day 1](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-1/project/README.MD)のプロジェクトをRust言語で試してみました。

Local Canisterへの配備まで実施した手順を記載します。

## 1. Rustプロジェクト作成

Rustのプロジェクト「day1」を作成します。`cargo new`コマンドを`--lib`オプションを付与して実行します。

```
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

[ic-cdk](https://docs.rs/ic-cdk/latest/ic\_cdk/)を使用します。最新バージョンでよいかと思いますので、以下のように実行ます。

```bash
$ cargo add ic-cdk
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

dfx.jsonの記述に関する公式ドキュメントは[こちら](https://internetcomputer.org/docs/current/references/dfx-json-reference)が参考になります。

## 4. candidの作成

dfx.jsonの [canisters] > [day1] > [candid]項目に指定したファイルに、Canisterに配置するDappが提供する関数のI/Fを定義します。

[Motoko Bootcamp Day1 📺 Interface](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-1/project/README.MD#-interface)に相当するcandidを用意します。

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

### 参考：Candidリファレンス

{% embed url="https://internetcomputer.org/docs/current/references/candid-ref" %}

## 5. lib.rsの編集

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

Canister内部で保持するデータは、[公式チュートリアル・サンプル](https://internetcomputer.org/docs/current/developer-docs/backend/rust/rust-profile)にもあるように、「thread_local!」内で管理すると良いようです。

###### [lib.rs](src/lib.rs)

```rust
use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<f64> = RefCell::new(0f64);
}

#[ic_cdk::update]
fn add(x: f64) -> f64 {
    COUNTER.with(|counter| {
        let mut c = counter.borrow_mut();
        *c += x;
        *c
    })
}
︙
```

## 6. Unitテスト

Rustではソース内にUnitテストコードを含めて記述することができます。

TODO: 今回の範囲ではロジックにIC色は無いため、UnitテストはLocal canisterに配備せずそのまま実行する方法としましたが、Canisterに配置したテストの方法は未調査。

```bash
$ cargo test
```

### 補足

assertによる一致判定を行う際、`assert_eq!(expected, actual)`なのか、`assert_eq!(actual, expected)`なのかは[議論](https://users.rust-lang.org/t/assert-eq-expected-and-actual/20304)がありますが、Rust公式ドキュメントの[Macro std::assert_eq]([https://doc.rust-lang.org/std/macro.assert_eq.html])のExamplesや、[JUnitなどの流れ](https://stackoverflow.com/questions/2404978/why-are-assertequals-parameters-in-the-order-expected-actual#answer-57059952)から、前者としています。

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
