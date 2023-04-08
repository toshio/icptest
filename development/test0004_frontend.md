# frontend

Internet ComputerのDappとして、WebのFrontendを動かすには、[Asset Canister](https://github.com/dfinity/sdk/tree/master/src/canisters/frontend/ic-frontend-canister)を使用します。

[公式マニュアル](https://internetcomputer.org/docs/current/developer-docs/frontend/)には、React Frameworkを使ったサンプルがありますが、こうしたFrameworkを使わない単純なHello, Worldの最小構成について押さえておきたいと思います。


###### dfx.json

```json
{
  "canisters": {
    "test0004_frontend": {
      "frontend": {
        "entrypoint": "dist/index.html"
      },
      "source": [
        "dist/"
        ],
      "type": "assets"
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

###### dist/index.html



```
dfx start --background --clean
```

```
dfx deploy
```



```
dfx canister create test0004_frontend
```

dfx stop



flutter create --platforms=web fluttertest

flutter build web
