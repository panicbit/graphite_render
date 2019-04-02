use url::Url;
use std::error::Error;
use reqwest::multipart::Form;
use crate::Path;
use crate::types::Response;

pub struct Client {
    http: reqwest::Client,
    render_url: Url,
}

impl Client {
    pub fn new(mut url: Url) -> Result<Self, Box<Error>> {
        url.path_segments_mut()
            .map_err(|_| "Url cannot be a base")?
            .push("render");
        
        Ok(Self {
            http: reqwest::Client::new(),
            render_url: url,
        })
    }

    pub fn request<I, T>(&self, targets: I) -> Result<Response, Box<Error>>
    where
        I: IntoIterator<Item=T>,
        T: AsRef<Path>,
    {
        let form = Form::new()
            .text("noNullPoints", "true")
            .text("format", "json")
            .text("from", "-30s");

        let form = targets
            .into_iter()
            .fold(form, |form, target| form.text("target", target.as_ref().to_string()));

        let response = self.http
            .post(self.render_url.clone())
            .multipart(form)
            .send()?
            .error_for_status()?
            .json::<Response>()?;

        Ok(response)
    }
}
