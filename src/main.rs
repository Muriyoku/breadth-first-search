use std::{collections::{HashMap, HashSet, VecDeque}};

#[derive(Clone)]
struct GamerProfile {
    name: String,
    fav_game: String,
    fav_ctgr: String, // ctgr == category
}

fn main() {
    let mut friends: HashMap<String, Vec<GamerProfile>> = HashMap::new();

    friends.insert(
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
    friends.insert(
        "jhon".to_string(),
        vec![GamerProfile {
            name: "mel".to_string(),
            fav_ctgr: "Casual".to_string(),
            fav_game: "Standerw Valley".to_string(),
        }],
    );
    friends.insert(
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
    friends.insert(
        "cleitin".to_string(),
        vec![GamerProfile {
            name: "Mel".to_string(),
            fav_ctgr: "Casual".to_string(),
            fav_game: "Standerw Valley".to_string(),
        }],
    );

    search("you".to_string(), friends);
}

fn search(name: String, list: HashMap<String, Vec<GamerProfile>>) {
    let mut queue: VecDeque<GamerProfile> = VecDeque::new();

    for f in list.get(&name).unwrap() {
        queue.push_back(f.clone());
    }

    let mut searched_people: HashSet<String> = HashSet::new();

    while !queue.is_empty() {
        let friend: GamerProfile = queue.pop_front().unwrap();

        if !searched_people.contains(&friend.name) {
            if friend.fav_game.to_lowercase() == "tales of berseria" && friend.fav_ctgr == "JRPG" {
                println!("Oh, you like Tales of Berseria, {}!", friend.name);

            } else {
                println!("current name: {}", friend.name);
                let friend_list = list.get(&friend.name);

                match friend_list {
                    Some(friends) => {
                        for f in friends {
                            queue.push_back(f.clone());
                        }
                    }
                    None => continue
                }

            }
            searched_people.insert(friend.name);
        } else {
            continue
        }
    }

}