use seed::fetch::fetch;
use seed::prelude::{Method, Request};
use seed::{prelude::*, *};
use serde::{Deserialize, Serialize};
use urlencoding::encode;

#[derive(Deserialize, Debug, Clone,  Default, Serialize)]
pub struct InstaPostParams {
    pub url: String,
    pub caption: String,
    pub location_id: String,
    pub tag_users: Vec<User>,
}

#[derive(Deserialize,Clone, Debug, Default, Serialize)]
pub struct User {
    username: String,
    x: f32,
    y: f32,
}


impl InstaPostParams {
    /// This method let developer update the feed parameters by keeping tract of
    /// each  inputted values
    pub fn new(
        mut self,
        media_url: String,
        caption: String,
        location_tag: String,
        user: User,
    ) -> Self {
        if !media_url.is_empty() {
            self.url = media_url;
        } else if !caption.is_empty() {
            self.caption = caption;
        } else if !location_tag.is_empty() {
            self.location_id = location_tag
        } else if !user.username.is_empty() {
            self.tag_users.push(user);
        }
        self
    }
}

#[derive(Deserialize, Debug, Clone, Default, Serialize)]
pub struct InstaMediaContainerId {
    pub id: String,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct InstagramPostApi {
    access_token: String,
    base_url: String,
}

impl InstagramPostApi {
    pub fn new(access_token: String, base_url: String) -> InstagramPostApi {
        InstagramPostApi {
            access_token,
            base_url,
        }
    }

    /// The ig  container is required to publishing your video, the feed and its
    /// parameter is sent in feed request to the IG container, then a
    /// container ID is sent back that let your published your video
    /// "publish_video " method. This method is expecting a  InstaPostParams
    /// struct  and posting_type ("video"or photo), https://developers.facebook.com/docs/instagram-api/reference/ig-user/media#creating
    ///
    /// Currently, it takes about video seconds to a minute for the feed
    /// container to be ready.
    pub async fn ig_media_container(
        self,
        post_params: InstaPostParams,
        media_type: String,
    ) -> seed::fetch::Result<InstaMediaContainerId> {
        let base_url = self.base_url.replace("EDGE", "media");
        let mut url: String;
        let caption = encode(&post_params.caption);
        if media_type == "video" {
            url = base_url
                + "?media_type=VIDEO"
                + "&video_url="
                + &post_params.url
                + "&access_token="
                + &self.access_token;
        } else {
            url =
                base_url + "?image_url=" + &post_params.url + "&access_token=" + &self.access_token;
        }

        if !post_params.location_id.is_empty() {
            url = url + "location_id=" + &post_params.location_id
        };
        if !post_params.caption.is_empty() {
            url = url + "&caption=" + &caption.to_string()
        };

        let request = Request::new(url).method(Method::Post);
    fetch(request).await?.json::<InstaMediaContainerId>().await

    }


    /// This  should be used when the container id of the feed is ready, this
    /// method will published the feed that you have made,
    /// It only accept the  container id of the feed you want to published.
    pub async fn publish_media(
        self,
        insta_container_id: String,
    ) -> seed::fetch::Result<InstaMediaContainerId> {
        let self_data = self.clone();

        let base_url = self_data.base_url.replace("EDGE", "media_publish");
        let url = base_url.to_string()
            + "?creation_id="
            + &insta_container_id
            + "&access_token="
            + &self_data.access_token;
        let request = Request::new(url).method(Method::Post);
        fetch(request).await?.json::<InstaMediaContainerId>().await
    }
}
