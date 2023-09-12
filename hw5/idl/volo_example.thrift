namespace rs volo.example

enum RCommand {
    Get,
    Set,
    Ping,
    Del,
    Publish,
    Subscribe,
    Unkonwn,
}

struct GetItemRequest {
    1: required RCommand cmd,
    2: optional list<string> args,
}

struct GetItemResponse {
    1: required bool ok,
    2: optional string data,
}

service ItemService {
    GetItemResponse GetItem (1: GetItemRequest req),
}

