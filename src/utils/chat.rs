use crate::{config::get_config, redis::get_redis};
use anyhow::Result;
pub async fn is_auto_search_on(chat_id: i64) -> Result<bool> {
    let config = get_config();
    let redis = match get_redis().await {
        Some(redis) => redis,
        None => return Ok(true),
    };
    let limit = config.general.empty_search_limit.unwrap_or(0);

    if (limit == 0 && redis.get_auto_search_enabled(chat_id).await?)
        || (limit > 0
            && redis.get_no_result_count(chat_id).await? < limit
            && redis.get_auto_search_enabled(chat_id).await?)
    {
        return Ok(true);
    }
    Ok(false)
}
