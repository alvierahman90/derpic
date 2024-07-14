use std::env;
use dotenvy::dotenv;
use image::io::Reader as ImageReader;
use poem::{
    error::{InternalServerError, NotFoundError},
    listener::TcpListener,
    Result, Route,
};
use poem_openapi::{
    param::Path, param::Query, param::Header, payload::Binary, payload::Json, ApiResponse, Object, OpenApi,
    OpenApiService,
};

use derpic::models::*;

struct Api;

#[derive(ApiResponse)]
enum ImageResponse {
    #[oai(status = 200, content_type = "image/png")]
    Image(Binary<Vec<u8>>),
}

#[derive(ApiResponse)]
enum AdminActionsResponse {
    #[oai(status = 201, content_type = "application/json")]
    NewToken(Json<Token>),
    #[oai(status = 200, content_type = "application/json")]
    Tokens(Json<Vec<Token>>),
    #[oai(status = 500)]
    InternalServerError,
    #[oai(status = 204)]
    DeletedToken,
    #[oai(status = 404)]
    TokenNotFound,
    #[oai(status = 401)]
    NotAuthorized,
}

#[derive(Object)]
struct ImageFilters {
    format: String,
    size: Option<usize>,
    resolution_x: Option<usize>,
    rotate: Option<i32>,
}

const DERPIC_ADMIN_TOKEN: &str = "DERPIC_ADMIN_TOKEN";

fn check_admin_token(token: &str) -> bool {
    match dotenv() {
        Err(_) => return false,
        _ => (),
    }
        match env::var(DERPIC_ADMIN_TOKEN) {
            Err(e) => {
                log::error!("{e}");
                false
            }
            Ok(admin_token) => token == admin_token,
        }

}

#[OpenApi]
impl Api {
    #[oai(path = "/admin/tokens", method = "get")]
    async fn get_admin_tokens(
        &self,
        #[oai(name = "X-Derpic-Admin-Token")]
        admin_token: Header<String>,
        ) -> Result<AdminActionsResponse> {
        if !check_admin_token(&admin_token.0) {
            return Ok(AdminActionsResponse::NotAuthorized);
        }

        let conn = &mut derpic::db::establish_connection();
        
        match Token::get(conn, TokenFilter::default()) {
            Ok(tokens) => Ok(AdminActionsResponse::Tokens(Json(tokens))),
            Err(e) => {
                log::error!("{e}");
                Ok(AdminActionsResponse::InternalServerError)
            }
        }

    }

    #[oai(path = "/admin/tokens", method = "post")]
    async fn post_admin_tokens(
        &self,
        #[oai(name = "X-Derpic-Admin-Token")]
        admin_token: Header<String>,
        token_name: Query<String>
        ) -> Result<AdminActionsResponse> {
        if !check_admin_token(&admin_token.0) {
            return Ok(AdminActionsResponse::NotAuthorized);
        }
        let conn = &mut derpic::db::establish_connection();
        
        match Token::new(conn, NewToken::new(token_name.0)) {
            Ok(token) => Ok(AdminActionsResponse::NewToken(Json(token))),
            Err(e) => {
                log::error!("{e}");
                Ok(AdminActionsResponse::InternalServerError)
            }
        }

    }

    #[oai(path = "/admin/tokens/:id", method = "delete")]
    async fn delete_admin_tokens(
        &self,
        #[oai(name = "X-Derpic-Admin-Token")]
        admin_token: Header<String>,
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
        }.pop();

        if let Some(token) = token {
            match token.revoke(conn) {
                Ok(_) => Ok(AdminActionsResponse::DeletedToken),
                Err(e) => {
                    log::error!("{e}");
                    Ok(AdminActionsResponse::InternalServerError)
                }
            }
        } else {
            Ok(AdminActionsResponse::TokenNotFound)
        }

    }

    #[oai(path = "/i/:filename", method = "get")]
    async fn get_image(
        &self,
        #[oai(name = "filename")]
        /// Name of image to get.
        filename: Path<String>,
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
    ) -> Result<ImageResponse> {

        log::debug!("height={:?}", height.0);


        // load image
        let mut image = ImageReader::open(filename.0)
            .map_err(|_| NotFoundError)?
            .decode()
            .map_err(InternalServerError)?;

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
            (Some(width), Some(height)) => image.resize(width, height, image::imageops::FilterType::Triangle),
            (Some(width), None) => image.resize(width, image.height(), image::imageops::FilterType::Triangle),
            (None, Some(height)) => image.resize(image.width(), height, image::imageops::FilterType::Triangle),
            (None, None) => image,
        };

        if let Some(true) = flipv.0 {
            image = image.flipv();
        }

        if let Some(true) = fliph.0 {
            image = image.fliph();
        }

        let mut response = Vec::new();
        image
            .write_to(
                &mut std::io::Cursor::new(&mut response),
                image::ImageFormat::Png,
                //image::ImageFormat::from_extension(format.0).ok_or(poem::Error::from_status(
                //poem::http::StatusCode::BAD_REQUEST,
                //))?,
            )
            .map_err(InternalServerError)?;

        Ok(ImageResponse::Image(Binary(response)))
    }
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let api_service = OpenApiService::new(Api, "derpic", "0.1").server("http://localhost:3000/");
    let ui_service = api_service.openapi_explorer();

    poem::Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(Route::new().nest("/", api_service).nest("/ui", ui_service))
        .await
}
