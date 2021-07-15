#[derive(Debug)]
 pub struct RedirectURL {

    // The Facebook url preamble for the oath dialog.
    facebook_oath_url: String,

    // The ID of your app, found in your app's dashboard.
     client_id: String,

    // The URL that you want to redirect the person logging in back to.
     redirect_uri: String,

    // A string value created by your app to maintain state between the request and callback.
    //todo randomly generate this
     state: String,

    // Determines whether the response data included when the redirect back to the app occurs is in URL parameters or fragments.
     response_type: ResponseType,

    // A comma or space separated list of Permissions to request from the person.
     scope: Vec<String>,
}
#[derive(Debug)]
 pub struct ResponseType {
    // Response data is included as URL parameters and contains code parameter (an encrypted string unique to each login request). This is the default behavior.
     code: String,

    // Response data is included as a URL fragment and contains an access token. Desktop apps must use this setting for response_type. This is most useful when the client will be handling the token.
     token: String,

    // Response data is included as a URL fragment and contains both an access token and the code parameter.
     code20token: String,

    // A comma-separated list of all Permissions granted to the app by the user at the time of login.
    // Can be combined with other response_type values.
    // When combined with token, response data is included as a URL fragment, otherwise included as a URL parameter.
     granted_scopes: Vec<String>,
}

#[allow(dead_code)]
impl RedirectURL {
    pub fn new_redirect_url(facebook_oath_url: String, client_id: String, redirect_uri: String, state: String, response_type: ResponseType, scope: Vec<String>) -> RedirectURL {
        RedirectURL {
            facebook_oath_url,
            client_id,
            redirect_uri,
            state,
            response_type,
            scope
        }
    }
    fn get_facebook_oath_url(&self) -> &String {
        &self.facebook_oath_url
}
    fn get_client_id(&self) -> &String {
        &self.client_id
}
    fn get_redirect_uri(&self) -> &String {
        &self.redirect_uri
    }
    fn get_state(&self) -> &String {
        &self.state
    }
    fn get_response_type(&self) -> &ResponseType {
        &self.response_type
    }
    fn get_scope(&self) -> &Vec<String> {
        &self.scope
    }

    fn set_facebook_oath_url(& mut self) -> &mut String {
        &mut self.facebook_oath_url
    }
    fn set_client_id(&mut self) -> &mut String {
        &mut self.client_id
    }
    fn set_redirect_uri(&mut self) -> &mut String {
        &mut self.redirect_uri
    }
    fn set_state(&mut self) -> &mut String {
        &mut self.state
    }
    fn set_response_type(&mut self) -> &mut ResponseType {
        &mut self.response_type
    }
    fn set_scope(&mut self) -> &mut Vec<String> {
        &mut self.scope
    }
}

#[allow(dead_code)]
impl ResponseType {
    pub fn new_response_type(code: String, token: String, code20token: String, granted_scopes: Vec<String>) -> ResponseType {
        ResponseType {
            code,
            token,
            code20token,
            granted_scopes,
        }
    }
    fn get_code(&self) -> &String {
        &self.code
    }
    fn get_token(&self) -> &String {
        &self.token
    }
    fn get_code20token(&self) -> &String {
        &self.code20token
    }
    fn get_granted_scopes(&self) -> &Vec<String> {
        &self.granted_scopes
    }

    fn set_code(&mut self) -> &str {
        &self.code
    }
    fn set_token(&mut self) -> &str {
        &self.token
    }
    fn set_code20token(&mut self) -> &str {
        &self.code20token
    }
    fn set_granted_scopes(&mut self) -> &Vec<String> {
        &mut self.granted_scopes
    }
}

pub fn main() {


    let response_type = ResponseType::new_response_type(
                            "some code".parse().unwrap(),
                            "the access token".parse().unwrap(),
                            "I don't remember".parse().unwrap(),
                            vec!["username".to_string(), "likes".to_string()]);

    let redirect_uri = r#"&redirect_uri={"https://www.domain.com/login"}"#;

    let redirect_url = RedirectURL::new_redirect_url("https://www.facebook.com/v11.0/dialog/oauth?".parse().unwrap(),
                                 "client_id={339031417714735}".parse().unwrap(),
                                 redirect_uri.parse().unwrap(),
                                 "this is random :O".parse().unwrap(),
                                 response_type,
                                 vec!["test".to_string()],
    );

    println!("{:?}", redirect_url);

}

