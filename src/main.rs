use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::{error::Error, sync::Arc, thread::sleep, time::Duration};
use url::Url;

fn main() -> Result<(), Box<dyn Error>> {
    let options = LaunchOptionsBuilder::default().headless(false).build()?;
    let browser = Browser::new(options)?;
    let tab = browser.wait_for_initial_tab()?;
    tab.navigate_to("http://fizika.sc-nm.si/")?;
    tab.wait_until_navigated()?;

    let lines = get_links(&tab)?;
    let url = Url::parse(&tab.get_url())?;

    for line in lines {
        let new_address = url.join(&line)?;
        tab.navigate_to(&new_address.to_string())?;
        tab.wait_until_navigated()?;
        process_tab(&tab)?;
        sleep(Duration::from_secs(1));
    }

    Ok(())
}

fn get_links(tab: &Arc<Tab>) -> Result<Vec<String>, Box<dyn Error>> {
    let mut links = Vec::new();
    let elements = tab.find_elements("body > div a")?;
    for element in elements {
        let attributes = element.get_attributes()?;
        let attributes = attributes.unwrap();
        let on_click = attributes.get("onclick").unwrap();
        let start = "window.open(\'".len();
        let end = on_click
            .chars()
            .skip(start)
            .position(|c| c == '\'')
            .unwrap();
        let tab_str = on_click.chars().skip(start).take(end).collect::<String>();
        links.push(tab_str);
    }
    Ok(links)
}

#[derive(Default)]
struct Chapter {
    title: String,
    exercises: Vec<Exercise>,
}

enum MediaType {
    Image,
    Video,
    Audio,
    Hint,
    Solution,
    Button,
}

struct Exercise {
    content: Vec<MediaType>,
}

fn process_tab(tab: &Arc<Tab>) -> Result<Chapter, Box<dyn Error>> {
    let mut a = Chapter::default();
    a.title = tab.get_title()?;
    let items = tab.find_elements("#container div")?;
    println!("{:?}", &items);
    Ok(a)
}
