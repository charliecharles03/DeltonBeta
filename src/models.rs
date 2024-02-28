pub struct Brag {
    pub title: String,
    pub description: String,
    pub impact : String,
    pub difficulty: String,
    pub date: String,
}

impl get_all_brag for Brag {
    fn get_all_brag(&self) -> String {
        format!("{} {} {} {} {}", self.title, self.description, self.impact, self.difficulty, self.date)
    }
}

pub trait BragTrait {
    fn get_brags(&self) -> String;
}




