const LOCAL_STORAGE: &str = "./storage.json"

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync + "static>>"
