use jun::repositories::UserRepository;

type OutputResult = Result<(), Box<dyn std::error::Error>>;

async fn test_create_user(repo: &UserRepository) -> OutputResult {
    let hasher = jun::hash::PasswordHasher::new("super secret key");
    let create_user = jun::models::CreateUser {
        username: "eden-east".to_string(),
        email: "edenofeast@email.com".to_string(),
        password: "the-very-best-password".to_string(),
    };

    let token = repo.create(create_user, &hasher).await?;
    println!("{:#?}", token);

    Ok(())
}

async fn output_users(repo: &UserRepository) -> OutputResult {
    let all = repo.all().await?;
    println!("{:#?}", all);

    Ok(())
}

#[async_std::main]
async fn main() -> OutputResult {
    let database_url = "postgres://jun:jun@localhost:5432/jun";
    let pool = sqlx::PgPool::new(database_url).await?;
    let repo = UserRepository::new(&pool);

    output_users(&repo).await?;
    test_create_user(&repo).await?;

    Ok(())
}
