fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use chrono::{Local, NaiveDateTime};
    use futures::stream::BoxStream;
    use futures::TryStreamExt;
    use sqlx::postgres::{PgPoolOptions, PgRow};
    use sqlx::{Acquire, Connection, Error, FromRow, PgConnection, Pool, Postgres, Row};
    use std::time::Duration;

    #[derive(Debug, FromRow)]
    struct Category {
        id: String,
        name: String,
        description: String,
    }

    #[derive(Debug, FromRow)]
    struct Brand {
        id: String,
        name: String,
        description: String,
        created_at: NaiveDateTime,
        updated_at: NaiveDateTime,
    }

    #[tokio::test]
    async fn test_manual_connection() -> Result<(), Error> {
        let url = "postgres://root:root@localhost:5432/rust_database";
        let connection = PgConnection::connect(url).await?;

        connection.close().await?;
        Ok(())
    }

    async fn get_pool() -> Result<Pool<Postgres>, Error> {
        let url = "postgres://root:root@localhost:5432/rust_database";
        PgPoolOptions::new()
            .max_connections(10)
            .min_connections(5)
            .acquire_timeout(Duration::from_secs(5))
            .idle_timeout(Duration::from_secs(60))
            .connect(url)
            .await
    }

    #[tokio::test]
    async fn test_pool_connection() -> Result<(), Error> {
        let pool = get_pool().await?;
        pool.close().await;
        Ok(())
    }

    #[tokio::test]
    async fn test_execute() -> Result<(), Error> {
        let pool = get_pool().await?;

        sqlx::query("insert into categories(id, name, description) values ('1', 'Test', 'Test 1')")
            .execute(&pool)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_execute_prepare_statement() -> Result<(), Error> {
        let pool = get_pool().await?;

        sqlx::query("insert into categories(id, name, description) values ($1, $2, $3)")
            .bind("2")
            .bind("Contoh")
            .bind("Contoh 1")
            .execute(&pool)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_optional() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result = sqlx::query("select * from categories where id = $1")
            .bind("1")
            .fetch_optional(&pool)
            .await?;

        if let Some(row) = result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!(
                "id : {}, name : {}, description : {}",
                id, name, description
            );
        } else {
            println!("Data is not found");
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_one() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result = sqlx::query("select * from categories where id = $1")
            .bind("1")
            .fetch_one(&pool)
            .await?;

        let id: String = result.get("id");
        let name: String = result.get("name");
        let description: String = result.get("description");
        println!(
            "id : {}, name : {}, description : {}",
            id, name, description
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch_all() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result = sqlx::query("select * from categories")
            .fetch_all(&pool)
            .await?;

        for row in result {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!(
                "id : {}, name : {}, description : {}",
                id, name, description
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_fetch() -> Result<(), Error> {
        let pool = get_pool().await?;

        let mut result = sqlx::query("select * from categories").fetch(&pool);

        while let Some(row) = result.try_next().await? {
            let id: String = row.get("id");
            let name: String = row.get("name");
            let description: String = row.get("description");
            println!(
                "id : {}, name : {}, description : {}",
                id, name, description
            );
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_result_mapping() -> Result<(), Error> {
        let pool = get_pool().await?;

        let mut result = sqlx::query("select * from categories")
            .map(|row: PgRow| Category {
                id: row.get("id"),
                name: row.get("name"),
                description: row.get("description"),
            })
            .fetch(&pool);

        while let Some(row) = result.try_next().await? {
            println!("{:?}", row);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_result_mapping_auto() -> Result<(), Error> {
        let pool = get_pool().await?;

        let mut result: BoxStream<Result<Category, Error>> =
            sqlx::query_as("select * from categories").fetch(&pool);

        while let Some(row) = result.try_next().await? {
            println!("{:?}", row);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_insert_brand() -> Result<(), Error> {
        let pool = get_pool().await?;

        sqlx::query("insert into brands values ($1, $2, $3, $4, $4)")
            .bind("1")
            .bind("Xiomay")
            .bind("Test insert brand")
            .bind(Local::now().naive_local())
            .execute(&pool)
            .await?;

        Ok(())
    }

    #[tokio::test]
    async fn test_result_mapping_brand_auto() -> Result<(), Error> {
        let pool = get_pool().await?;

        let mut result: BoxStream<Result<Brand, Error>> =
            sqlx::query_as("select * from brands").fetch(&pool);

        while let Some(row) = result.try_next().await? {
            println!("{:?}", row);
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_insert_brand_transaction() -> Result<(), Error> {
        let pool = get_pool().await?;
        let mut transaction = pool.begin().await?;

        sqlx::query("insert into brands values ($1, $2, $3, $4, $4)")
            .bind("2")
            .bind("Xiomay 2")
            .bind("Test insert brand")
            .bind(Local::now().naive_local())
            .execute(&mut *transaction)
            .await?;

        sqlx::query("insert into brands values ($1, $2, $3, $4, $4)")
            .bind("3")
            .bind("Xiomay 3")
            .bind("Test insert brand")
            .bind(Local::now().naive_local())
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;


        Ok(())
    }

    #[tokio::test]
    async fn test_auto_increment() -> Result<(), Error> {
        let pool = get_pool().await?;

        let result = sqlx::query("insert into sellers(name) values($1) returning id")
            .bind("Contoh Selller")
            .fetch_one(&pool).await?;

        let id: i32 = result.get("id");

        println!("Inserted id is {}", id);

        Ok(())
    }

    #[tokio::test]
    async fn test_auto_increment_transaction() -> Result<(), Error> {
        let pool = get_pool().await?;
        let mut transaction = pool.begin().await?;

        sqlx::query("insert into sellers(name) values($1)")
            .bind("Contoh Selller")
            .execute(&mut *transaction).await?;

        let result = sqlx::query("select lastval() as id")
            .fetch_one(&mut *transaction).await?;

        let id: i32 = result.get_unchecked("id");

        println!("Inserted id is {}", id);

        Ok(())
    }
}
