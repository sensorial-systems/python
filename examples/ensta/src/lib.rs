pub struct Client {
    client: pyo3::PyObject
}

impl Client {
    pub fn new() -> Self {
        python310::Python::default().setup();

        let client = pyo3::Python::with_gil(|py| {
            let instagrapi = py.import("ensta").unwrap();
            instagrapi.getattr("Guest").unwrap().call0().unwrap().into()
        });
        Self { client }
    }

    pub fn biography_of(&self, user_name: &str) -> String {
        pyo3::Python::with_gil(|py| {
            let user_info = self.client.call_method1(py, "profile", (user_name,)).unwrap();
            user_info.getattr(py, "biography").unwrap().extract(py).unwrap()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    #[test]
    fn test() {
        let client = Client::new();
        assert_eq!(client.biography_of("notdanilo"), "futuriste teqnophobique");
    }
}
