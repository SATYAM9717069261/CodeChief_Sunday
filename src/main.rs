use reqwest::blocking;
use scraper::{Html, Selector};
use serde::Serialize;
use std::env;
use std::error::Error;
use std::fs;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Debug)]
struct Quote {
    text: String,
    author: String,
    tags: Vec<String>,
}

const BASE_URL: &str = "http://quotes.toscrape.com";

fn main() {
    // 1. Setup CLI Input
    let args: Vec<String> = env::args().collect();

    let tag_filter = if args.len() > 1 {
        args[1].clone()
    } else {
        String::new()
    };

    if tag_filter.is_empty() {
        println!("Starting full scrape of all quotes...");
    } else {
        println!("Starting scrape for tag: {}", tag_filter);
    }

    // 2. Determine URL & Start Scrape
    let start_url = build_starting_url(&tag_filter);

    let all_quotes = match scrape_all_pages(&start_url) {
        Ok(q) => q,
        Err(e) => {
            eprintln!("Scraping failed: {}", e);
            return;
        }
    };

    // 3. Export Data
    println!("\n\nScraping complete. Exporting to JSON...");

    if let Err(e) = export_to_json(&all_quotes, "quotes.json") {
        eprintln!("Error saving file: {}", e);
        return;
    }

    println!(
        "Successfully saved {} quotes to quotes.json.",
        all_quotes.len()
    );
}

// --- Step 1: Setup (YOUR TURN) ---

fn build_starting_url(tag: &str) -> String {
    // TODO 1:
    // If tag empty -> page 1
    // Else -> tag route page 1

    if tag.is_empty() {
        format!("{}/page/1/", BASE_URL)
    } else {
        format!("{}/tag/{}/page/1/", BASE_URL, tag)
    }
}

// --- Step 2: Core Scraping Tools ---

fn fetch_and_parse_page(url: &str) -> Result<Html, Box<dyn Error>> {
    // TODO 2:
    // Execute GET request

    let response = blocking::get(url)?;

    // TODO 3:
    // Status validation

    if !response.status().is_success() {
        return Err(format!(
            "Bad response status: {}",
            response.status()
        )
        .into());
    }

    // TODO 4:
    // Convert body -> Html document

    let body = response.text()?;
    let document = Html::parse_document(&body);

    Ok(document)
}

fn extract_tags(element: scraper::ElementRef) -> Vec<String> {
    let mut tags = Vec::new();

    // TODO 6:
    // Target tags

    let tag_selector = Selector::parse(".tags .tag").unwrap();

    // TODO 7:
    // Extract + trim

    for tag in element.select(&tag_selector) {
        tags.push(tag.inner_html().trim().to_string());
    }

    tags
}

fn extract_quotes_from_doc(doc: &Html) -> Vec<Quote> {
    let mut quotes = Vec::new();

    // TODO 8:
    // Select quote blocks

    let quote_selector = Selector::parse(".quote").unwrap();
    let text_selector = Selector::parse(".text").unwrap();
    let author_selector = Selector::parse(".author").unwrap();

    for quote_block in doc.select(&quote_selector) {
        // TODO 9:
        // Extract text + author

        let text = quote_block
            .select(&text_selector)
            .next()
            .map(|e| e.inner_html())
            .unwrap_or_default();

        let author = quote_block
            .select(&author_selector)
            .next()
            .map(|e| e.inner_html())
            .unwrap_or_default();

        // TODO 10:
        // Extract tags

        let tags = extract_tags(quote_block);

        // TODO 11:
        // Build Quote

        quotes.push(Quote {
            text,
            author,
            tags,
        });
    }

    quotes
}

fn get_next_page_url(doc: &Html) -> String {
    // TODO 12:
    // Next button

    let next_selector = Selector::parse(".next a").unwrap();

    // TODO 13 + 14:
    // href extraction

    if let Some(next) = doc.select(&next_selector).next() {
        if let Some(link) = next.value().attr("href") {
            return link.to_string();
        }
    }

    String::new()
}

// --- Step 3: Orchestration ---

fn scrape_all_pages(
    start_url: &str,
) -> Result<Vec<Quote>, Box<dyn Error>> {
    let mut all_quotes = Vec::new();
    let mut current_url = start_url.to_string();
    let mut page_count = 0;

    loop {
        // TODO 15:
        // Fetch page

        let doc = fetch_and_parse_page(&current_url)?;

        // TODO 16:
        // Extract quotes

        let quotes = extract_quotes_from_doc(&doc);

        // TODO 17:
        // Append + increment

        all_quotes.extend(quotes);
        page_count += 1;

        println!(
            "\rScraped {} pages... ({} quotes collected)",
            page_count,
            all_quotes.len()
        );

        // TODO 18:
        // Safety limit

        if page_count > 50 {
            break;
        }

        // TODO 19:
        // Find next

        let next_route = get_next_page_url(&doc);

        // TODO 20:
        // No next -> stop

        if next_route.is_empty() {
            break;
        }

        // TODO 21:
        // Absolute URL

        current_url = format!("{}{}", BASE_URL, next_route);

        // TODO 22:
        // Delay

        thread::sleep(Duration::from_millis(500));
    }

    Ok(all_quotes)
}

// --- Step 4: Export ---

fn export_to_json(
    quotes: &[Quote],
    filename: &str,
) -> Result<(), Box<dyn Error>> {
    // TODO 23:
    // Pretty JSON

    let json = serde_json::to_string_pretty(quotes)?;

    // TODO 24:
    // Write file

    fs::write(filename, json)?;

    Ok(())
}
