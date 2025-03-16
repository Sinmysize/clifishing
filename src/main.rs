use std::{thread, time::Duration};
use rand::{self, distr::{weighted::WeightedIndex, Distribution}, rng, Rng};

fn clear() {
    print!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn main() {
    loop {
        clear();
        let ui =
        r"
           |\                     
           | \
           |  \          
           |___\          
       ____|____            
_______\_______/_____________________________
        ";

        println!("Fishing...\n\n {}", ui);

        let rarity = [("\x1b[2mCommon\x1b[0m", 8), ("\x1b[32mUncommon\x1b[0m", 5), ("\x1b[35mRare\x1b[0m", 3), ("\x1b[33mLegendary\x1b[0m", 1)];
        let dist = WeightedIndex::new(rarity.iter().map(|selection| selection.1)).unwrap();

        let rand_duration = rand::rng().random_range(6..11);
        thread::sleep(Duration::from_secs(rand_duration));

        let fish_rarity = rarity[dist.sample(&mut rng())].0;

        let fish: &str = 
        r"
          _
        ><_>
        ";

        println!("You caught a {} Fish!\n{}", fish_rarity, fish);

        thread::sleep(Duration::from_secs(3));
    }
}
