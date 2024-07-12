use dotenvy::dotenv;
use image::io::Reader as ImageReader;
use poem::{
    error::{InternalServerError, NotFoundError},
    listener::TcpListener,
    Result, Route,
};
use poem_openapi::{
    param::Path, param::Query, payload::Binary, payload::Json, ApiResponse, Object, OpenApi,
    OpenApiService,
};

struct Api;

#[derive(ApiResponse)]
enum ImageResponse {
    #[oai(status = 200, content_type = "image/png")]
    Image(Binary<Vec<u8>>),
}

#[derive(Object)]
struct ImageFilters {
    format: String,
    size: Option<usize>,
    resolution_x: Option<usize>,
    rotate: Option<i32>,
}

#[OpenApi]
impl Api {
    #[oai(path = "/i/:filename", method = "get")]
    async fn get_image(
        &self,
        filename: Path<String>,
        rotation: Query<Option<i32>>,
    ) -> Result<ImageResponse> {
        //dotenv().map_err(InternalServerError)?;

        let mut image = ImageReader::open(filename.0)
            .map_err(|_| NotFoundError)?
            .decode()
            .map_err(InternalServerError)?;
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
