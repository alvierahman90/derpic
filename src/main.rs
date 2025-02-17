use diesel::result::Error as DieselError;
use dotenvy::dotenv;
use poem::{
    error::{InternalServerError, NotFoundError},
    listener::TcpListener,
    middleware::Cors,
    web::Redirect,
    EndpointExt, IntoResponse, Result, Route,
};
use poem_openapi::{
    param::Header, param::Path, param::Query, payload::Binary, payload::Json, ApiResponse, Object,
    OpenApi, OpenApiService,
};

use derpic::models::*;

struct Api;

#[derive(ApiResponse)]
enum ImageResponse {
    #[oai(status = 200, content_type = "image/png")]
    Png(Binary<Vec<u8>>),
    #[oai(status = 200, content_type = "image/jpeg")]
    Jpeg(Binary<Vec<u8>>),
    #[oai(status = 200, content_type = "image/webp")]
    WebP(Binary<Vec<u8>>),
    #[oai(status = 200, content_type = "image/gif")]
    Gif(Binary<Vec<u8>>),
    #[oai(status = 200, content_type = "image/xyz")]
    Xyz(Binary<Vec<u8>>),
    #[oai(status = 400)]
    BadRequest,
}

#[derive(ApiResponse)]
enum AdminActionsResponse {
    #[oai(status = 201, content_type = "application/json")]
    NewToken(Json<TokenEncodedSlug>),
    #[oai(status = 200, content_type = "application/json")]
    Tokens(Json<Vec<TokenEncodedSlug>>),
    #[oai(status = 500)]
    InternalServerError,
    #[oai(status = 204)]
    DeletedToken,
    #[oai(status = 401)]
    NotAuthorized,
}

#[derive(ApiResponse)]
enum ImageUploadResult {
    #[oai(status = 201, content_type = "application/json")]
    CreatedImage(Json<i32>),
    #[oai(status = 401)]
    NotAuthorized,
}

#[derive(ApiResponse)]
enum ListImagesResponse {
    #[oai(status = 200, content_type = "application/json")]
    Images(Json<Vec<DbImageNoImageData>>),
    #[oai(status = 401)]
    NotAuthorized,
}

#[derive(ApiResponse)]
enum DeleteImagesResponse {
    #[oai(status = 200, content_type = "application/json")]
    Ok(Json<usize>),
}

#[derive(ApiResponse)]
enum MeResponse {
    #[oai(status = 200)]
    Ok(Json<TokenEncodedSlug>),
}

#[derive(Object)]
struct ImageFilters {
    format: String,
    size: Option<usize>,
    resolution_x: Option<usize>,
    rotate: Option<i32>,
}

fn check_admin_token(token: &str) -> bool {
    token == derpic::env::admin_token()
}

#[OpenApi]
impl Api {
    #[oai(path = "/", method = "get")]
    async fn index_redirect(&self) -> poem::Result<()> {
        Err(poem::error::Error::from_response(
            IntoResponse::into_response(Redirect::moved_permanent(format!(
                "{}dash",
                derpic::env::public_base_url()
            ))),
        ))
    }
    #[oai(path = "/admin/tokens", method = "get")]
    async fn get_admin_tokens(
        &self,
        #[oai(name = "X-Derpic-Admin-Token")] admin_token: Header<String>,
    ) -> Result<AdminActionsResponse> {
        if !check_admin_token(&admin_token.0) {
            return Ok(AdminActionsResponse::NotAuthorized);
        }

        let conn = &mut derpic::db::establish_connection();

        match Token::get(conn, TokenFilter::default()) {
            Ok(tokens) => Ok(AdminActionsResponse::Tokens(Json(
                tokens.into_iter().map(|t| t.into()).collect(),
            ))),
            Err(e) => {
                log::error!("{e}");
                Ok(AdminActionsResponse::InternalServerError)
            }
        }
    }

    #[oai(path = "/admin/tokens", method = "post")]
    async fn post_admin_tokens(
        &self,
        #[oai(name = "X-Derpic-Admin-Token")] admin_token: Header<String>,
        token_name: Query<String>,
    ) -> Result<AdminActionsResponse> {
        if !check_admin_token(&admin_token.0) {
            return Ok(AdminActionsResponse::NotAuthorized);
        }
        let conn = &mut derpic::db::establish_connection();

        match Token::new(conn, NewToken::new(token_name.0)) {
            Ok(token) => Ok(AdminActionsResponse::NewToken(Json(token.into()))),
            Err(e) => {
                log::error!("{e}");
                Ok(AdminActionsResponse::InternalServerError)
            }
        }
    }

    #[oai(path = "/admin/tokens/:id", method = "delete")]
    async fn delete_admin_tokens(
        &self,
        #[oai(name = "X-Derpic-Admin-Token")] admin_token: Header<String>,
        id: Path<i32>,
        delete_images: Query<Option<bool>>,
    ) -> Result<AdminActionsResponse> {
        if !check_admin_token(&admin_token.0) {
            return Ok(AdminActionsResponse::NotAuthorized);
        }
        let conn = &mut derpic::db::establish_connection();

        let token = match Token::get(conn, TokenFilter::default().with_id(Some(id.0))) {
            Err(e) => {
                log::error!("{e}");
                return Ok(AdminActionsResponse::InternalServerError);
            }
            Ok(tokens) => tokens,
        }
        .pop();

        if token.is_none() {
            return Err(NotFoundError.into());
        }

        let token = token.unwrap();

        if let Some(true) = delete_images.0 {
            let deleted_images = DbImage::get(
                conn,
                DbImageFilter::default().with_token_id(Some(token.id())),
            )
            .map_err(InternalServerError)?
            .into_iter()
            .map(|e| e.delete(conn))
            .collect::<Vec<Result<usize, DieselError>>>();

            for result in deleted_images {
                result.map_err(InternalServerError)?;
            }
        }

        if let Err(e) = token.revoke(conn) {
            log::error!("{e}");
            return Err(InternalServerError(e));
        }

        Ok(AdminActionsResponse::DeletedToken)
    }

    #[oai(path = "/i/:slug", method = "get")]
    async fn get_image(
        &self,
        #[oai(name = "slug")]
        /// Name of image to get, with file extension.
        slug: Path<String>,
        #[oai(name = "rotation")]
        /// Angle to rotate image by. Valid values are 90, 180, 270, -90, -180, -270.
        rotation: Query<Option<i32>>,
        #[oai(name = "width")]
        /// Maximum width of image. Defaults to actual image width.
        width: Query<Option<u32>>,
        #[oai(name = "height")]
        /// Maximum height of image. Defaults to actual image height.
        height: Query<Option<u32>>,
        #[oai(name = "flipv")]
        /// Flip image vertically.
        flipv: Query<Option<bool>>,
        #[oai(name = "fliph")]
        /// Flip image horizontally.
        fliph: Query<Option<bool>>,
        /// If set, all other options are ignored and raw file as stored is sent.
        raw: Query<Option<bool>>,
    ) -> Result<ImageResponse> {
        let mut slug_split = slug.split('.');
        let Some(slug) = slug_split.next() else {
            return Ok(ImageResponse::BadRequest);
        };
        let Some(requested_extension) =
            image::ImageFormat::from_extension(slug_split.next().unwrap_or("png"))
        else {
            return Ok(ImageResponse::BadRequest);
        };

        let conn = &mut derpic::db::establish_connection();
        let db_image = match DbImage::get_by_slug(conn, slug.to_string()) {
            Err(e) => {
                log::error!("{e}");
                return Err(InternalServerError(Box::new(e)));
            }
            Ok(None) => return Err(NotFoundError.into()),
            Ok(Some(i)) => i,
        };

        let raw = match raw.0 {
            Some(raw) => raw,
            None => match requested_extension {
                image::ImageFormat::Gif => {
                    rotation.is_none()
                        && width.is_none()
                        && height.is_none()
                        && fliph.is_none()
                        && flipv.is_none()
                }
                _ => false,
            },
        };
        if raw {
            return Ok(ImageResponse::Xyz(Binary(db_image.as_raw_image())));
        }

        let mut image = match db_image.as_image() {
            Err(e) => {
                log::error!("{e}");
                return Err(InternalServerError(e));
            }
            Ok(i) => i,
        };

        // rotate image
        if let Some(angle) = rotation.0 {
            image = match angle {
                90 => image.rotate90(),
                180 => image.rotate180(),
                270 => image.rotate270(),
                -90 => image.rotate270(),
                -180 => image.rotate180(),
                -270 => image.rotate90(),
                _ => image,
            };
        };

        // resize image
        image = match (width.0, height.0) {
            (Some(width), Some(height)) => {
                image.resize(width, height, image::imageops::FilterType::Triangle)
            }
            (Some(width), None) => {
                image.resize(width, image.height(), image::imageops::FilterType::Triangle)
            }
            (None, Some(height)) => {
                image.resize(image.width(), height, image::imageops::FilterType::Triangle)
            }
            (None, None) => image,
        };

        if let Some(true) = flipv.0 {
            image = image.flipv();
        }

        if let Some(true) = fliph.0 {
            image = image.fliph();
        }

        let mut response = Vec::new();
        if let Err(e) = image.write_to(
            &mut std::io::Cursor::new(&mut response),
            requested_extension,
        ) {
            log::debug!("{}", e);
            return Err(InternalServerError(e));
        };

        match requested_extension {
            image::ImageFormat::Png => Ok(ImageResponse::Png(Binary(response))),
            image::ImageFormat::Jpeg => Ok(ImageResponse::Jpeg(Binary(response))),
            image::ImageFormat::WebP => Ok(ImageResponse::WebP(Binary(response))),
            image::ImageFormat::Gif => Ok(ImageResponse::Gif(Binary(response))),
            _ => Ok(ImageResponse::Xyz(Binary(response))),
        }
    }

    #[oai(path = "/i", method = "get")]
    async fn get_images(
        &self,
        #[oai(name = "X-Derpic-Token")] token: Header<Option<String>>,
        #[oai(name = "X-Derpic-Admin-Token")] admin_token: Header<Option<String>>,
    ) -> Result<ListImagesResponse> {
        let conn = &mut derpic::db::establish_connection();

        if let Some(token) = admin_token.0 {
            if !check_admin_token(&token) {
                return Ok(ListImagesResponse::NotAuthorized);
            }

            return Ok(ListImagesResponse::Images(Json(
                DbImage::get(conn, DbImageFilter::default())
                    .map_err(InternalServerError)?
                    .into_iter()
                    .map(|image| image.into())
                    .collect(),
            )));
        }

        if token.is_none() {
            return Ok(ListImagesResponse::NotAuthorized);
        }

        let token = match token_decode(token.0.unwrap()) {
            Err(_) => return Err(NotFoundError.into()),
            Ok(token) => token,
        };

        let token = match Token::get_by_token(conn, token) {
            Err(e) => {
                log::error!("{e}");
                return Err(InternalServerError(e));
            }
            Ok(None) => return Ok(ListImagesResponse::NotAuthorized),
            Ok(Some(token)) => token,
        };

        Ok(ListImagesResponse::Images(Json(
            DbImage::get(
                conn,
                DbImageFilter::default().with_token_id(Some(token.id())),
            )
            .map_err(InternalServerError)?
            .into_iter()
            .map(|image| image.into())
            .collect(),
        )))
    }

    #[oai(path = "/i", method = "post")]
    async fn post_image(
        &self,
        raw: Query<Option<bool>>,
        #[oai(name = "X-Derpic-Token")] token: Header<String>,
        data: Binary<Vec<u8>>,
    ) -> Result<ImageUploadResult> {
        let raw = raw.unwrap_or(false);
        let conn = &mut derpic::db::establish_connection();
        let token = match token_decode(token.0) {
            Err(_) => return Ok(ImageUploadResult::NotAuthorized),
            Ok(token) => token,
        };

        let token = match Token::get_by_token(conn, token) {
            Err(e) => {
                log::error!("{e}");
                return Err(InternalServerError(e));
            }
            Ok(None) => return Ok(ImageUploadResult::NotAuthorized),
            Ok(Some(token)) => token,
        };

        let image = if raw {
            data.0
        } else {
            let mut buf = std::io::Cursor::new(vec![]);
            let cursor = std::io::Cursor::new(data.0);
            let dynamic_image = image::io::Reader::new(cursor)
                .with_guessed_format()
                .map_err(InternalServerError)?
                .decode()
                .map_err(InternalServerError)?;
            dynamic_image
                .write_to(&mut buf, image::ImageFormat::Png)
                .unwrap();
            buf.into_inner()
        };

        match DbImage::new(
            conn,
            NewDbImage {
                token_id: token.id(),
                image,
            },
        ) {
            Err(e) => {
                log::error!("{e}");
                Err(InternalServerError(e))
            }
            Ok(image) => Ok(ImageUploadResult::CreatedImage(Json(image.id()))),
        }
    }

    #[oai(path = "/i/:slug", method = "delete")]
    async fn delete_image(
        &self,
        #[oai(name = "slug")]
        /// Name of image to get.
        slug: Path<String>,
        #[oai(name = "X-Derpic-Token")] token: Header<String>,
    ) -> Result<DeleteImagesResponse> {
        let conn = &mut derpic::db::establish_connection();
        let db_image = match DbImage::get_by_slug(conn, slug.0) {
            Err(e) => {
                log::error!("{e}");
                return Err(InternalServerError(Box::new(e)));
            }
            Ok(None) => return Err(NotFoundError.into()),
            Ok(Some(i)) => i,
        };

        let token = match token_decode(token.0) {
            Err(_) => return Err(NotFoundError.into()),
            Ok(token) => token,
        };

        let token = match Token::get_by_token(conn, token) {
            Err(e) => {
                log::error!("{e}");
                return Err(InternalServerError(e));
            }
            Ok(None) => return Err(NotFoundError.into()),
            Ok(Some(token)) => token,
        };

        if db_image.token_id() != token.id() {
            return Err(NotFoundError.into());
        }

        Ok(DeleteImagesResponse::Ok(Json(
            db_image.delete(conn).map_err(InternalServerError)?,
        )))
    }

    #[oai(path = "/me", method = "get")]
    async fn get_me(
        &self,
        #[oai(name = "X-Derpic-Username")] username: Header<String>,
        #[oai(name = "X-Derpic-Token")] token: Header<String>,
    ) -> Result<MeResponse> {
        let conn = &mut derpic::db::establish_connection();

        let token = match token_decode(token.0) {
            Err(_) => return Err(NotFoundError.into()),
            Ok(token) => match Token::get_by_token(conn, token) {
                Ok(Some(token)) => token,
                Ok(None) => return Err(NotFoundError.into()),
                Err(e) => return Err(InternalServerError(e)),
            },
        };

        match username.0 == token.name() {
            true => Ok(MeResponse::Ok(Json(token.into()))),
            false => Err(NotFoundError.into()),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    env_logger::init();
    log::info!(
        "DERPIC_STATIC_FILES={}",
        derpic::env::static_files_directory()
    );

    let conn = &mut derpic::db::establish_connection();
    derpic::db::run_migrations(conn).unwrap();

    let api_service =
        OpenApiService::new(Api, "derpic", "0.1").server(derpic::env::public_base_url());
    let ui_service = api_service.openapi_explorer();

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(
            Route::new()
                .nest(
                    "/dash",
                    poem::endpoint::StaticFilesEndpoint::new(derpic::env::static_files_directory())
                        .index_file("index.html"),
                )
                .nest("/", api_service.with(Cors::new()))
                .nest("/ui", ui_service),
        )
        .await
}
