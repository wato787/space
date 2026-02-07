# modern-css notes

## 2026-02-07

### 目的
- 近年のCSS標準（2023-2025あたり）を中心に、従来の書き方からの差分を整理する。

### 主要アップデートのメモ
- **カスケード制御**: `@layer` / `@scope` で衝突と詳細度を整理できる。
- **コンポーネント単位のレスポンシブ**: `@container` でコンテナ起点に切り替え可能。
- **セレクタ拡張**: `:has()` による親条件、`:is()`/`:where()` による詳細度調整。
- **CSS変数の拡張**: カスタムプロパティ + `@property` で型付きにできる。
- **レイアウト**: Grid/Subgrid、論理プロパティ（`padding-inline` 等）で書字方向に強い。
- **表現**: `clamp()`/`min()`/`max()`、`color-mix()` などで計算と色指定を簡潔に。

### 移行/導入の進め方（案）
- 既存CSSを **レイヤー分割**（reset/base/components/utilities）して衝突を減らす。
- 新機能は `@supports` で段階導入し、互換性を維持する。
- `@container` による **部品単位レスポンシブ** を先行導入すると効果が出やすい。

### 遷移のベスプラ補足
- `transition: all` は避け、対象プロパティを明示する。
- `transform`/`opacity` を優先し、レイアウト系プロパティは極力避ける。
- `prefers-reduced-motion` を尊重して、動きが不要な利用者に配慮する。

### 追加で調べたいこと
- `@scope` と `@layer` の運用パターン（reset/base/components/utilities）
- `@container` の style queries と採用時期
