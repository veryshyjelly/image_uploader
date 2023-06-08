use std::{
    fs,
    io::{Result, Write},
    path::Path,
};

use actix_cors::Cors;
use actix_web::{
    get,
    http::{
        header::{ContentType, CONTENT_LENGTH},
        StatusCode,
    },
    middleware::Logger,
    post, web, App, HttpRequest, HttpResponse, HttpServer,
};

use actix_multipart::Multipart;
use futures_util::TryStreamExt as _;
use image::DynamicImage;
use mime::{Mime, IMAGE_GIF, IMAGE_JPEG, IMAGE_PNG};
use serde::Serialize;
use uuid::Uuid;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        let cors = Cors::permissive();

        App::new()
            .service(root)
            .service(upload)
            .service(get_photo)
            .service(
                actix_files::Files::new("/home", "./image-uploader/out/").index_file("index.html"), // .show_files_listing(),
            )
            .wrap(Logger::default())
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn root() -> String {
    "Server is up and running".to_string()
}

#[derive(Debug, Serialize)]
struct FileUploadResponse {
    link: String,
}

#[post("/upload")]
async fn upload(mut payload: Multipart, req: HttpRequest) -> HttpResponse {
    print!(
        "content-length: {:?}\n",
        req.headers().get(CONTENT_LENGTH).unwrap()
    );
    // 1. limit the file size
    // 2. limit the file count
    // 3. limit the file type
    // 4. save the file
    // 5. convert into gif

    let max_file_size: usize = 100_000;
    let max_file_count: usize = 1;
    let legal_filetypes: [Mime; 3] = [IMAGE_PNG, IMAGE_JPEG, IMAGE_GIF];

    let content_length: usize = match req.headers().get(CONTENT_LENGTH) {
        Some(hv) => hv.to_str().unwrap_or("0").parse().unwrap(),
        None => 0,
    };

    if content_length == 0 || content_length > max_file_size {
        return HttpResponse::BadRequest().into();
    }

    let mut current_count: usize = 0;

    let res_file_id = Uuid::new_v4();

    loop {
        if current_count >= max_file_count {
            break;
        }

        if let Ok(Some(mut field)) = payload.try_next().await {
            let file_type: Option<&Mime> = field.content_type();
            if file_type.is_none() {
                continue;
            }
            if !legal_filetypes.contains(file_type.unwrap()) {
                continue;
            }

            let dir = "./data/images/";

            let destination: String = format!(
                "{}{}-{}",
                dir,
                Uuid::new_v4(),
                field.content_disposition().get_filename().unwrap(),
            );

            let mut saved_file = fs::File::create(&destination).unwrap();
            while let Ok(Some(chunk)) = field.try_next().await {
                let _ = saved_file.write_all(&chunk).unwrap();
            }

            web::block(move || async move {
                let updated_img: DynamicImage = image::open(&destination).unwrap();
                let _ = fs::remove_file(&destination).unwrap();
                let _ = updated_img
                    .resize_exact(200, 200, image::imageops::FilterType::Triangle)
                    .save(&format!("{}{}.gif", dir, res_file_id))
                    .unwrap();
            })
            .await
            .unwrap()
            .await;
        } else {
            break;
        }

        current_count += 1;
    }

    HttpResponse::build(StatusCode::OK)
        .content_type(ContentType::json())
        .body(
            serde_json::to_string(&FileUploadResponse {
                link: format!("/photo/{}", res_file_id),
            })
            .unwrap(),
        )
}

#[get("/photo/{photo_id}")]
async fn get_photo(path: web::Path<String>) -> Result<HttpResponse> {
    let photo_id = path.into_inner();

    let image_path = format!("./data/images/{}.gif", photo_id);

    if Path::new(&image_path).is_file() {
        let image_content = web::Bytes::from(std::fs::read(image_path).unwrap());
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type("image/gif")
            .body(image_content))
    } else {
        Ok(HttpResponse::build(StatusCode::NOT_FOUND).body("requested file not found"))
    }
}
