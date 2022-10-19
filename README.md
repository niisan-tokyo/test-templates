# test-templates
multi language test templates

# PHP
メイン言語。
ちょっと盛りすぎたかも。。。

```
composer install
```
で使えるようにする。

```
composer test -- <filename>
```

で動く。
もちろん、phpunit.xml入れればオプションとか入れなくてもいい

今回の例では

```
composer test -- tests/FizzBuzzTest.php --color --testdox
```

で実行。

# Ruby
いまいち書き方を覚えられない。。。

すでにユニットテストはバンドルされているらしく

```
ruby FizzBuzzTest.rb 
```

で動かす。

# TypeScript
denoの使い方わからないので、nodeでいく。

```
npm install
```
でインストールして、

```
npx jest
```
で実行。

今の実装はよくなさそう？（letはあまり使わないほうがいいって聞いている）

# Rust
テストの書き方を知るために、プロジェクトを作ってみる

```
cargo new fizz_buzz --lib
```

テストの実行はこんな感じ
```
cd fizz_buzz
cargo test
```

適当にmain作って動作させる。

```
cargo run
```

# Python
デフォルトでテスト入っているっぽい。

```
cd fizzbuzz
python -m unittest fizzbuzz_test.py 
```

こんな感じで動かしている

なお、Windowsだとなぜかremote dockerでエラーが出るので、

```
cd .devcontainer
docker-compose up -d
```

やって、リモートコンテナでアタッチするようにしている
ローカルにpythonがある場合はそっち使っていい。

# Haskell
まず、パッケージリストを最新化する
`cabal update`

ユニットテストのインストール
`cabal install --lib HUnit`

テストの実行はこんな感じ
`runghc FizzBuzzTest.hs`

# Golang

これで準備をしている

```
go mod init example.com/fizzbuzz
go mod tidy
```

テストはこんな感じ

```
go test
```