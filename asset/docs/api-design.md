# API設計（草案）

## POST /drafts

| 項目 | 内容 |
|------|------|
| メソッド | POST |
| 認証 | 必須（Bearer） |
| Body | `{"title": "string", "content": "string", "tags": ["string"]}` |
| 戻り値 | `201 Created` + `{"id": "uuid"}` |

---

## GET /drafts

| 項目 | 内容 |
|------|------|
| メソッド | GET |
| 認証 | 必須 |
| クエリ | `?tag=optional` |
| 戻り値 | 一覧データ（配列） |

---

## その他

- 今後、公開/非公開切り替え機能も追加したい
- ユーザー単位で分類するか、匿名アカウントにするか要検討
