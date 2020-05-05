use rocket_contrib::databases::redis;

// 此处定义之后可将 RedisDbConn 作为 Request Guard 使用以获取 Redis 数据库连接
#[database("redis")]
pub struct RedisDbConn(redis::Connection);
