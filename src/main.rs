use std::process::Command;

struct Location <'a> {
    name:&'a str,
    lat:f64,
    long:f64,
}

impl <'a> Location <'a> {
    fn distance (&self, other:&Location) -> f64 {
        ((self.lat-other.lat).powi(2)+(self.long-other.long).powi(2)).sqrt()
    }

    fn closest_place <'b> (&'b self, list:&'b [Location]) -> &Location {
        let mut minimal_distance:f64 = 1000000.0;
        let mut close_location:&Location=&self;
        for place in list {
            if self.distance(place) < minimal_distance {
                close_location = place;
                minimal_distance = self.distance(place)
            }
        }
        close_location
    }
}

fn main() {
   let my_location = Location{
    name:"Me",
    lat:get_geodata()[0],
    long:get_geodata()[1],
    };

    let servery_list:[Location;5]=[
    Location {name: "Seibel", lat: 29.716017643539782, long: -95.39847203418154,},
    Location {name: "Baker", lat: 29.717108498444823, long: -95.3993597608468,},
    Location {name: "South", lat: 29.71552005664459, long: -95.40104077516328,},
    Location {name: "North", lat: 29.721861820986337, long: -95.39670726061257,},
    Location {name: "Seibel", lat: 29.721119919804234, long: -95.39838953823505,},
    ];

    println!("{}",(my_location.closest_place(&servery_list)).name)
}

fn get_geodata() -> [f64;2] {
    //set directory path
    let script_path = ".\\src\\geodata.ps1";

    //get geodata
    let output = Command::new("powershell.exe")
        .arg("-ExecutionPolicy")
        .arg("Bypass") // Change this to your preferred execution policy if necessary
        .arg("-File")
        .arg(script_path)
        .output()
        .expect("Failed to execute PowerShell script");

    let ps_geo_output = String::from_utf8(output.stdout).expect("Error parsing stdout");

    let mut deleting_ps_geo_out=ps_geo_output.clone();

    //cut out beginging part
    let mut i:u32 = 0;
    while i<74 {
       deleting_ps_geo_out.remove(0);
       i += 1;
    }

    //turn into numbers
    let mut split = deleting_ps_geo_out.split_ascii_whitespace();
    let lat_str = (split.next()).unwrap();
    let long_str = (split.next()).unwrap();

    let lat=(*lat_str).trim().parse::<f64>().unwrap();
    let long=(*long_str).trim().parse::<f64>().unwrap();

    //create output array
    [lat, long]
}