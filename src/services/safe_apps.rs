use crate::cache::cache_operations::RequestCached;
use crate::config::safe_apps_cache_duration;
use crate::models::backend::safe_app::SafeApp as BackendSafeApp;
use crate::models::service::safe_app::SafeApp;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;

pub async fn safe_apps(context: &Context<'_>, chain_id: &String) -> ApiResult<Vec<SafeApp>> {
    let url = config_uri!("/v1/safe-apps/?chainId={}", chain_id);

    let data = RequestCached::new(url)
        .cache_duration(safe_apps_cache_duration())
        .execute(context.client(), context.cache())
        .await?;

    Ok(serde_json::from_str::<Vec<BackendSafeApp>>(&data)?
        .into_iter()
        .map(|backend_safe_app| backend_safe_app.into())
        .collect::<Vec<SafeApp>>())
}