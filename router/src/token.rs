use jsonwebtoken as jwt;
use jwt::{decode, encode, Header, Validation};
use chrono::Local;

use crate::error::{Result, RouterError};
use jsonwebtoken::TokenData;

static SECRET: &'static str = "mysecret";
const EXPIRED: i64 = 60 * 60 * 24 * 30;

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    aud: String,
    sub: String,
    iss: String,
    exp: i64,
}

pub fn do_encode(phone: &str) -> Result<String> {
    let the_claims = Claims { aud: phone.to_string(), sub: "stock".to_string(), iss: "ss".to_string(), exp: Local::now().timestamp() + EXPIRED };
    let header = Header::default();
    encode(&header, &the_claims, SECRET.as_ref()).map_err(Into::into)
}

pub fn do_decode(token: &str) -> Result<TokenData<Claims>> {
    decode::<Claims>(&token, SECRET.as_ref(), &Validation::default()).map_err(Into::into)
}

mod tests {
    use crate::token::{do_encode, do_decode};

    #[test]
    fn test() {
        let phone = "18500863838";

        match do_encode(&phone) {
            Ok(mut token) => {
                println!("token is {}", token);
//                token = format!("a{}", token);

                match do_decode(token.as_str()) {
                    Ok(v) => {
                        println!("{:?}", v);
                    }
                    Err(e) => {
                        println!("{:?}", e);
                    }
                }
            }
            Err(e) => {
                println!("{:#?}", e)
            }
        }
    }
}