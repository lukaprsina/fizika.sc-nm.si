use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
    sync::Arc,
    thread::sleep,
    time::Duration,
};
use url::Url;

fn main() -> Result<(), Box<dyn Error>> {
    let options = LaunchOptionsBuilder::default().headless(false).build()?;
    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to("http://fizika.sc-nm.si/")?;
    tab.wait_until_navigated()?;

    let lines = get_links(&tab)?;
    let url = Url::parse(&tab.get_url())?;

    /* for line in lines {
        let new_address = url.join(&line)?;
        println!("{}", new_address);
        tab.navigate_to(&new_address.to_string())?;
        tab.wait_until_navigated()?;
        process_tab(&tab)?;
        sleep(Duration::from_secs(1));
    } */

    Ok(())
}

fn get_links(tab: &Arc<Tab>) -> Result<Vec<String>, Box<dyn Error>> {
    let links = Vec::new();
    let elements = tab.find_elements("body > div a")?;
    for element in elements {
        let attributes = element.get_attributes()?;
        if let Some(attributes) = attributes {
            if let Some(on_click) = attributes.get("onclick") {
                println!("{}", on_click);
            }
        }
    }
    Ok(links)
}

fn process_tab(_tab: &Arc<Tab>) -> Result<(), Box<dyn Error>> {
    Ok(())
}
