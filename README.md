# Rust

## Technologies

- [actix_web](https://actix.rs/) ... web framework

## Setup

`prepare.rs`を`make tag.json`で実行して`tag.json`を生成します。これは二つのcsvファイルをmergeしたjsonファイルです。
`sort.py`を実行して`output.json`を生成できます。これはjsonデータをdateごとにsortしたものです。
`tikan.py`ではjson内の`latitude`を`lat`という文字列に変換できます。


## Run

```shell
$ make run
```
