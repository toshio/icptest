# Homework diary 📔

2023年5月に開催された[Motoko Bootcamp Day 2](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-2/project/README.MD)のプロジェクトをRust言語で実装します。

## 1. Rustプロジェクト作成

Rustのプロジェクト「day2」を作成します。`cargo new`コマンドを`--lib`オプションを付与して実行します。

```
$ cargo new day2 --lib
$ cd day2
```

生成されたファイルは以下の通りです。

```
day2
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

candid、serdeは、CandidType, Deserialize指定に必要?

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
    "day2": {
      "candid": "./day2.did",
      "package": "day2",
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

dfx.jsonの [canisters] > [day2] > [candid]項目に指定したファイルに、Canisterに配置するDappが提供する関数のI/Fを定義します。

[Motoko Bootcamp Day2 📺 Interface](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-2/project/README.MD#-interfacee)に相当するcandidを用意します。

###### [day2.did](day2.did)

```
type Time = int;
type Homework = record {
    "title": text;
    "description": text;
    "dueDate": Time;
    "completed": bool;
};

type ResultHomework = variant { Ok: Homework; Err: text };
type ResultOnly = variant { Ok; Err: text };

service: {
    addHomework: (Homework) -> (nat);
    getHomework: (nat) -> (ResultHomework) query;
    updateHomework: (nat, Homework) -> (ResultOnly);
    markAsCompleted: (nat) -> (ResultOnly);
    deleteHomework: (nat) -> (ResultOnly);
    getAllHomework: () -> (vec Homework) query;
    getPendingHomework: () -> (vec Homework) query;
    searchHomework: (text) -> (vec Homework) query;
}
```

#### Time型

Motokoの[Time](https://internetcomputer.org/docs/current/motoko/main/base/Time)はint (System time is represent as nanoseconds since 1970-01-01.)のようで、Rust言語ではint128でOKと思われます。

#### TODO: Result型

Local caniterに配備してCandid UIで結果を見ると、motokoの[Result.Result](https://internetcomputer.org/docs/current/motoko/main/base/Result)は`variant {ok:xxx, err:text}`（先頭小文字）となるけど、Rustの[`Result<T, E>`](https://doc.rust-lang.org/std/result/enum.Result.html)は`variant {Ok:xxx, Err:text}` (先頭大文字)の違いがあるようです。

candid定義を Motoko に合わせて、`variant { ok:Homework; err: text }`とするとRust側でResultを返す際にマッピングエラーとなってしまい、[Motoko Bootcamp Day 2](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-2/project/README.MD)に合わせることができず・・・。要調査

## 5. lib.rsの編集

`cargo new`コマンドで生成されたlib.rsの中身をクリアして、day2用のプログラムを作成します。

[Motoko Bootcamp Day 2](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-2/project/README.MD)と同じように、以下の関数を実装します。

* addHomework()
* getHomework()
* updateHomework()
* markAsCompleted()
* deleteHomework()
* getAllHomework()
* getPendingHomework()
* searchHomework()

###### [lib.rs](src/lib.rs)

Rust言語仕様の理解が十分でないため、作成したソースコードは所有権まわりをはじめ最適化されていない可能性がありますのでご注意ください。もしも、おかしな実装等が見つかりましたらが、ご指摘いただけますとさいわいです。

## 6. Unitテスト

Rustではソース内にUnitテストコードを含めて記述することができます。

TODO: 今回の範囲ではロジックにIC色は無いため、UnitテストはLocal canisterに配備せずそのまま実行する方法としましたが、Canisterに配置したテストの方法は未調査。

```bash
$ cargo test
```

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
