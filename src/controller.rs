use actix_web::{get, web, App, HttpResponse, HttpServer, Responder, Result, post};


#[get("/health_check")]
pub async fn health_check() -> impl Responder {
    let response = "bro its okay".to_string();
    HttpResponse::Ok().json(response)
}


#[get("/brag")]
pub async fn get_brag() -> impl Responder {
    let response = Brag {
        title: "I am a title".to_string(),
        description: "I am a description".to_string(),
        impact: "I am an impact".to_string(),
        difficulty: "I am a difficulty".to_string(),
        date: "I am a date".to_string(),
    }; 
    HttpResponse::Ok().json(response.get_all_brags())
}

#[post("/brag")]
pub async fn post_brag() -> impl Responder {
    let response = "I am a post request".to_string();
    HttpResponse::Ok().json(response)
}



pub struct Brag {
    pub title: String,
    pub description: String,
    pub impact : String,
    pub difficulty: String,
    pub date: String,
}

impl BragTrait for Brag {
    fn get_all_brags(&self) -> String {
        format!("{} {} {} {} {}", self.title, self.description, self.impact, self.difficulty, self.date)
    }
}

pub trait BragTrait {
    fn get_all_brags(&self) -> String;
}




