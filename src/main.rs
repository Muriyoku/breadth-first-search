use std::{
    collections::{HashMap, HashSet, VecDeque},
    process::exit,
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
            fav_game: "Standerw Valley".to_string(),
        }],
    );

    search("you".to_string(), netword);
}

fn search(name: String, list: HashMap<String, Vec<GamerProfile>>) -> bool {
    let mut searched_people: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<GamerProfile> = VecDeque::new();
    let gamer: Option<&Vec<GamerProfile>> = list.get(&name);

    match gamer {
        Some(g) => {
            for f in g {
                queue.push_back(f.clone())
            }
        }
        None => exit(1),
    }

    while !queue.is_empty() {
        let gamer_opt: Option<GamerProfile> = queue.pop_front();
        let gamer = match gamer_opt {
            Some(g) => g,
            None => return false,
        };

        if !searched_people.contains(&gamer.name) {
            if gamer.fav_game.to_lowercase() == "Final Fantasy XIII".to_lowercase() {
                println!("Oh, you like Final Fantasy XIII, {}!", gamer.name);
                return true;
            } else {
                let friends_list: Option<&Vec<GamerProfile>> = list.get(&gamer.name);

                match friends_list {
                    Some(friend) => {
                        for f in friend {
                            queue.push_back(f.clone());
                        }
                    }
                    None => {
                        searched_people.insert(gamer.name);
                        continue;
                    }
                }
            }

            searched_people.insert(gamer.name);
        } else {
            continue;
        }
    }

    return false;
}
