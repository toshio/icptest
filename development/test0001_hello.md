# Hello

まずはじめに、最もシンプルなHelloを返すDappsを開発してみます。

## プロジェクト作成

以下のようにdfx newコマンドを使うことで、Rust用のサンプルアプリケーションを自動生成することができます。

```bash
$ dfx new --type=rust test0001_helloworld
$ cd test0001_helloworld/
```

## ローカル環境でのCanister起動

```bash
$ dfx start --clean --background
Running dfx start for version 0.12.1
Using the default definition for the 'local' shared network because /home/toshio/.config/dfx/networks.json does not exist.
Dashboard: http://localhost:43839/_/dashboard
```

## ローカル環境へのDeploy

```bash
$ dfx deploy
︙
Deployed canisters.
URLs:
  Frontend canister via browser
    test0001_hello_frontend: http://127.0.0.1:4943/?canisterId=ryjl3-tyaaa-aaaaa-aaaba-cai
  Backend canister via Candid interface:
    test0001_hello_backend: http://127.0.0.1:4943/?canisterId=r7inp-6aaaa-aaaaa-aaabq-cai&id=rrkah-fqaaa-aaaaa-aaaaq-cai
```

## 実行

Webブラウザーでそれぞれアクセスしてみましょう。

### Dashboard

![](<.gitbook/assets/test0001_hello_01_dashboard.png>)

### Frontend

![](<.gitbook/assets/test0001_hello_02_frontend.png>)

### Backend

![](<.gitbook/assets/test0001_hello_03_backend.png>)
