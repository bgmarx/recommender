extern crate reqwest;
extern crate failure;
extern crate csv;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
use std::collections::HashMap;

struct WishlistEntry {
    user_id: usize,
    book_id: usize,
}

#[derive(Debug, Deserialize, Serialize)]
struct Book {
    book_id: usize,
    title: String,
}

fn deserialize_ratings(path: &Path) -> Result<Vec<WishlistEntry>, failure::Error> {
    let mut reader = csv::Reader::from_path(path)?;

    let entries = reader.deserialize()
        .collect::<Result<Vec<_>, _>>()?;

    Ok(entries)
}

fn deserialize_books(path: &Path) -> Result<(HashMap<usize, String>, HashMap<String, usize>), failure::Error> {
    let mut reader = csv::Reader::from_path(path)?;

    let entries: Vec<Book> = reader.deserialize::<Book>()
        .collect::<Result<Vec<_>, _>>()?;
        

}
fn download(url: &str, destination: &Path) -> Result<(), failure::Error> {
    //if file exists return
    if destination.exists() {
        return Ok(())
    }

    let file = File::create(destination)?;

    let mut writer = BufWriter::new(file);

    let mut response = reqwest::get(url)?;
    response.copy_to(&mut writer)?;

    Ok(())
}

fn download_data(ratings_path: &Path, books_path: &Path) {
    let ratings_url = "https://github.com/zygmuntz/\
                       goodbooks-10k/raw/master/ratings.csv";
    let books_url = "https://github.com/zygmuntz/\
                     goodbooks-10k/raw/master/books.csv";
    download(&ratings_url,
             ratings_path).expect("couldn't DL ratings");
    download(&books_url,
             books_path).expect("couldn't LD metadata");
}
