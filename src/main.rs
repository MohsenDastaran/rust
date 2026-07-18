#[derive(Debug)]
struct Song {
    title: String,
    release_year: u64,
    duration_secs: u64,
}
impl Song {
    fn new(title: String, release_year: u64, duration_secs: u64) -> Self {
        Self {
            title,
            release_year,
            duration_secs,
        }
    }
    fn display_song_info(&self) {
        // we have options for self:
        // self: takes ownership of the instance
        //  passing immutably: &self (self parameter takes ownership of the instance)
        //  passing mutably: &mut self (self parameter takes ownership of the instance and allows mutation)
        // immutable reference: &self (self parameter takes a reference to the instance)

        println!("Song: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Duration Seconds: {}", self.duration_secs);
    }
    fn double_duration(&mut self) {
        self.duration_secs *= 2;
        self.display_song_info();
    }

    fn is_longer(&self, other: &Song) -> bool {
        self.duration_secs > other.duration_secs
    }
}
fn main() {
    // let mut song = Song {
    //     title: "Creep".into(),
    //     release_year: 2020,
    //     duration_secs: 200,
    // };

    // using assosciated function to create a new instance of Song
    let mut song = Song::new("Creep".into(), 2020, 200);

    song.double_duration();

    let another_song = Song {
        title: "Some Other Song".into(),
        release_year: 2010,
        duration_secs: 20,
    };
    println!(
        "Is {} longer than {}? {}",
        song.title,
        another_song.title,
        song.is_longer(&another_song)
    );
}
