use crate::domain::entity::customer::*;
use anyhow::Result;
use chrono::prelude::*;
use chrono::Duration;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::info;
use uuid::Uuid;

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct CustomerJsonWebToken {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Clone)]
pub struct Identity {
    pub token: CustomerJsonWebToken,
}

impl Identity {
    pub fn new(customer: &Customer, issueat: &DateTime<Utc>, duration: Duration) -> Identity {
        let iat = *issueat;
        let exp = iat + duration;

        let token = CustomerJsonWebToken {
            sub: customer.get_id(),
            exp: exp.timestamp(),
            iat: iat.timestamp(),
        };

        Identity { token }
    }

    pub fn from_string(value: &str) -> Result<Identity> {
        let result = decode::<CustomerJsonWebToken>(
            value,
            // TODO: move to configuration
            &DecodingKey::from_secret("secret".as_ref()),
            &Validation::default(),
        );
        let jwt = result?;

        Ok(Identity { token: jwt.claims })
    }

    pub fn to_string(&self) -> Result<String> {
        let session_str = encode(
            &Header::default(),
            &self.token,
            // TODO: move to configuration
            &EncodingKey::from_secret("secret".as_ref()),
        )?;

        Ok(session_str)
    }

    pub fn get_id(&self) -> Uuid {
        // self.token.sub.get_id()
        self.token.sub
    }

    pub fn get_expireat(&self) -> DateTime<Utc> {
        let ts = self.token.exp;
        Utc.timestamp_opt(ts, 0).unwrap()
    }

    // pub fn get_username(&self) -> String {
    //     self.token.sub.get_username()
    // }
}
