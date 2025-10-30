use std::{
    collections::{HashMap, HashSet, VecDeque}, process
};

#[derive(Clone)]
struct GamerProfile {
    name: String,
    fav_game: String,
    fav_ctgr: String, // ctgr == category
}

fn main() {
    let mut netword: HashMap<String, Vec<GamerProfile>> = HashMap::new();

    netword.insert(
        "you".to_string(),
        vec![
            GamerProfile {
                name: "jhon".to_string(),
                fav_ctgr: "RPG".to_string(),
                fav_game: "Final Fantasy XIII".to_string(),
            },
            GamerProfile {
                name: "jeff".to_string(),
                fav_ctgr: "FPS".to_string(),
                fav_game: "Call of Duty II".to_string(),
            },
        ],
    );
    netword.insert(
        "jhon".to_string(),
        vec![GamerProfile {
            name: "mel".to_string(),
            fav_ctgr: "Casual".to_string(),
            fav_game: "Standerw Valley".to_string(),
        }],
    );
    netword.insert(
        "jeff".to_string(),
        vec![
            GamerProfile {
                name: "cleitin".to_string(),
                fav_ctgr: "JRPG".to_string(),
                fav_game: "Tales of Berseria".to_string(),
            },
            GamerProfile {
                name: "jhon".to_string(),
                fav_ctgr: "RPG".to_string(),
                fav_game: "Final Fantasy XIII".to_string(),
            },
        ],
    );
    netword.insert(
        "cleitin".to_string(),
        vec![GamerProfile {
            name: "mel".to_string(),
            fav_ctgr: "Casual".to_string(),
            fav_game: "Stardew Valley".to_string(),
        }],
    );

    let _: Result<GamerProfile, String> = search("you".to_string(), netword).or_else(|e|  {
        eprintln!("{}", e);
        process::exit(1);
    });
}

fn search(start: String, network: HashMap<String, Vec<GamerProfile>>) -> Result<GamerProfile, String> {
    let mut queue = VecDeque::new();
    let mut searched= HashSet::new();

    if let Some(friends) = network.get(&start) {
        queue.extend(friends.iter().cloned());
    } else {
        return Err("User not found!".to_string());
    }

    while let Some(gamer) = queue.pop_front(){
        if searched.contains(&gamer.name) {
            continue;
        }

        if gamer.fav_game.eq_ignore_ascii_case("Final Fantasy XIII") {
            println!("Oh, you like Final Fantasy XIII, {}!", gamer.name);
            return Ok(gamer);
        }

        if let Some(friends) = network.get(&gamer.name) {
            queue.extend(friends.iter().cloned());
        }

        searched.insert(gamer.name);
    }

    return Err("No one in the network corresponds to your search".to_string());
}
