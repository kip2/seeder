[English](README.md)|[日本語](README-ja.md)

<h1 align="center"> seeder </h1>

コンソールで実行できる、シンプルなシーダーアプリケーションです。

シーダーアプリとは、テーブルにデータを登録するためのアプリケーションのことです。

このアプリケーションでは、別ファイルに定義したデータを、テーブルに一気に登録する機能を持っています。

また、指定した数のランダムなデータを登録することも可能です。
テスト環境の構築等に使用することができます。

## 前提

- 本アプリケーションはPostgreSQLでの使用が前提となっています。
- DB接続情報は、.envに設定して行います。
- バイナリファイルの形では配布していませんので、各自の環境でビルドを行ってください。

# 環境設定

## ビルドの方法

Rustのインストールされた環境で、以下のコマンドを実行します。

```shell
cargo build --release
```

ビルドされた実行ファイルをディレクトリに配置してください。

ビルドされたファイルは以下のパスに生成されます。

```shell
./target/release/migrate
```

## DBのアクセス設定

事前のDBアクセス情報の設定が必要になります。

設定は以下の手順で行ってください。

1. .envファイルをビルドしたmigrateと同じディレクトリに配置する。
2. .envファイルに以下の形式で、DBアクセス設定を記入しください。

```env
DATABASE_URL=postgres://username:password@hostname:port/db_name
```

# 事前準備

## 準備の前提

DBへテーブルが登録されているのが前提となります。

テーブル登録については、[migrate](https://github.com/kip2/migrate)を使用すると良いでしょう。

登録されたテーブル情報を、.envのDBのアクセス設定に含めてください。

.envの形式を再掲します。
```env
DATABASE_URL=postgres://username:password@hostname:port/db_name
```

---

# 実行方法

## 利用可能なコマンドについてまとめ

以下のコマンドが使用可能です。

詳細な利用方法の解説は別の項で解説します。

```sh
# ファイルを指定して、テーブルへのデータ登録を実行
# file-path: 登録したいデータが設定されたファイルのパス
./seeder -f <file-path1>
# 複数ファイルでの登録も対応しています
./seeder -f <file-path1> <file-path2>

# ランダムなデータを登録する
# file-path: 登録対象のテーブルのカラム情報が設定されたファイルのパス
# n: 登録したいランダムデータの個数
./seeder -r <file-path> <n>

# -f -r どちらのコマンドでも利用されるJSONファイルの、テンプレートを生成するオプション
# file-path: テンプレートを生成する先のファイルパス
./seeder -c <file-path>

```

---

## テーブルへのデータの登録方法(-fオプション)

テーブルへのデータの登録は、あらかじめJSONファイルに定義したデータを用いて行います。

そのため、以下2つの手順が必要になります。

- ファイルの準備
- コマンドの実行

### ファイルの準備

登録したいデータをJSONファイルに定義してください。

なお、JSONファイルのテンプレートを生成するコマンドも用意しています。

```sh
# 指定したパスに、テンプレートJSONファイルを生成する
# file-path: テンプレートを生成する先のファイルパス
./seeder -c <file-path>
```

作成したテンプレートファイルに、各種の情報を記載します。

以下の点に注意して記載してください

- 使用できる`data_type`は以下の4つ。
  - `int`
  - `float`
  - `string`
  - `date` 

JSONファイルの定義例は以下のようになります。

```json
{
    "table_name": "computer_parts",
    "table_columns": [
        {
            "data_type": "string",
            "column_name": "name",
        },
        {
            "data_type": "int",
            "column_name": "lifespan"
        }
    ]
    ,
    "table_rows": [
        [
            "Ryzen 9 5900X",
            5
        ],
    ]
}
```

### コマンドの実行

登録したいデータの定義ファイルが用意できたら、あとはコマンドを実行するだけです。
なお、複数ファイルの実行も可能です。
```sh
# file-path: 登録したいデータが設定されたファイルのパス
./seeder -f <file-path>

# 複数ファイルでの登録も対応しています
./seeder -f <file-path1> <file-path2>
```

---

## ランダムデータの登録(-rオプション)

テスト用などに、テーブルへランダムデータを複数件、登録することが可能です。

そのため、以下2つの手順が必要になります。

- ファイルの準備
- コマンドの実行

### ファイルの準備

登録したいテーブルのカラムの情報を定義する必要があります。

登録したいテーブルのカラム情報の定義をJSONファイルに記載してください。

なお、JSONファイルのテンプレートを生成するコマンドも用意しています。

```sh
# 指定したパスに、テンプレートJSONファイルを生成する
# file-path: テンプレートを生成する先のファイルパス
./seeder -c <file-path>
```

作成したテンプレートファイルに、カラムの情報を記載します。

以下の点に注意して記載してください

- 使用できる`data_type`は以下の4つ。
  - `int`
  - `float`
  - `string`
  - `date` 
- "table_rows"の項目は空になっていますが、実行時に必要のため、そのまま残して下さい。

JSONファイルの定義例は以下のようになります。

```json
{
    "table_name": "computer_parts",
    "table_columns": [
        {
            "data_type": "string",
            "column_name": "name",
        },
        {
            "data_type": "int",
            "column_name": "lifespan"
        }
    ]
    ,
    "table_rows": [
    ]
}
```

### コマンドの実行

JSONファイルが用意できたら、コマンドを実行してください。

コマンドには、

- カラムデータを定義したファイルのパス
- 生成したいランダムデータの個数

を含めてください。


```sh
# file-path: 登録対象のテーブルのカラム情報が設定されたファイルのパス
# n: 登録したいランダムデータの個数
./seeder -r <file-path> <n>
```

---

## テンプレートファイルの作成(-cオプション)

seeder実行にあたって必要なファイルの、テンプレートファイルを生成するコマンドです。

-f -r オプションのどちらでも利用するJSONファイルを生成します。

以下のコマンドを実行後、-f -rで必要な情報を記載してください。

```sh
# 指定したパスに、テンプレートJSONファイルを生成する
# file-path: テンプレートを生成する先のファイルパス
./seeder -c <file-path>
```

---

# help

コマンドについて困ったときは、ヘルプを参照してください。

以下のコマンドでヘルプが参照できます。

```sh
./seeder -h

# もしくは
./seeder --help
```

