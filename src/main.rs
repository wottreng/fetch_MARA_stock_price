use std::error::Error;

#[tokio::main]
async fn fetch() -> Result<(), Box<dyn Error>> {
    let body = reqwest::get("https://finance.yahoo.com/quote/MARA/")
        .await?
        .text().await?;

    //println!("body = {:?}", body);

    let substring_pos = body.find("data-symbol=\"MARA\"").unwrap();
    // println!("{}", substring_pos);
    let data = &body[substring_pos..(substring_pos+200)];
    let value_pos = data.find("value=").unwrap();
    let mara_value = &data[(value_pos+7)..(value_pos+12)];
    println!("Mara value: {}", mara_value);
    Ok(())
}

fn main(){
    println!("fetching MARA stock price...");
    let res = fetch();
}
