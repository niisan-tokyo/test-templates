# test-templates
multi language test templates

# PHP
メイン言語。
ちょっと盛りすぎたかも。。。

```
composer test -- <filename>
```

で動く。
もちろん、phpunit.xml入れればオプションとか入れなくてもいい

今回の例では

```
composer test -- tests\FizzBuzzTest.php --color --testdox
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