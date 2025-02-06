// This is your observer/subscriber


trait Channel{
    fn update(&mut self, news: &str);
}

#[derive(Debug, Clone)]
struct NewsChannel {
    news: String
}

impl NewsChannel {
    fn new() -> NewsChannel {
        NewsChannel {
            news: "".to_string()
            // news: &<str
        }
    }
    fn get_news(&self) -> String {
        self.news.clone()
    }
    fn set_news(&mut self, news: String) {
        self.news = news;
    }
}

impl Channel for NewsChannel {
    fn update(&mut self, news: &str) {
        self.set_news(news.to_string());
        println!("Set news: {}", news);
    }
}

// Observable
struct NewsAgency<'a> {
    news: String,
    channels: Vec<&'a mut dyn Channel>
}

impl<'a> NewsAgency<'a> {
    fn new() -> Self {
        Self {
            news: "".to_string(),
            channels: Vec::new()
        }
    }

    // attach
    fn add_observer(&mut self, channel: &'a mut dyn Channel) {
        self.channels.push(channel);
    }
    
    // detach
    fn remove_observer(&mut self, channel: &'a dyn Channel) {
        self.channels.retain(|o| !std::ptr::eq(*o, channel));
    }

    // notify
    fn notify(&mut self) {
        for c in self.channels.iter_mut() {
            c.update(&mut self.news);
        }
    }
    fn set_news(&mut self, news: String) {
        self.news = news;
        self.notify();
    }
}

fn main() {

    let mut observable = NewsAgency::new();
    let mut observer = NewsChannel::new();

    observable.add_observer(&mut observer); // adding subscriber in the list of channels

    observable.set_news("Terror".to_string());
    println!("{:?}", observer.get_news());

}
