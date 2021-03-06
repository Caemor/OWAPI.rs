use owapi::v3::stats::{Statistics, OverallStats};
use serde_json::from_str;


/// Solen from https://github.com/SunDwarf/OWAPI/blob/bf731c489ba18abceb281d753c72d14d75b07ad2/api.md#get-apiv3ubattletagstats
static TEST_STATS_OFFICIAL: &'static str = include_str!("../../test_data/stats_official.json");
static TEST_STATS_REAL_QUICKPLAY: &'static str = include_str!("../../test_data/stats_real_quick.json");
static TEST_STATS_REAL_COMPETITIVE: &'static str = include_str!("../../test_data/stats_real_comp.json");


#[test]
fn deserialisation_official() {
    assert_eq!(from_str::<Statistics>(TEST_STATS_OFFICIAL).unwrap(),
               Statistics {
                   overall_stats: OverallStats {
                       win_rate: 52.0,
                       level: 20,
                       prestige: 1,
                       avatar: "https://blzgdapipro-a.akamaihd.net/game/unlocks/0x0250000000000BBA.png".to_string(),
                       rank_image: None,
                       tier: None,
                       wins: 9,
                       games: 17,
                       comprank: 2395,
                       losses: 8,
                       ties: None,
                   },
                   game_stats: json!({
                       "objective_kills": 121.0,
                       "games_won": 9.0,
                       "kpd": 1.92,
                       "objective_kills_most_in_game": 26.0,
                       "time_spent_on_fire_most_in_game": 0.075,
                       "healing_done": 15798.0,
                       "defensive_assists": 20.0,
                       "offensive_assists": 4.0,
                       "final_blows_most_in_game": 22.0,
                       "objective_time": 0.37027777777777776,
                       "melee_final_blows": 3.0,
                       "medals": 37.0,
                       "cards": 4.0,
                       "multikill_best": 4.0,
                       "multikills": 4.0,
                       "defensive_assists_most_in_game": 11.0,
                       "offensive_assists_most_in_game": 2.0,
                       "melee_final_blow_most_in_game": 1.0,
                       "damage_done": 201576.0,
                       "medals_silver": 12.0,
                       "medals_gold": 12.0,
                       "healing_done_most_in_game": 2597.0,
                       "environmental_kills": 5.0,
                       "medals_bronze": 13.0,
                       "solo_kills": 29.0,
                       "time_spent_on_fire": 0.33999999999999997,
                       "eliminations_most_in_game": 44.0,
                       "final_blows": 152.0,
                       "time_played": 3.0,
                       "environmental_deaths": 6.0,
                       "solo_kills_most_in_game": 22.0,
                       "damage_done_most_in_game": 22230.0,
                       "games_played": 17.0,
                       "eliminations": 315.0,
                       "objective_time_most_in_game": 0.060000000000000005,
                       "deaths": 164.0,
                   })
                       .as_object()
                       .unwrap()
                       .clone()
                       .into_iter()
                       .collect(),
                   average_stats: json!({
                       "healing_done_avg": 929.0,
                       "eliminations_avg": 18.52,
                       "melee_final_blows_avg": 0.17,
                       "final_blows_avg": 8.94,
                       "defensive_assists_avg": 1.0,
                       "damage_done_avg": 11857.0,
                       "deaths_avg": 9.64,
                       "objective_time_avg": 0.021666666666666667,
                       "offensive_assists_avg": 0.0,
                       "solo_kills_avg": 1.7,
                       "time_spent_on_fire_avg": 0.02,
                       "objective_kills_avg": 7.11,
                   })
                       .as_object()
                       .unwrap()
                       .clone()
                       .into_iter()
                       .collect(),
                   competitive: true,
               });
}

#[test]
fn deserialisation_real_quickplay() {
    assert_eq!(from_str::<Statistics>(TEST_STATS_REAL_QUICKPLAY).unwrap(),
               Statistics {
                   overall_stats: OverallStats {
                       comprank: 2113,
                       games: 1513,
                       tier: Some("gold".to_string()),
                       losses: 793,
                       prestige: 2,
                       win_rate: 47.59,
                       avatar: "https://blzgdapipro-a.akamaihd.net/game/unlocks/0x02500000000008E9.png".to_string(),
                       level: 2,
                       rank_image: Some("https://blzgdapipro-a.akamaihd.net/game/playerlevelrewards/0x025000000000092C_Border.png".to_string()),
                       wins: 720,
                       ties: None,
                   },
                   game_stats: json!({
                       "defensive_assists": 642.0,
                       "offensive_assists_most_in_game": 14.0,
                       "eliminations_most_in_game": 44.0,
                       "healing_done": 817434.0,
                       "final_blows_most_in_game": 32.0,
                       "healing_done_most_in_game": 13642.0,
                       "kill_streak_best": 25.0,
                       "turrets_destroyed": 1692.0,
                       "solo_kills": 3940.0,
                       "teleporter_pads_destroyed": 103.0,
                       "objective_time_most_in_game": 0.05083333333333333,
                       "objective_time": 13.27777777777778,
                       "time_spent_on_fire": 16.31388888888889,
                       "objective_kills_most_in_game": 28.0,
                       "deaths": 10325.0,
                       "turret_destroyed_most_in_game": 1.0,
                       "medals_bronze": 1272.0,
                       "medals": 3710.0,
                       "multikill_best": 5.0,
                       "objective_kills": 6495.0,
                       "environmental_deaths": 184.0,
                       "recon_assists": 22.0,
                       "damage_done": 6967170.0,
                       "multikills": 130.0,
                       "defensive_assists_most_in_game": 21.0,
                       "solo_kills_most_in_game": 32.0,
                       "kpd": 1.7,
                       "cards": 365.0,
                       "damage_done_most_in_game": 17103.0,
                       "offensive_assists": 270.0,
                       "eliminations": 17577.0,
                       "games_won": 720.0,
                       "melee_final_blows": 68.0,
                       "environmental_kills": 81.0,
                       "medals_gold": 1184.0,
                       "medals_silver": 1254.0,
                       "final_blows": 9979.0,
                       "melee_final_blows_most_in_game": 3.0,
                       "recon_assists_most_in_game": 13.0,
                       "time_spent_on_fire_most_in_game": 0.1888888888888889,
                       "time_played": 194.0,
                   }).as_object().unwrap().clone().into_iter().collect(),
                   average_stats: json!({
                       "damage_done_avg": 4602.0,
                       "deaths_avg": 6.81,
                       "healing_done_avg": 540.0,
                       "objective_time_avg": 0.008611111111111113,
                       "time_spent_on_fire_avg": 0.010555555555555556,
                       "melee_final_blows_avg": 0.04,
                       "final_blows_avg": 6.59,
                       "eliminations_avg": 11.6,
                       "objective_kills_avg": 4.28,
                       "solo_kills_avg": 2.6,
                   }).as_object().unwrap().clone().into_iter().collect(),
                   competitive: false,
               });
}

#[test]
fn deserialisation_real_competitive() {
    assert_eq!(from_str::<Statistics>(TEST_STATS_REAL_COMPETITIVE).unwrap(),
               Statistics {
                   overall_stats: OverallStats {
                       comprank: 2113,
                       games: 10,
                       tier: Some("gold".to_string()),
                       losses: 5,
                       prestige: 2,
                       win_rate: 50.0,
                       avatar: "https://blzgdapipro-a.akamaihd.net/game/unlocks/0x02500000000008E9.png".to_string(),
                       level: 2,
                       rank_image: Some("https://blzgdapipro-a.akamaihd.net/game/playerlevelrewards/0x025000000000092C_Border.png".to_string()),
                       ties: Some(0),
                       wins: 5,
                   },
                   game_stats: json!({
                       "defensive_assists": 162.0,
                       "offensive_assists_most_in_game": 11.0,
                       "eliminations_most_in_game": 35.0,
                       "healing_done": 105157.0,
                       "final_blows_most_in_game": 5.0,
                       "healing_done_most_in_game": 17382.0,
                       "offensive_assists": 25.0,
                       "solo_kills": 5.0,
                       "medals": 23.0,
                       "objective_time_most_in_game": 0.07722222222222222,
                       "objective_time": 0.3913888888888889,
                       "time_spent_on_fire": 0.09166666666666666,
                       "objective_kills_most_in_game": 21.0,
                       "deaths": 101.0,
                       "kpd": 1.32,
                       "games_lost": 5.0,
                       "medals_bronze": 5.0,
                       "games_played": 10.0,
                       "objective_kills": 81.0,
                       "damage_done": 68761.0,
                       "defensive_assists_most_in_game": 30.0,
                       "solo_kills_most_in_game": 5.0,
                       "cards": 9.0,
                       "damage_done_most_in_game": 11573.0,
                       "environmental_kill": 1.0,
                       "eliminations": 133.0,
                       "games_won": 5.0,
                       "final_blows": 26.0,
                       "medals_gold": 14.0,
                       "medals_silver": 4.0,
                       "environmental_death": 1.0,
                       "turrets_destroyed": 6.0,
                       "time_spent_on_fire_most_in_game": 0.044722222222222226,
                       "time_played": 2.0,
                   }).as_object().unwrap().clone().into_iter().collect(),
                   average_stats: json!({
                       "damage_done_avg": 6876.0,
                       "deaths_avg": 10.1,
                       "healing_done_avg": 10516.0,
                       "objective_time_avg": 0.03888888888888889,
                       "time_spent_on_fire_avg": 0.009166666666666668,
                       "eliminations_avg": 13.3,
                       "final_blows_avg": 2.6,
                       "objective_kills_avg": 8.1,
                       "solo_kills_avg": 0.5,
                   }).as_object().unwrap().clone().into_iter().collect(),
                   competitive: true,
               });
}
