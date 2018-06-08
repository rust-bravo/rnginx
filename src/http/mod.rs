pub enum HTTP {
    OK { code: u32, desc: &'static str },
    NotFound { code: u32, desc: &'static str },
}

static PROTOCOL: &'static str = "HTTP/1.1";

impl HTTP {
    fn value(state: HTTP) -> &'static str {
        match state {
            OK => "200 OK\r\n\r\n",
            NotFound => "404 OK\r\n\r\n",
        }
    }

    //fn message(state: HTTP) -> &'static str {
    //    let mut protocol = format!("{}", PROTOCOL);
    //    format!("{} {} {}", protocol, state.desc, state.code)
    //}
}
