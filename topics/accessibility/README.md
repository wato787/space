# accessibility

## 結論（TL;DR）

- セマンティックHTMLを優先し、必要最小限だけARIAで補完する
- まずはキーボード操作とフォーカス可視化を満たす
- 状態変化・エラーは利用者に通知される形で表現する
- ランドマーク/スキップリンクで主要コンテンツへ素早く到達させる

## 目的 / 背景

- 画面が見えない/操作が難しい状況でも使えるUIを設計したい
- チームで最低限のチェック観点を共有したい

## メモ

- 詳細な調査ログは `notes.md` に残す
- 「役割（role） / 名前（name） / 値（value）」を意識する
- フォームは `label` / `fieldset` / `legend` で意味づけ
- エラーは `aria-invalid` とメッセージを関連付ける
- 動的更新は `aria-live` で通知する
- スキップリンクは最小コストで効果が高い

## コード例

- `code/index.html`：セマンティックHTML + 最小ARIAの例
  - スキップリンク、ランドマーク、見出し構造
  - 状態付きボタン（`aria-pressed` / `aria-expanded`）
  - フォームラベル + エラー表示（`aria-describedby` / `aria-invalid`）
  - `aria-live` によるステータス通知
  - 実行: ブラウザで直接開く
  - ローカルサーバ（任意）: `python3 -m http.server`
    - `http://localhost:8000/topics/accessibility/code/index.html`

## テスト

- `tests/checklist.md`：手動チェックリスト
- 実行（目安）:
  - キーボードのみで操作できるか
  - フォーカスが見えるか
  - エラー/状態変更が読み上げで伝わるか
  - スキップリンクで `main` に移動できるか

## links

- WCAG 2.2: https://www.w3.org/TR/WCAG22/
- WAI-ARIA Authoring Practices: https://www.w3.org/WAI/ARIA/apg/
- MDN Accessibility: https://developer.mozilla.org/en-US/docs/Web/Accessibility
- WAI: https://www.w3.org/WAI/

## TODO

- コントラスト/色覚多様性のチェック手順を追加
- 既存UIの監査メモを追加

