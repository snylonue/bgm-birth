# bgm-birth

## Usage

```shell
$ ./bgm-birth -h
Usage: bgm-birth.exe [OPTIONS]

Options:
  -j, --json                         show json response
      --locked                       show locked subjects
      --min-comments <MIN_COMMENTS>  filter subjects with too few comments
  -s, --summary
  -h, --help                         Print help
  -V, --version                      Print version
```

By default only names are printed:

```shell
$ ./bgm-birth
上代由香里
タカヤ・ノリコ
岩崎みなみ
白井夕璃
式守伊吹
相田ケンスケ
セイラ・マス
高村恭司
竜胆棗
雨宮大吾
今宮紀子
南ことり
葉月深景
篠崎あゆみ
アゼル
槻司鸢丸
```

```shell
$ ./bgm-birth -j # use -j to output json
$ ./bgm-birth -s # use -s to output more infomation
```