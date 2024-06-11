use crate::{
  api::api_wrapper,
  errors::AppError,
  utils::{SearchQuery, SearchResponse},
};

pub async fn search(form: &SearchQuery) -> Result<SearchResponse, AppError> {
  api_wrapper::<SearchResponse, SearchQuery>(form).await
}
