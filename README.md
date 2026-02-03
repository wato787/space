# space（学習メモ / 技術キャッチアップ置き場）

気になった技術・概念を **トピック単位でフォルダにためていく** 学習用リポジトリです。  
各トピック配下に、以下を自由に置けるようにします。

- **md**: まとめ、疑問、リンク集、検証ログ
- **コード例**: 最小再現・サンプル・スニペット
- **テスト**: 期待する挙動を固定して理解を深める

## 使い方（最短）

### 新しいトピックを追加

```bash
./scripts/new-topic.sh "<topic-name>"
```

例:

```bash
./scripts/new-topic.sh "http-cache"
./scripts/new-topic.sh "react-server-components"
```

作成後は `topics/<topic-name>/README.md` から書き始めればOKです。

## ディレクトリ構成

```
topics/
  README.md                # トピック一覧（手で育てる）
  _template/               # 新規トピック用テンプレ
  <topic-name>/
    README.md              # 要点まとめ・結論・TODO
    notes.md               # 調査ログ（時系列でもOK）
    code/                  # 最小再現・サンプル
    tests/                 # 学びを固定するためのテスト
```

## 運用ルール（ゆるめ）

- **1トピック1フォルダ**: 迷ったら分割してOK
- **結論を先に書く**: `README.md` 冒頭に「何がわかったか」を短く
- **参照リンクは残す**: 公式・PR・RFC・Issue 等へのリンクを `links` セクションに
- **再現コードは小さく**: 依存は最小、実行手順を `README.md` に書く

詳細は `CONTRIBUTING.md` を参照してください。
