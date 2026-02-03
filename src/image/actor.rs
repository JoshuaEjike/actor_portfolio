use chrono::Utc;
use reqwest::{Client, multipart};
use sha1::{Digest, Sha1};
use tokio::sync::mpsc;

use crate::{
    errors::api_errors::ApiErrors,
    image::{
        dispatcher::image_dispatcher,
        dto::CloudinaryResponse,
        messages::{ImageMessage, ImageUploadResult},
    },
};

pub struct ImageActor {
    client: Client,
    cloud_name: String,
    api_key: String,
    api_secret: String,
}

impl ImageActor {
    pub fn new(cloud_name: String, api_key: String, api_secret: String) -> Self {
        Self {
            client: Client::new(),
            cloud_name,
            api_key,
            api_secret,
        }
    }

    pub async fn run(self, rx: mpsc::Receiver<ImageMessage>) {
        image_dispatcher(&self, rx).await;
    }

    // pub async fn upload(&self, file: String) -> Result<ImageUploadResult, String> {

    //     let url = format!(
    //         "https://api.cloudinary.com/v1_1/{}/image/upload",
    //         self.cloud_name
    //     );

    //     let res = self
    //         .client
    //         .post(url)
    //         .basic_auth(&self.api_key, Some(&self.api_secret))
    //         .form(&[("file", file)])
    //         .send()
    //         .await
    //         .map_err(|_| "Upload failed")?;

    //     let body: CloudinaryResponse = res.json().await.map_err(|_| "Invalid response")?;

    //     Ok(ImageUploadResult {
    //         url: body.secure_url,
    //         public_id: body.public_id,
    //     })
    // }

    pub async fn upload(&self, file: String) -> Result<ImageUploadResult, ApiErrors> {
        let timestamp = Utc::now().timestamp();

        let mut hasher = Sha1::new();
        hasher.update(format!("timestamp={}{}", timestamp, self.api_secret));
        let signature = format!("{:x}", hasher.finalize());

        let url = format!(
            "https://api.cloudinary.com/v1_1/{}/image/upload",
            self.cloud_name
        );

        let form = multipart::Form::new()
            .text("file", file)
            .text("api_key", self.api_key.clone())
            .text("timestamp", timestamp.to_string())
            .text("signature", signature);

        let res = self
            .client
            .post(url)
            .basic_auth(&self.api_key, Some(&self.api_secret))
            .multipart(form)
            .send()
            .await
            .map_err(|e| ApiErrors::InternalServerError(e.to_string()))?;

        println!("image upload: {res:?}");

        if !res.status().is_success() {
            return Err(ApiErrors::InternalServerError(
                "Cloudinary rejected upload".to_string(),
            ));
        }

        let body: CloudinaryResponse = res.json().await.map_err(|_| {
            ApiErrors::InternalServerError("Invalid Cloudinary response".to_string())
        })?;

        Ok(ImageUploadResult {
            url: body.secure_url,
            public_id: body.public_id,
        })
    }
}
