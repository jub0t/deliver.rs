# Upload Content

```sh
    GET /upload-content/{document_id}
```

### Parameters

| Name | Type | Required |
| --- | --- | --- |
| `document_id` | `String` | **TRUE** |

### Headers

| Name | Type | Required |
| --- | --- | --- |
| `token` | `String` | **TRUE** |

### Body

| Name | Type | Required |
| --- | --- | --- |
| `name` | `String` | **TRUE** |
| `content` | `Buffer` | **TRUE** |