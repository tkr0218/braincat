linuxコマンドラインツール作成の練習につくってます
brainfuckを猫の鳴き声に置き換えただけです
| brainf*ck | braincat |
| :---: | :--- |
| > | みゃお |
| < | にゃん |
| + | にゃー |
| - | にゃにゃにゃ |
| . | ごろごろ |
| , | うにゃ |
| [ | （^・ω・^=） |
| ] | （=^・ω・^） |

改行は無視されます
例えば
```txt
みゃおにゃーにゃーにゃーにゃーにゃーにゃーにゃーにゃーにゃー（^・ω・^=）
にゃんにゃーにゃーにゃーにゃーにゃーにゃーにゃーにゃーみゃおにゃにゃにゃ
（=^・ω・^）にゃんごろごろみゃおにゃーにゃーにゃーにゃーにゃーにゃーにゃー
（^・ω・^=）にゃんにゃーにゃーにゃーにゃーみゃおにゃにゃにゃ（=^・ω・^）
にゃんにゃーごろごろにゃーにゃーにゃーにゃーにゃーにゃーにゃー
ごろごろごろごろにゃーにゃーにゃーごろごろ（^・ω・^=）
にゃにゃにゃ（=^・ω・^）みゃおにゃーにゃーにゃーにゃー
にゃーにゃーにゃーにゃー（^・ω・^=）にゃんにゃーにゃーにゃーにゃー
みゃおにゃにゃにゃ（=^・ω・^）にゃんごろごろみゃおにゃーにゃーにゃー
にゃーにゃーにゃーにゃーにゃーにゃーにゃーにゃー（^・ω・^=）にゃんにゃー
にゃーにゃーにゃーにゃーにゃーにゃーにゃーみゃおにゃにゃにゃ（=^・ω・^）
にゃんにゃにゃにゃごろごろにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃ
にゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃごろごろにゃーにゃーにゃー
ごろごろにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃ
にゃごろごろにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃにゃ
にゃにゃにゃにゃにゃにゃにゃごろごろ（^・ω・^=）にゃにゃにゃ（=^・ω・^）みゃお
にゃーにゃーにゃーにゃーにゃーにゃーにゃーにゃー（^・ω・^=）にゃんにゃー
にゃーにゃーにゃーみゃおにゃにゃにゃ（=^・ω・^）にゃんにゃーごろごろ（^・ω・^=）
にゃにゃにゃ（=^・ω・^）にゃーにゃーにゃーにゃーにゃーにゃーにゃーにゃーにゃーにゃーごろごろ
```/
といった内容が書かれたexample.txtがあったとき
```txt
   >braincat example.txt
```/
と入力すれば
```txt
   >Hello world!
```/
と表示されるはずです
