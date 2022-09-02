use hyper_tls::HttpsConnector;
use hyper::{client::{Client, HttpConnector}, Body, Request, Method, StatusCode};
use dotenv::dotenv;
use std::{env, time::{SystemTime, UNIX_EPOCH, Duration}, thread, str, fmt};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let cookie = env::var("COOKIE").unwrap();

    let client = Client::builder().build::<_, Body>(HttpsConnector::new());

    let mut x = DateRblxDescriptionChanger::new(client, &cookie, 1677196800000, TimeScale::Minutes, Some("My birthday".to_owned()));

    let sleep = Duration::new(5, 0);

    loop {
        thread::sleep(sleep);
        println!("{}", match x.step().await {
            Ok(status) => format!("No error; code: {}", status),
            Err(err) => format!("Error; {}", err)
        });
    }
}

pub struct RblxDescriptionChanger<'a> {
    cookie: &'a str,
    token: Option<String>,
    client: Client<HttpsConnector<HttpConnector>>
}
impl<'a> RblxDescriptionChanger<'a> {
    pub fn new(client: Client<HttpsConnector<HttpConnector>>, cookie: &'a str) -> Self {
        Self {
            cookie,
            token: None,
            client
        }
    }
    pub async fn step(&mut self, txt: String) -> Result<StatusCode, hyper::Error> {
        self.client.request(Request::builder()
            .method(Method::POST)
            .uri("https://accountinformation.roblox.com/v1/description")
            .header("Content-Type", "application/json")
            .header("Cookie", format!(".ROBLOSECURITY={}", self.cookie))
            .header("X-CSRF-Token", self.token.as_ref().unwrap_or(&"".to_string()))
            .body(Body::from(format!(r#"{{"description":"{}"}}"#, txt))).unwrap())
                .await
                .map(|resp| {
                    let status = resp.status();
                    (status.as_u16()==403).then(|| self.token = resp.headers().get("x-csrf-token").map(|v| str::from_utf8(v.as_bytes()).unwrap().to_owned()));
                    status
                })
    }
}

enum TimeScale {
    Seconds,
    Minutes,
    Hours,
    Days
}
impl TimeScale {
    fn to_u128(&self) -> u128 {
        match self {
            TimeScale::Seconds => 1_000,
            TimeScale::Minutes => 60_000,
            TimeScale::Hours => 3_600_000,
            TimeScale::Days => 86_400_000
        }
    }
}
impl fmt::Display for TimeScale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            TimeScale::Seconds => "Seconds",
            TimeScale::Minutes => "Minutes",
            TimeScale::Hours => "Hours",
            TimeScale::Days => "Days"
        })
    }
}


struct DateRblxDescriptionChanger<'a> {
    date: u128,
    time_scale: TimeScale,
    date_alias: String,
    changer: RblxDescriptionChanger<'a>
}
impl<'a> DateRblxDescriptionChanger<'a> {
    fn new(
        client: Client<HttpsConnector<HttpConnector>>,
        cookie: &'a str, 
        date: u128,
        time_scale: TimeScale,
        date_alias: Option<String>
    ) -> Self {
        Self {
            date,
            time_scale,
            date_alias: date_alias.unwrap_or(date.to_string()),
            changer: RblxDescriptionChanger::new(client, cookie)
        }
    }
    async fn step(&mut self) -> Result<StatusCode, hyper::Error> {
        self.changer.step(format!("{} until {}:\n{}", self.time_scale, self.date_alias, (self.date-SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap().as_millis())/self.time_scale.to_u128()))
        .await
    }
}