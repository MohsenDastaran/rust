#[derive(Debug)]
struct Song {
    title: String,
    release_year: u64,
    duration_secs: u64,
}
impl Song {
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
        println!("Doubled Duration Seconds: {:?}", self.duration_secs);
    }
}
fn main() {
    let mut song = Song {
        title: "Creep".into(),
        release_year: 2020,
        duration_secs: 200,
    };
    song.double_duration();
    song.display_song_info();
}
