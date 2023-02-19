use anyhow::{Result, Ok};
use reqwest::Url;
use serde::Serialize;
use voyager::{scraper::Selector, Scraper};

pub struct BookScraper {
  item_selector: Selector,
  image_selector: Selector,
  title_selector: Selector,
  pagination_selector: Selector,
  base_url: Url
}

impl Default for BookScraper {
  fn default() -> Self {
    Self {
      item_selector: Selector::parse(".category-title-container a").unwrap(),
      title_selector: Selector::parse(".category-title-title").unwrap(),
      image_selector: Selector::parse(".bookimage").unwrap(),
      pagination_selector: Selector::parse(".pagination-list li a").unwrap(),
      base_url: Url::parse("https://pragprog.com").unwrap()
    }
  }
}

#[derive(Debug, Serialize)]
pub struct ShopItem {
  pub title: String,
  pub image_url: String,
  pub link: String,
}

#[derive(Debug)]
pub enum StoreState {
  Book(ShopItem),
  Page(usize)
}

impl Scraper for BookScraper {
  type Output = ShopItem;
  type State = StoreState;

  fn scrape(
    &mut self,
    response: voyager::Response<Self::State>,
    crawler: &mut voyager::Crawler<Self>,
  ) -> Result<Option<Self::Output>> {
    let html = response.html();

    if let Some(state) = response.state {
      match state {
        StoreState::Page(page) => {
          for item in html.select(&self.item_selector) {
            let title: String = item.select(&self.title_selector).next()
              .map(|d| d.text().collect()).unwrap();
            let image = item.select(&self.image_selector).next()
              .map(|d| d.value().attr("src").unwrap()).unwrap();
            let link = item.value().attr("href").unwrap().to_owned();
            let shop_item = ShopItem {
              title,
              image_url: format!("{}{image}", self.base_url),
              link
            };
            crawler.visit_with_state(shop_item.link.clone(), StoreState::Book(shop_item))
          }
          if let Some(next_page_url) = html.select(&self.pagination_selector)
            .last().unwrap().value().attr("href") {
              crawler.visit_with_state(format!("{}{next_page_url}", self.base_url), StoreState::Page(page + 1))
            }
        },
        StoreState::Book(book) => {
          return Ok(Some(book));
        }
      }
    } else {
      crawler.visit_with_state(response.response_url.to_string(), StoreState::Page(0))
    }
    Ok(None)
  }
}