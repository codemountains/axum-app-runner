# Hello Axum on App Runner

An example of deploying a REST API using Rust and Axum web framework to [AWS App Runner](https://aws.amazon.com/apprunner/).

See more: [AWS App Runner で Rust / Axum 製 Web アプリケーションを動かす](https://zenn.dev/collabostyle/articles/76f1c87b743e97)

## Response

URL: `https://{your_id}.awsapprunner.com/`

### 200 OK

```json
{
    "mountains": [
        {
            "id": 1,
            "name": "エベレスト",
            "elevation": 8848
        },
        {
            "id": 2,
            "name": "K2",
            "elevation": 8611
        },
        {
            "id": 3,
            "name": "カンチェンジェンガ",
            "elevation": 8586
        },
        {
            "id": 4,
            "name": "ローツェ",
            "elevation": 8516
        },
        {
            "id": 5,
            "name": "マカルー",
            "elevation": 8463
        },
        {
            "id": 6,
            "name": "チョ・オユー",
            "elevation": 8188
        },
        {
            "id": 7,
            "name": "ダウラギリ",
            "elevation": 8167
        },
        {
            "id": 8,
            "name": "マナスル",
            "elevation": 8163
        },
        {
            "id": 9,
            "name": "ナンガ・パルバット",
            "elevation": 8126
        },
        {
            "id": 10,
            "name": "アンナプルナ",
            "elevation": 8091
        },
        {
            "id": 11,
            "name": "ガッシャーブルⅠ峰",
            "elevation": 8080
        },
        {
            "id": 12,
            "name": "ブロード・ピーク",
            "elevation": 8051
        },
        {
            "id": 13,
            "name": "ガッシャーブルムⅡ峰",
            "elevation": 8035
        },
        {
            "id": 14,
            "name": "シシャパンマ",
            "elevation": 8027
        }
    ],
    "total": 14
}
```
