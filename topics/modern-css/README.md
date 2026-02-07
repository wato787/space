# modern-css

## 結論（TL;DR）

- レイアウト/レスポンシブは Flex/Grid + コンテナクエリ中心で考えると、コンポーネントが再利用しやすい。
- `@layer`/`@scope`/`:is()`/`:where()` でカスケードと詳細度を制御し、グローバル衝突を減らす。
- 遷移は `transform`/`opacity` を優先し、`prefers-reduced-motion` に対応する。

## 目的 / 背景

- 「よくあるCSS」の知識（float、BEM、media queries、preprocessor）から、近年の標準機能へアップデートする。

## よくあるCSS vs 最新CSS（ざっくり比較）

| 観点 | よくあるCSS | 最新CSS（モダン） |
| --- | --- | --- |
| レイアウト | float/inline-block/position | Flex/Grid/Subgrid |
| レスポンシブ | viewportの`@media`中心 | `@container`/`clamp()`/`min()`/`max()` |
| スコープ | BEMや命名規約で衝突回避 | `@layer`/`@scope`でカスケード制御 |
| 変数 | Sass/Lessの変数 | CSSカスタムプロパティ + `@property` |
| セレクタ | classと詳細度勝負 | `:is()`/`:where()`/`:has()` |
| 単位 | px/rem中心 | `dvh/svh/lvh`、logical properties |
| 機能検出 | なし/ベンダー頼み | `@supports`で段階的適用 |

## 実践メモ

- `@layer` で **優先順位を固定** し、意図しない上書きを減らす。
- `@container` で **コンポーネントのサイズ起点** にレスポンシブを作れる。
- `:where()` は詳細度0なので **ユーティリティの下地** に使いやすい。
- `clamp()` で **流体タイポグラフィ** を簡潔に表現できる。
- `@supports` で **新機能を安全に段階導入** する。

## 遷移（transition）のベストプラクティス

- `transition: all` は避け、**必要なプロパティだけ** 指定する。
- **`transform`/`opacity` を優先**（レイアウトや再計算の負担が小さい）。
- `width/height/top/left` などの **レイアウト系は基本避ける**。
- **短めの時間**（UIは150〜250ms程度）で一貫性を持たせる。
- `prefers-reduced-motion` に対応し、**動きを減らす設定**を尊重する。
- `will-change` は **必要なときだけ短時間** 付与する。

```css
.button {
  transition: transform 180ms ease, opacity 180ms ease;
}

@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    transition-duration: 0.01ms !important;
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
  }
}
```

## コード例

- `code/` に最小サンプル（HTML/CSS）を置いてある。
- 実行方法は `code/README.md` を参照。

## テスト

- `tests/` に手動確認のチェックリストを置く。

## links

- https://developer.mozilla.org/ja/docs/Web/CSS/CSS_nesting
- https://developer.mozilla.org/ja/docs/Web/CSS/CSS_containment/Container_queries
- https://developer.mozilla.org/ja/docs/Web/CSS/@layer
- https://developer.mozilla.org/ja/docs/Web/CSS/@scope
- https://developer.mozilla.org/ja/docs/Web/CSS/:has
- https://developer.mozilla.org/ja/docs/Web/CSS/transition
- https://developer.mozilla.org/ja/docs/Web/CSS/clamp
- https://developer.mozilla.org/ja/docs/Web/CSS/CSS_logical_properties_and_values
- https://developer.mozilla.org/ja/docs/Web/CSS/@supports
- https://developer.mozilla.org/ja/docs/Web/CSS/prefers-reduced-motion

## TODO

- `@scope`/container style queries の対応状況を整理する
- `@layer` の運用例（reset/base/components/utilities）を追加する
