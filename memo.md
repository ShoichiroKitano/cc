# cargo test
```
# 全部のテスト実行
$ cargo test

# 特定のテストファイルを実行(integrationのみ)
$ cargo test --test ファイル名

# 特定のテスト関数を実行
$ cargo test テスト関数名
$ cargo test --tests --examples テスト関数名
ex unit) cargo test --tests --examples root::tests::function
ex integration) cargo test --tests --examples function
```

unit testが失敗する状態だとintegration testが実行されないっぽい。

テスト
