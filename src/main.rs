use headless_chrome::{Browser, Element, LaunchOptionsBuilder, Tab};
use scraper::{Html, Selector};
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
        let attributes = element.get_attributes()?.expect("No attributes");
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
    Text(String),
    Image,
    Video,
    Audio,
    Hint,
    Solution,
    Button,
}

struct Exercise {
    content: Vec<MediaType>,
    num_popups: usize,
}

struct Popup {
    content: Vec<MediaType>,
}

fn process_tab(tab: &Arc<Tab>) -> Result<Chapter, Box<dyn Error>> {
    let items = tab.find_elements("#container > .eplxSlide")?;

    for item in items.iter().skip(4) {
        let html = item
            .call_js_fn("function() { return this.innerHTML;}", false)?
            .value
            .expect("Can't get innerHTML on div");

        let attributes = item.get_attributes()?.expect("No attributes");
        let popup = attributes
            .get("class")
            .expect("No classes in page div")
            .contains("popupImpl");
        let fragment = Html::parse_fragment(html.as_str().expect("Can't parse HTML"));
        parse_fragment(fragment, popup);
    }

    Ok(Chapter {
        title: "".to_string(),
        exercises: Vec::new(),
    })
}

fn parse_fragment(fragment: Html, popup: bool) -> Exercise {
    let selector = if popup {
        Selector::parse("div.popupContent").expect("Can't parse popup selector")
    } else {
        Selector::parse("p.subheading, div.content.interactive-area").expect("Can't parse selector")
    };

    for element in fragment.select(&selector) {
        println!("{}", element.inner_html());
    }

    Exercise {
        content: Vec::new(),
        num_popups: 0,
    }
}
