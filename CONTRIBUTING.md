Thank you for looking at this file. This is a contributing guide.

This repository has one rule and three steps before development.

## Rule

This repository is Adpot GitHub-Flow. So we should create PullRequests before the merge.

## Steps

1. Please fork this repository.
2. We should use a virtual environment with venv. So we should make a virtual environment with venv.
3. We should Install taskipy and maturin And run the command `task dev`. This command needs to run to apply changes every time.
4. Have a good development!

## pre-commit

We recommend that you use `pre-commit` from this repository, as it allows you to format your code and perform static analysis.

### How to use

Just run the command `pre-commit install`.
By this. Code is automatically formatted and statically analyzed at commit time.
If you wish to do this manually, you can also execute the following command.

``` bash
pre-commit run

# If you want to run it on all files
pre-commit run --all-files
```

---

## Japanese - 日本語

このファイルをご覧いただきありがとうございます。これは開発ガイドです。

このリポジトリには一つのルールと三つの開発前に行うステップがあります。

## ルール

このリポジトリはGitHub-Flowを採用しています。そのためマージする前にプルリクエストを作成する必要があります

## ステップ

1. このリポジトリをフォークしてください。
2. venvを使った仮想環境が必要です。venvを使って仮想環境を作成してください。
3. taskipyとmaturinをインストールし、`task dev`というコマンドを実行する必要があります。このコマンドは、毎回変更を適用するために実行する必要があります。
4. 良い開発を!

## pre-commit

このリポジトリでは`pre-commit`を利用でき、コードのフォーマットや、静的解析を行うことができるため、利用することを推奨します。

### 利用法

`pre-commit install`とコマンドを実行するだけです。
これによって。コミット時に自動的にコードのフォーマット、静的解析が行われます。
また、これを手動で行いたいときは以下のコマンドを実行します。

``` bash
pre-commit run

# すべてのファイルに対して実行したい場合
pre-commit run --all-files
```
