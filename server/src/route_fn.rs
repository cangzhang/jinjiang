use axum::{extract::Path, Extension, Json};

use crate::{
    prisma::{novel, novel_statistics},
    AppJsonResult, Database,
};

pub async fn novel_statistics(
    Path(novel_id): Path<i32>,
    Extension(db): Database,
) -> AppJsonResult<Vec<novel_statistics::Data>> {
    let statistics: Vec<novel_statistics::Data> = db
        .novel_statistics()
        .find_many(vec![novel_statistics::novel_id::equals(novel_id)])
        .exec()
        .await?;

    Ok(Json::from(statistics))
}

pub async fn novel_detail(
    Path(novel_id): Path<i32>,
    Extension(db): Database,
) -> AppJsonResult<Option<novel::Data>> {
    let item: Option<novel::Data> = db
        .novel()
        .find_first(vec![novel::novel_id::equals(novel_id)])
        .exec()
        .await?;
    Ok(Json::from(item))
}
