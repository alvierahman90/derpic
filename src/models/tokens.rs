use crate::schema::tokens::{self, dsl};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use poem_openapi::Object;

pub fn token_encode(slug: Vec<u8>) -> String {
    URL_SAFE_NO_PAD.encode(slug)
}

pub fn token_decode(slug: String) -> Result<Vec<u8>, base64::DecodeError> {
    URL_SAFE_NO_PAD.decode(slug)
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Token {
    id: i32,
    token: Vec<u8>,
    name: String,
    revoked: bool,
}

#[derive(Object)]
pub struct TokenEncodedSlug {
    id: i32,
    token: String,
    name: String,
    revoked: bool,
}

impl From<Token> for TokenEncodedSlug {
    fn from(other: Token) -> Self {
        Self {
            id: other.id,
            token: token_encode(other.token),
            name: other.name,
            revoked: other.revoked,
        }
    }
}

impl Token {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn token(&self) -> Vec<u8> {
        self.token.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn revoked(&self) -> bool {
        self.revoked
    }

    pub fn new(conn: &mut PgConnection, new_token: NewToken) -> Result<Self, DieselError> {
        diesel::insert_into(tokens::table)
            .values(&new_token)
            .returning(Token::as_returning())
            .get_result(conn)
    }

    pub fn get(conn: &mut PgConnection, filters: TokenFilter) -> Result<Vec<Token>, DieselError> {
        let mut query = tokens::table.into_boxed();

        if let Some(val) = filters.id {
            query = query.filter(tokens::id.eq(val));
        }

        if let Some(val) = filters.token {
            query = query.filter(tokens::token.eq(val));
        }

        if let Some(val) = filters.name {
            query = query.filter(tokens::name.eq(val));
        }

        if let Some(val) = filters.revoked {
            query = query.filter(tokens::revoked.eq(val));
        }

        query.select(Self::as_select()).load(conn)
    }

    /// Retrieves a token by its token, **filtering out revoked tokens*.
    pub fn get_by_token(
        conn: &mut PgConnection,
        token: Vec<u8>,
    ) -> Result<Option<Self>, DieselError> {
        Ok(tokens::table
            .filter(tokens::token.eq(token))
            .filter(tokens::revoked.eq(false))
            .select(Self::as_select())
            .load(conn)?
            .pop())
    }

    pub fn delete(self, conn: &mut PgConnection) -> Result<usize, DieselError> {
        diesel::delete(dsl::tokens.find(self.id)).execute(conn)
    }

    pub fn revoke(self, conn: &mut PgConnection) -> Result<Self, DieselError> {
        diesel::update(dsl::tokens.find(self.id))
            .set(dsl::revoked.eq(true))
            .returning(Self::as_returning())
            .get_result(conn)
    }
}

#[derive(Insertable)]
#[diesel(table_name = tokens)]
pub struct NewToken {
    name: String,
    token: Vec<u8>,
    revoked: bool,
}

impl NewToken {
    pub fn new(name: String) -> Self {
        Self {
            name,
            revoked: false,
            token: crate::random_bytes(32),
        }
    }
}

#[derive(Default)]
pub struct TokenFilter {
    pub id: Option<i32>,
    pub token: Option<Vec<u8>>,
    pub name: Option<String>,
    pub revoked: Option<bool>,
}

impl TokenFilter {
    pub fn with_id(self, id: Option<i32>) -> Self {
        Self { id, ..self }
    }
}
