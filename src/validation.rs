
pub fn check_is_url(url: String) -> String {
    if (url.find("http://") != None) || (url.find("https://") != None) {
        return url;
    } else {
        return "".to_string();
    }
}