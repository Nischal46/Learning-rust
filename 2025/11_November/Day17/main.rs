trait AccessToken {
    fn check_token(&self) -> String;
}

#[derive(Debug)]
struct HttpResponse<'a> {
    status: String,
    status_code: u8,
    token: &'a str

}

//NOTE: providing lifetime parameter
impl AccessToken for HttpResponse<'_> {
    fn check_token(&self) -> String {
        let response = format!("token is {}", &self.status_code);
        response
    }
}

fn main(){
    let api_data = vec![2,3,4,5];
    let api_response = HttpResponse {
        status: "success".to_string(),
        status_code: 200,
        token: "dfgdfgdfgdfgdfgfdg"
        // data: api_data
    };

    println!("{:#?}", api_response);
    println!("{}", api_response.check_token());

}