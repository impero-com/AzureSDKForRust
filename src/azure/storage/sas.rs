use url::Url;

pub struct SasToken<K, V>
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    params: Vec<(K, V)>,
}

impl<K, V> SasToken<K, V>
where
    K: AsRef<str>,
    V: AsRef<str>,
{
    pub fn new(params: Vec<(K, V)>) -> Self {
        Self { params }
    }

    pub fn add_to_uri(&self, uri: &str) -> String {
        Url::parse_with_params(uri, &self.params).unwrap().to_string()
    }
}
