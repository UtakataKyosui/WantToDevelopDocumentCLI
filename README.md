# WTD - Want-Driven Development CLI

WTD（Want-Driven Development）は、プロジェクトのアイデアや要求を管理し、テンプレートベースで開発ドキュメントを生成するRust製のCLIツールです。

## 概要

WTDは以下の機能を提供します：

- **テンプレートベースのプロジェクト生成**: カスタマイズ可能なテンプレートから"Want"ドキュメント（プロジェクトアイデア・要求）と開発ドキュメントを含むプロジェクトを作成
- **複数プロジェクトの管理**: プロジェクトの選択と状態追跡
- **構造化されたプロジェクトディレクトリ**: テンプレート化されたコンテンツで整理されたプロジェクト構造を生成

## インストール

```bash
# リポジトリをクローン
git clone <repository-url>
cd wtd

# ビルド
cargo build --release

# インストール（オプション）
cargo install --path .
```

## 初期セットアップ

初回使用時は、以下のコマンドでデフォルトテンプレートを設定してください：

```bash
wtd setup
```

これにより `~/.config/wtd/` にテンプレートと設定ファイルが作成されます。

## 使い方

### 基本コマンド

#### プロジェクトの初期化・生成

```bash
# 新しいプロジェクトを作成
wtd init "マイプロジェクト"

# 出力先ディレクトリを指定
wtd init "マイプロジェクト" --output /path/to/output
```

プロジェクトは以下の構造で生成されます：
```
<出力先>/WTD/<プロジェクト名>/
├── Want/
│   └── <プロジェクト-スラッグ>.md
└── Develop-Docs/
    └── <各種>.md
```

#### プロジェクトの選択

```bash
# プロジェクトを選択（現在のアクティブプロジェクトとして設定）
wtd select "マイプロジェクト"
```

#### 状態確認

```bash
# 現在選択されているプロジェクトを表示
wtd status
```

### テンプレート管理

#### テンプレート一覧表示

```bash
# 利用可能なテンプレート一覧を表示
wtd template-list
```

#### 新しいテンプレート作成

```bash
# カスタムテンプレートを作成
wtd template-new "マイテンプレート"
```

これにより `~/.config/wtd/templates/マイテンプレート/` にテンプレートファイルが作成されます：
- `template.yaml`: テンプレートのメタデータ
- `want.md`: Wantドキュメントのテンプレート
- `docs/readme.md`: 開発ドキュメントのテンプレート

#### テンプレート削除

```bash
# テンプレートを削除（確認プロンプト付き）
wtd template-delete "マイテンプレート"
```

## テンプレートシステム

### テンプレート構造

テンプレートは `~/.config/wtd/templates/` に保存され、以下の構成要素で構成されます：

- **template.yaml**: テンプレートの名前、説明、ファイル構造を定義するメタデータ
- **テンプレートファイル**: Teraテンプレート記法を使用した各種ファイル（`want.md`、`docs/api-design.md`など）

### 利用可能な変数

テンプレート内で以下の変数が使用できます：

- `{{title}}`: プロジェクト名
- `{{slug}}`: URL安全なプロジェクト名
- `{{date}}`: プロジェクト作成日
- `{{project}}`: プロジェクト情報

### template.yaml の例

```yaml
name: "基本テンプレート"
description: "標準的なプロジェクトテンプレート"
want:
  filename: "{{slug}}.md"
  template: "want.md"
docs:
  - filename: "README.md"
    template: "docs/readme.md"
  - filename: "api-design.md"
    template: "docs/api-design.md"
tags: ["basic", "documentation"]
```

## 設定

設定ファイルは `~/.config/wtd/config.json` に保存されます：

```json
{
  "selected_project": "現在選択中のプロジェクト",
  "default_output_dir": "プロジェクト作成のデフォルト出力先",
  "default_template": "使用するデフォルトテンプレート",
  "author": "テンプレート用の作成者名"
}
```

## 開発

### ビルドとテスト

```bash
# 開発ビルド
cargo build

# テスト実行
cargo test

# 型チェック
cargo check

# リリースビルド
cargo build --release
```

### デバッグ実行

```bash
# 各サブコマンドの実行例
cargo run -- init "テストプロジェクト"
cargo run -- select "テストプロジェクト"
cargo run -- status
cargo run -- setup
```

## トラブルシューティング

### よくある問題

1. **設定ディレクトリが見つからない**
   - 初回は `wtd setup` を実行してください

2. **テンプレートが見つからない**
   - `~/.config/wtd/templates/` にテンプレートが存在することを確認してください

3. **プロジェクト作成に失敗する**
   - 出力先ディレクトリの書き込み権限を確認してください
   - テンプレートファイルの形式が正しいことを確認してください
