use std::collections::HashMap;

use excel_extractor::BookScraper;
use futures::StreamExt;
use umya_spreadsheet::{reader, writer};
use voyager::{Collector, CrawlerConfig};

#[tokio::main]
async fn main() {
  let mut book = reader::xlsx::read("./scrape.xlsx").unwrap();

  let conf = CrawlerConfig::default().allow_domain("pragprog.com");
  let mut collector = Collector::new(BookScraper::default(), conf);
  collector.crawler_mut().visit("https://pragprog.com/titles/");
  let mut keywords: HashMap<String, (u32, u32)> = HashMap::default();
  
  let sheet = book.get_sheet_mut(&0).unwrap();
  
  for col in 2..sheet.get_highest_column() {
    if let Some(keyword) = sheet.get_cell_by_column_and_row(&col, &2) {
      keywords.insert(keyword.get_value().into(), (col as u32, 2));
    }
  }

  while let Some(data) = collector.next().await {
    if let Ok(item) = data {
      for (keyword, pos) in keywords.iter_mut() {
        if item.title.contains(keyword) {
          println!("Found a book that contains the keyword `{keyword}`: {}", &item.title);
          let new_pos = (pos.0, pos.1 + 1);
          *pos = new_pos;
          sheet.get_cell_value_by_column_and_row_mut(&pos.0, &pos.1).set_value_from_string(item.title.clone());
          sheet.get_cell_value_by_column_and_row_mut(&(pos.0 + 1), &pos.1).set_value_from_string(item.image_url.clone());
          sheet.get_cell_value_by_column_and_row_mut(&(pos.0 + 2), &pos.1).set_value_from_string(item.link.clone());    
        }
      }
    }
  }
  let _ = writer::xlsx::write(&book, "./scrape.xlsx");
}