#[derive(Debug)]
struct Song {
    title: String,
    release_year: u64,
    release_secs: u64,
}
impl Song {
    fn display_song_info(self) {
        // we have options for self:
        //  passing immutably: &self (self parameter takes ownership of the instance)
        //  passing mutably: &mut self (self parameter takes ownership of the instance and allows mutation)
        // immutable reference: &self (self parameter takes a reference to the instance)

        println!("Song: {}", self.title);
        println!("Release Year: {}", self.release_year);
        println!("Release Seconds: {}", self.release_secs);
    }
}
fn main() {
    let song = Song {
        title: "Creep".into(),
        release_year: 2020,
        release_secs: 200,
    };
    song.display_song_info();

    Song {
        title: "Crsdfsdfeep".into(),
        release_year: 244020,
        release_secs: 24545400,
    }
    .display_song_info();
}
