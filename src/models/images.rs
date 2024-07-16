use super::Token;
use crate::schema::images::{self as table, dsl};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use poem_openapi::Object;

fn slug_encode(slug: Vec<u8>) -> String {
    URL_SAFE_NO_PAD.encode(slug)
}

fn slug_decode(slug: String) -> Result<Vec<u8>, base64::DecodeError> {
    URL_SAFE_NO_PAD.decode(slug)
}

#[derive(Object, Queryable, Selectable, Associations, Debug, PartialEq)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(Token))]
#[diesel(table_name = table)]
pub struct DbImage {
    id: i32,
    token_id: i32,
    image: Vec<u8>,
    slug: Vec<u8>,
}

#[derive(Object)]
pub struct DbImageNoImageData {
    id: i32,
    token_id: i32,
    slug: String,
}

impl From<DbImage> for DbImageNoImageData {
    fn from(other: DbImage) -> Self {
        Self {
            id: other.id,
            token_id: other.token_id,
            slug: slug_encode(other.slug),
        }
    }
}

impl DbImage {
    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn token_id(&self) -> i32 {
        self.token_id
    }

    pub fn as_image(self) -> Result<image::DynamicImage, image::ImageError> {
        let cursor = std::io::Cursor::new(self.image);
        image::io::Reader::new(cursor)
            .with_guessed_format()?
            .decode()
    }

    pub fn new(conn: &mut PgConnection, new: NewDbImage) -> Result<Self, DieselError> {
        diesel::insert_into(table::table)
            .values::<NewDbImageWithSlug>(new.into())
            .returning(Self::as_returning())
            .get_result(conn)
    }

    pub fn get(conn: &mut PgConnection, filters: DbImageFilter) -> Result<Vec<Self>, DieselError> {
        let mut query = table::table.into_boxed();

        if let Some(val) = filters.id {
            query = query.filter(table::id.eq(val));
        }

        if let Some(val) = filters.token_id {
            query = query.filter(table::token_id.eq(val));
        }

        query.select(Self::as_select()).load(conn)
    }

    pub fn get_by_slug(conn: &mut PgConnection, slug: String) -> Result<Option<Self>, DieselError> {
        let slug = match slug_decode(slug) {
            Err(_) => return Ok(None),
            Ok(slug) => slug,
        };

        Ok(table::table
            .filter(table::slug.eq(slug))
            .select(Self::as_select())
            .load(conn)?
            .pop())
    }

    pub fn delete(self, conn: &mut PgConnection) -> Result<usize, DieselError> {
        diesel::delete(dsl::images.find(self.id)).execute(conn)
    }
}

pub struct NewDbImage {
    pub token_id: i32,
    pub image: Vec<u8>,
}

#[derive(Insertable)]
#[diesel(table_name = table)]
struct NewDbImageWithSlug {
    pub token_id: i32,
    pub image: Vec<u8>,
    pub slug: Vec<u8>,
}

impl From<NewDbImage> for NewDbImageWithSlug {
    fn from(other: NewDbImage) -> Self {
        Self {
            token_id: other.token_id,
            image: other.image,
            slug: crate::random_bytes(16),
        }
    }
}

#[derive(Default)]
pub struct DbImageFilter {
    id: Option<i32>,
    token_id: Option<i32>,
}

impl DbImageFilter {
    pub fn with_token_id(self, token_id: Option<i32>) -> Self {
        Self { token_id, ..self }
    }
}
