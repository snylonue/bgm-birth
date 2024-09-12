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
...
{
  "id": 22589,
  "name": "アゼル",
  "summary": "流星学園の2年生で、シンのクラスメイト。\r\nリ・クリエの力から解放され、改めてシンたちの友達となってからは、記録係として生徒会に参加するようにな った。\r\n同じ天使のロロットやエミリナに連れられて、少しずつ人と関わることにも慣れてきたのか、以前と比べると話しやすくなったように感じられる。\r\nラーメンとカメラに凝っていて、それらの趣味には一家言あるらしい。\r\nかつてのようにただ神の意志に従うのではなく、自分の進む道は自分の意思で決めようと努力中。\r\nまた、その道を示してくれたシンに対しては、何かしら特別な感情が見え隠れしている。\r\n\r\n「ふ……たまには、こういうのも悪くないな」",
  "locked": false,
  "stat": {
    "comments": 7,
    "collects": 6
  }
}
{
  "id": 24465,
  "name": "槻司鸢丸",
  "summary": "学生会副会长。虽然是个在富裕的家庭长大的公子哥，但口气与行动却都是粗鲁的代名词。经常与刚转校而来的草十郎商量事情。",
  "locked": false,
  "stat": {
    "comments": 7,
    "collects": 12
  }
}
$ ./bgm-birth -s # use -s to output more infomation
...
14: アゼル: 流星学園の2年生で、シンのクラスメイト。
リ・クリエの力から解放され、改めてシンたちの友達となってからは、記録係として生徒会に参加するようになった。
同じ天使のロロットやエミリナに連れられて、少しずつ人と関わることにも慣れてきたのか、以前と比べると話しやすくなったように感じられる。
ラーメンとカメラに凝っていて、それらの趣味には一家言あるらしい。
かつてのようにただ神の意志に従うのではなく、自分の進む道は自分の意思で決めようと努力中。
また、その道を示してくれたシンに対しては、何かしら特別な感情が見え隠れしている。

「ふ……たまには、こういうのも悪くないな」

15: 槻司鸢丸: 学生会副会长。虽然是个在富裕的家庭长大的公子哥，但口气与行动却都是粗鲁的代名词。经常与刚转校而来的草十郎商量事情。
```