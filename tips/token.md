# 🪙 Token

ICP上でTokenの取り扱う規格として、EthereumにおけるERC-20のように『ICRC-1』があります。

## Specificaiton

|Proposal|Description|did|
:--------|:-------------|:-----------------------|
|ICRC-1  |[Token Standard](https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-1/README.md)|[ICRC-1.did](https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-1/ICRC-1.did)|
|ICRC-2  |[Approve and Transfer From](https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-2/README.md)|[ICRC-2.did](https://github.com/dfinity/ICRC-1/blob/main/standards/ICRC-2/ICRC-2.did)|

## トークン発行

ICP上でトークンを発行する方法について公式ドキュメントがあります。

[https://internetcomputer.org/docs/current/developer-docs/integrations/icrc-1/deploy-new-token](https://internetcomputer.org/docs/current/developer-docs/integrations/icrc-1/deploy-new-token)

試しにローカルPC上でつくってみることにします。

### (1) ダウロードスクリプト

```bash
$ curl -o download_latest_icrc1_ledger.sh "https://raw.githubusercontent.com/dfinity/ic/master/rs/rosetta-api/scripts/download_latest_icrc1_ledger.sh"
```

※2023年9月19日時点で、Githubにコミットされているファイル名が「_ledger.sh」ではなく、「_leger.sh」となっているので注意

### (2) ダウンロードスクリプトの実行

```bash
$ bash download_latest_icrc1_ledger.sh
Found artifacts for commit 21fa6190f619ae1179e3511092cd5644f61ceb3e. Downloading icrc1_ledger.did and icrc1_ledger.wasm.gz
```

※jqコマンドがインストールされている必要があります。

### (3) dfx.json

```json
{
  "canisters": {
    "icrc1-ledger": {
      "type": "custom",
      "wasm": "icrc1-ledger.wasm",
      "candid": "icrc1-ledger.did"
    }
  }
}
```

### (4) deploy_token.sh修正

解説ページに載っているスクリプトをベースに、配備用スクリプトを用意します。

###### [deploy_token.sh](deploy_token.sh)

```bash
#!/bin/bash

# Change the variable to "ic" to deploy the ledger on the mainnet.
export NETWORK=local

# Change the variable to the principal that can mint and burn tokens.
export MINTER_PRINCIPAL=$(dfx identity get-principal)

# Change the variable to the principal that controls archive canisters.
export ARCHIVE_CONTROLLER=$(dfx identity get-principal)

export TOKEN_NAME="TM Token"
export TOKEN_SYMBOL=XTM

dfx deploy --network ${NETWORK} icrc1-ledger --argument "(variant { Init = 
      record {
        token_name = \"${TOKEN_NAME}\";
        token_symbol = \"${TOKEN_SYMBOL}\";
        minting_account = record { owner = principal \"${MINTER_PRINCIPAL}\";};
        initial_balances = vec {};
        metadata = vec {};
        transfer_fee = 10;
        archive_options = record {
          trigger_threshold = 2000;
          num_blocks_to_archive = 1000;
          controller_id = principal \"${ARCHIVE_CONTROLLER}\";
        }
}})"
```

### (5) deploy

```bash
$ dfx start --background --clean
$ bash deploy_token.sh
```

### (6) deploy

$ dfx start --background --clean

## その他情報

### The Internet Computer Token Standards: A Comparison

Fungible Tokenの規格として、ICRC-1以外にも、IS20、DIP20、EXTといった規格があるようです。

https://www.blog.bitfinity.network/the-internet-computer-token-standards-a-comparison/
