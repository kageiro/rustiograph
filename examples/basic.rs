// Using RustioGraph 0.0.1

use rustiograph::bounded::{ShortName, Title};
use rustiograph::content::ContentInput;
use rustiograph::methods::CreateAccount;
use rustiograph::{Rustiograph, bounded::AuthorName, methods::CreatePage};
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let mut telegraph = Rustiograph::new(None)?;

    // Create account and set token
    telegraph
        .create_account(CreateAccount {
            short_name: ShortName::new("MyBot")?,
            author_name: Some(AuthorName::new("Test Author")?),
            author_url: None,
            auth: true,
        })
        .await?;

    println!("{:?}", telegraph.token());

    // Create page
    let page = telegraph
        .create_page(CreatePage {
            title: Title::new("Test Title")?,
            content: ContentInput::Html("<p>Hello</p>".into()),
            author_name: Some(AuthorName::new("Test Author")?),
            author_url: None,
            return_content: false,
            as_user: false,
        })
        .await?;

    println!("Time: {:?}", start.elapsed());

    println!("Time: {:#?}", page);

    Ok(())
}
