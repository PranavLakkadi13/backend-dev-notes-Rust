use rocket::{
    Data,
    data::ToByteUnit,
    http::ContentType,
    launch, post, routes,
    tokio::{
        fs::File,
        io::{AsyncReadExt, AsyncWriteExt},
    },
};
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};

#[post("/upload", data = "<data>")]
async fn upload(content_type: &ContentType, data: Data<'_>) -> Result<String, std::io::Error> {
    let options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("file").size_limit(u64::from(32.mebibytes())),
    ]);

    // here we will just get the file and dump the file in the current codebase no cloud
    let multi_form_data = MultipartFormData::parse(content_type, data, options)
        .await
        .unwrap();

    let file = multi_form_data.files.get("file");

    if let Some(file_fields) = file {
        let file_field = &file_fields[0];
        let filename = &file_field.file_name;
        let content_type = &file_field.content_type;

        println!(
            "the file name: {:?} , the type of content: {:?}",
            &filename, &content_type
        );

        let mut file = File::create(filename.clone().unwrap()).await.unwrap();

        let path = &file_field.path;

        let mut temp_file = File::open(path).await.unwrap();

        let mut buffer = Vec::new();

        temp_file.read_to_end(&mut buffer).await.unwrap();

        file.write_all(&buffer).await.unwrap();

        return Ok("File processed".to_string());
    }

    Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Upload failed.....",
    ))
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![upload])
}
