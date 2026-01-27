# HyprPresto

current version: 0.1.0

EN -> [here](https://github.com/Uliboooo/hypr-presto/blob/main/README.md)

Linux向けのミニマルでキーボード中心のアプリケーションランチャーで、速度と効率のために設計されています。HyprPrestoを使用すると、単一のキーストロークでお気に入りのアプリケーションを起動できます。

## 特徴

- **インスタント起動:** 単一のキープレスでアプリケーションをトリガーします。
- **モダンなUI:** GTK4で構築された、クリーンでダークテーマのインターフェース。
- **Waylandネイティブ:** `gtk4-layer-shell`で構築されており、HyprlandやSwayなどのコンポジターに最適です。
- **軽量:** 高速な起動と最小限のリソース使用量。
- **シンプルな設定:** 読みやすいTOML設定。

## 使用方法

![demo](https://github.com/Uliboooo/hypr-presto/blob/main/resource_for_readme/demo.mp4)

1.  `config.toml`を準備します。
2.  アプリケーションを実行します。
    ```bash
    hyprpresto
    ```
3.  ランチャーウィンドウが表示されます。
4.  起動したいアプリに対応するキーを**押します**（例: `f`を押す）。
5.  アプリが起動し、HyprPrestoは自動的に閉じます。
6.  起動せずに閉じるには、`Esc`を押します。

## 依存関係

HyprPrestoを実行するには、システムに以下がインストールされている必要があります。

- **GTK4**
- **gtk4-layer-shell**

ソースからビルドするには、以下も必要です。
- Rust (cargo)
- `blueprint-compiler`

## インストール方法

現在、HyprPrestoはGitHubでバイナリリリースとして利用可能です。

1.  [Releases](https://github.com/uliboooo/hypr-presto/releases)ページにアクセスします。
2.  最新の`hypr-presto`バイナリをダウンロードします。
3.  実行可能にします。
    ```bash
    chmod +x hypr-presto
    ```
4.  (オプション) `$PATH`内のディレクトリに移動します。例: `/usr/local/bin`:
    ```bash
    sudo mv hypr-presto /usr/local/bin/
    ```

## 設定

HyprPrestoは設定ファイルの場所としてXDG規格に従います。

**パス:** `~/.config/hypr-presto/config.toml`

### 構造

設定ファイルはTOML形式を使用します。ショートカットは`[apps]`セクションに定義します。キーは押したい単一の文字で、値はアプリケーションの**Desktop Entry ID**（通常は拡張子なしの`.desktop`ファイルの名前）です。

### `config.toml`の例

```toml
[apps]
f = "firefox"                 # 'f'を押すとFirefoxを起動
g = "com.mitchellh.ghostty"   # 'g'を押すとGhosttyを起動
c = "code"                    # 'c'を押すとVS Codeを起動
```

> **注意:** 正しいApp IDを見つけるには、`/usr/share/applications/`内のファイル名を確認します（例: `spotify.desktop`の場合は`spotify`を使用）。


### Hyprlandとの統合

`hyprland.conf`でHyprPrestoをキーにバインドできます。

```conf
bind = $mainMod, P, exec, hypr-presto(bin path)
```
