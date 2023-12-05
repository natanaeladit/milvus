use milvus::{schema::{FieldSchema, CollectionSchemaBuilder}, client::Client, error::Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    const URL: &str = "http://localhost:19530";

    let client = Client::new(URL).await?;

    const DEFAULT_VEC_FIELD: &str = "feature";
    let schema =
        CollectionSchemaBuilder::new("hello_milvus", "a guide example for milvus rust SDK")
            .add_field(FieldSchema::new_primary_int64(
                "id",
                "primary key field",
                true,
            ))
            .add_field(FieldSchema::new_float_vector(
                DEFAULT_VEC_FIELD,
                "feature field",
                256,
            ))
            .build()?;
    let collection = client.create_collection(schema.clone(), None).await?;
    Ok(())
}