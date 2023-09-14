namespace rs volo.example

struct Item {
    1: required i64 id,
    2: required string title,
    3: required string content,

    10: optional map<string, string> extra,
}

struct GetItemRequest {
    1: required string op,
    2: required string key,
    3: required string val,
}

struct GetItemResponse {
    1: required string op,
    2: required string key,
    3: required string val,
    4: required bool status,
}

service ItemService {
    GetItemResponse GetItem (1: GetItemRequest req),
}
