# 📔 Diary

Rust CDKの学習を兼ねて簡単なDiary Dappをつくってみることにします。

ユーザごとにDiaryを一つ持つことができ、日付をキーとして、タイトルとコンテンツを文字列として持つシンプルな構造を考えます。

## リポジトリ

[https://github.com/toshio/icptest/tree/master/samples/diary](https://github.com/toshio/icptest/tree/master/samples/diary)


## Backend

### Backend I/F

ユーザに関する情報と、日記の保存/読み込みを行う基本的なI/Fを考えます。

|関数                 |区分  |概要|
|:--------------------|:-----|:-------|
|set(UserConfig)      |update|ユーザ情報の設定|
|get(Principal)       |query |ユーザ情報の取得|
|save(Date)           |update|日記の保存|
|delete(Date)         |update|日記の削除|
|load(Principal,Date) |query |日記の取得|
|list(Principal)      |query |日記一覧の取得|

日記の更新は本人のみ可能ですが、日記に公開/非公開の設定を行えるようにして、他人の日記の参照は、引数にはPrincipalを渡すことで可能にします。

### Build & Deploy

```bash
$ dfx start --background --clean
$ dfx deploy backend
```


## Frontend

### Generate I/F

Frontendから利用できるようにソースファイルを生成します。

```bash
$ dfx generate backend
Generating type declarations for canister backend:
warning: some crates are on edition 2021 which defaults to `resolver = "2"`, but virtual workspaces default to `resolver = "1"`
note: to keep the current resolver, specify `workspace.resolver = "1"` in the workspace root's manifest
note: to use the edition 2021 resolver, specify `workspace.resolver = "2"` in the workspace root's manifest
  src/declarations/backend/backend.did.d.ts
  src/declarations/backend/backend.did.js
  src/backend/backend.did
```

### Generate Frontend

```bash
$ cd src
$ $ npx create-next-app@latest
✔ What is your project named? … frontend
✔ Would you like to use TypeScript? … Yes
✔ Would you like to use ESLint? … Yes
✔ Would you like to use Tailwind CSS? … No
✔ Would you like to use `src/` directory? … Yes
✔ Would you like to use App Router? (recommended) … No
✔ Would you like to customize the default import alias? … No
︙
```
