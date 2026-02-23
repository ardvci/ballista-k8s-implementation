use ballista::datafusion::execution::SessionStateBuilder;
use ballista::datafusion::prelude::{SessionConfig, SessionContext};
use std::time::Instant;
use datafusion::functions_aggregate::average::avg;
use datafusion::functions_aggregate::count::count;
use datafusion::prelude::{col, lit, ParquetReadOptions};
use anyhow::Result;
use rusty_cloud::DATA_PATH;


#[tokio::main]
async fn main() -> Result<()>{
    use ballista::extension::{SessionConfigExt, SessionContextExt};
    let url = "df1://ballista-cluster-scheduler:50050";

    let session_config = SessionConfig::new_with_ballista()
        .with_information_schema(true)
        .with_ballista_job_name("Super Cool Ballista App");

    let state = SessionStateBuilder::new()
        .with_default_features()
        .with_config(session_config)
        .build();

    let ctx: SessionContext = SessionContext::remote_with_state(&url,state).await?;

    ctx.register_parquet("logs", DATA_PATH, ParquetReadOptions::default()).await?;

    let start = Instant::now();

    let df = ctx.table("logs").await?
        .filter(col("status_code").eq(lit(200)))?
        .aggregate(vec![col("user_id")], vec![count(col("response_time_ms")), avg(col("response_time_ms"))])?
        .filter(col("count(logs.response_time_ms)").gt(lit(10)))?
        .sort(vec![col("avg(logs.response_time_ms)").sort(false, true)])?
        .limit(0, Some(5))?;

    df.clone().explain(false, false)?.show().await?;

    df.show().await?;

    println!("Query Time: {:?}", start.elapsed());

    Ok(())

}

