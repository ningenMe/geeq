use once_cell::sync::Lazy;
use std::env;
use std::collections::HashMap;
use serde::Deserialize;

static GITHUB_OAUTH_CLIENT_ID: Lazy<String> = Lazy::new(|| {
    env::var("GITHUB_OAUTH_CLIENT_ID").expect("env variable is not found")
});
static GITHUB_OAUTH_CLIENT_SECRET: Lazy<String> = Lazy::new(|| {
    env::var("GITHUB_OAUTH_CLIENT_SECRET").expect("env variable is not found")
});

#[derive(Deserialize, Debug)]
pub struct GithubLoginOauthAccessTokenDto {
    pub access_token: String,
}

#[derive(Deserialize, Debug)]
pub struct GithubUserDto {
    pub login: String,
    pub avatar_url: String,
}

//TODO エラーハンドリング
pub async fn post_login_oauth_access_token(code: String) -> Result<GithubLoginOauthAccessTokenDto,reqwest::Error> {
    let mut map = HashMap::new();
    map.insert("client_id", &*GITHUB_OAUTH_CLIENT_ID);
    map.insert("client_secret", &*GITHUB_OAUTH_CLIENT_SECRET);
    map.insert("code", &code);

    let client = reqwest::Client::new();
    let response = client
        //TODO エンドポイントは定数ファイルにまとめる
        .post("https://github.com/login/oauth/access_token")
        .header(
            reqwest::header::CONTENT_TYPE, 
            reqwest::header::HeaderValue::from_static("application/json") // これどこかに定義ないのか？
        )
        .header(
            reqwest::header::ACCEPT,
            reqwest::header::HeaderValue::from_static("application/json")
        )
        .json(&map)
        .send()
        .await?
        .text()
        .await?;

    //TODO intercepterとかでIOをログに落とす
    println!("{:?}", response);

    let dto = serde_json::from_str::<GithubLoginOauthAccessTokenDto>(&response).unwrap();
    return Ok(dto);
}

pub async fn get_user(access_token: String) -> Result<GithubUserDto,reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/user")
        .header(
            reqwest::header::AUTHORIZATION,
            format!("Bearer {}", access_token)
        )
        .header(
            reqwest::header::USER_AGENT,
            "Geeq Api" // 自分のユーザーエージェントを指定する
        )
        .send()
        .await?
        .text()
        .await?;

    //TODO intercepterとかでIOをログに落とす
    println!("{:?}", response);
    let dto = serde_json::from_str::<GithubUserDto>(&response).unwrap();
    return Ok(dto);
}