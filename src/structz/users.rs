use chrono::{Local, NaiveTime};
use std::collections::HashMap;
use scraper::{Html,Selector};
use reqwest;
use regex::Regex;
pub struct User {
    pub username: String,
    pub password: String,
}
impl User {
    pub fn new(username: String, password: String) -> User {
        User{
            username,
            password,
        }
    }
    // pub fn change_password(&mut self,pass: String){
    //     self.password = pass;
    // }
    // pub fn change_username(&mut self,user: String){
    //     self.username = user;
    // }
    pub async fn get_notes(&self,log_info: &HashMap<&str, &str>) -> Result<Vec<Vec<String>>, Box<(dyn std::error::Error + Send + Sync)>>  { 
        let client = reqwest::Client::builder().cookie_store(true).build()?;
        client.post("https://www.pepal.eu/include/php/ident.php").form(&log_info).send().await?;
        let req = client.get("https://www.pepal.eu/?my=notes").send().await?;
        let body = req.text().await?;
        let parsed_html = Html::parse_fragment(&body);
        let selector = &Selector::parse("tr.note_devoir").unwrap();
        let mut notes = vec![];
        for element  in parsed_html.select(selector){
            //retirer les \n \t puis enlever tous les espaces et enfin stocker le tout dans un Vecteur
            let mut info_txt = element.text().map(|s| s.trim().to_string()).filter(|s| !s.is_empty()).collect::<Vec<_>>();
            info_txt.remove(1);
            info_txt.swap(0,1);
            notes.push(info_txt);
        }
        Ok(notes)
    }
    pub async fn set_presence(&self,log_info: &HashMap<&str, &str>) -> Result<(), Box<(dyn std::error::Error + Send + Sync)>> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;
        client
            .post("https://www.pepal.eu/include/php/ident.php")
            .form(&log_info)
            .send()
            .await?;
        let resp = client.get("https://www.pepal.eu/presences").send().await?; // récupère la page des notes  
        let body = resp.text().await?;
        let re = Regex::new("<td><a href=\"(.*?)\" class=\"btn btn-primary\"><i class=\"icon wb-list\"></i> <span class=\"hidden-sm-down\">Relevé de présence</span></a></td>").unwrap();

        let test = re.captures_iter(&body)
        .map(|story| {
            story[1].to_string()
        }).collect::<Vec<_>>();
        // println!("{:?}",test); // voir si tout se passe bien
        let presence_id = test.iter()
            .map(|element| {
                element.split("/")
                .skip(3).collect::<Vec<_>>()
            }).collect::<Vec<Vec<_>>>();
        // println!("{:?}",presence_id); /: refer to l.64
        let mut param = HashMap::new();
        param.insert("act", "set_present"); // to set présent
        let seance_pk_idx = if is_past_noon() { // get seance id morning/after-noon
            println!("id de l'aprem");
            1
        } else {
            println!("id du matin");
            0
        };
        param.insert("seance_pk", presence_id[seance_pk_idx][0]);
        println!("{:?}",param);

        client.post("https://www.pepal.eu/student/upload.php").form(&param).send().await?; //valider la présence 
        println!("Success"); //c pas vraiment vrai mais je patch ça after !!!!
        // impl to say if presence is set or already set of error (like page not found, exist, error)   
        Ok(())
    }
}
fn is_past_noon() -> bool{
    let time_of_day = Local::now().time();
    let past_noon = NaiveTime::from_hms(12, 0, 0);
    if time_of_day > past_noon{
        true
    }else{
        false
    }
}