use std::fmt::Display;

use crate::datetime::AxDateTime;

mod datetime;

fn main() {
    struct AxDateTimeDisplay {
        date_time: AxDateTime
    }

    impl Display for AxDateTimeDisplay {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(format!(
                "{}-{:02}-{:02} {:02}:{:02}:{:02}", 
                self.date_time.year, 
                self.date_time.month, 
                self.date_time.day, 
                self.date_time.hour, 
                self.date_time.minute, 
                self.date_time.second
            ).as_str())
        }
    }
    
    let date = AxDateTime::now(3);
    println!("AxDateTime {:?}", date);

    let date_time = AxDateTime::from_timestamp(0, 0).unwrap();

    println!("{}", AxDateTimeDisplay{date_time});
}
