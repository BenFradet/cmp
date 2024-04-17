use reqwest::IntoUrl;

trait Search {
    fn search<T>(&self, search_term: &str) -> T where T: IntoUrl;
}