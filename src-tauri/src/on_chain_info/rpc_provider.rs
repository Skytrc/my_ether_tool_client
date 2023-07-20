struct provider {
    https: String,
    websocket: String,
    api_key: String,
}

enum RPC_Provider {
    ALCHEMY(provider),
}

