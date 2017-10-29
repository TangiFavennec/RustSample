#[macro_use]
extern crate serde_derive;

extern crate rand;
extern crate rmp_serde;
extern crate serde_json;
extern crate serde;
extern crate clap;

use clap::{App, Arg, SubCommand};
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io;
use std::num;
use std::io::Read;
use std::io::prelude::*;
use std::fmt;
use std::error::Error;
use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Match {
	match_id:i32,
	receiving_team : String,
	receiving_team_count : i32,
	coming_team : String,
	coming_team_count : i32,
	is_played : bool
}

#[derive(Eq)]
struct Stats {
	name : String,
	scored : i32,
	conceded : i32,
	win : i32,
	tie : i32,
	loss : i32,
}

impl Stats{
	fn get_points(&self) -> i32{
		&self.win * 3 + self.tie 
	}
	
	fn get_score_diff(&self) -> i32{
		self.scored - self.conceded
	}
}

impl fmt::Display for Stats{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
		write!(f, "{} | Points : {}, Wins : {}, Ties : {}, Losses : {} , Scored : {}, Conceded : {}, Diff : {}",self.name, self.get_points(), self.win,self.tie,self.loss,self.scored,self.conceded,self.get_score_diff())
	}	
}

 impl Ord for Stats{
     // reverse comparison wow
	 fn cmp(&self, other: &Stats) -> Ordering{
	     let points_diff = &self.get_points() - other.get_points();
		 if points_diff > 0
		 {
			 Ordering::Less
		 }
		 else if points_diff < 0
		 {
			 Ordering::Greater
		 }
		 else{
			 let diff_average = self.scored - self.conceded - other.scored + other.conceded;
			 if diff_average > 0
			 {
				 Ordering::Less
			 }
			 else if diff_average < 0 {
				 Ordering::Greater
			 }
			 else{
				 let diff_scores = self.scored - other.scored;
				 if diff_scores > 0
				 {
					 Ordering::Less
				 }
				 else if diff_scores < 0
				 {
					 Ordering::Greater
				 }
				 else
				 {
					 Ordering::Equal
				 }
			 }
		 }
	 }
}

impl PartialOrd for Stats{
	fn partial_cmp(&self, other: &Stats) -> Option<Ordering>{
		Some(self.cmp(other))
	}
}

impl PartialEq for Stats {
    fn eq(&self, other: &Stats) -> bool {
        self.get_points() == other.get_points() && self.scored == other.scored && self.conceded == other.conceded
    }
}
// Championship structure

#[derive(Serialize, Deserialize, Debug)]
struct Championship {
	teams: Vec<String>,
	matches:Vec<Match>
}

impl Championship 
{
    fn setResult(&mut self, target_id : i32, receiving_team_count : i32, coming_team_count: i32)
    {
        for m in self.matches.iter_mut() {
		     if m.match_id == target_id
			 {
			     m.receiving_team_count = receiving_team_count;
				 m.coming_team_count = coming_team_count;
				 m.is_played = true;
		     }
		}
    }

    fn show_calendar(&self)
    {
	    println!("Here are the championships results:");
        for x in &self.matches {
			let mut score = "Not Played Yet".to_string();
			if x.is_played
			{
			    score = format!(" {} - {}", x.receiving_team_count, x.coming_team_count);
			}
		
			println!("Match Id : {} | {} - {} : {}", x.match_id, x.receiving_team, x.coming_team, score);
        }
    }

    fn show_teams(&self)
    {	
		let mut number = 1;
	
        for t in &self.teams {
		    println!("{}", t);
			number += 1;
		}
    }

    fn show_rankings(&self)
    {
        println!("Here are the championships rankings:");
		
		let mut rankings = HashMap::new();
		for t in &self.teams{
			let stat =  Stats {name:t.to_string(),scored:0,conceded:0,win:0,tie:0,loss:0};
			rankings.insert(t,stat);
		}
				
		for m in &self.matches {		
					
			let mut receiving_team_stat = Stats {name:m.receiving_team.to_string(),scored:0,conceded:0,win:0,tie:0,loss:0};
			let mut coming_team_stat = Stats {name:m.coming_team.to_string(),scored:0,conceded:0,win:0,tie:0,loss:0};
			if !m.is_played
			{
				continue;
			}
						
			let result = m.receiving_team_count - m.coming_team_count;
			
			// Updates match result counts
			if result > 0
			{
			    receiving_team_stat.win +=1;
			}
			else if result == 0
			{
			    receiving_team_stat.tie +=1;
			    coming_team_stat.tie +=1;
			}
			else{
			    coming_team_stat.win +=1;
			}
			
			// Updates scored and conceded goals counts and then updates stat or create stat if it does not exist yet		
			if let Some(stat) = rankings.get_mut(&m.receiving_team) {
				 receiving_team_stat.scored = stat.scored + m.receiving_team_count;
				 receiving_team_stat.conceded = stat.conceded + m.coming_team_count;
				 *stat = receiving_team_stat;
			}
			
			if let Some(stat) = rankings.get_mut(&m.coming_team) {
				coming_team_stat.scored = stat.scored + m.coming_team_count;
				coming_team_stat.conceded = stat.conceded + m.receiving_team_count;
				*stat = coming_team_stat;
				
			}
		}
		
		let mut display_data = Vec::new();
		for (_key,value) in &rankings{
			display_data.push(value);
		}
				
		let mut rank = 1;
				
		display_data.sort_by(|a,b| a.cmp(b));
		
		for stat in &display_data {
		    println!( "Rank {} : {}", rank, stat);
			rank+=1;
		}
		
    }
}

fn get_teams_string_from_file() -> Result<String, io::Error> {
    let f = File::open("teams.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn create_championship()
{
	println!("Championship Creation processing");
    let result = get_teams_string_from_file();
	
	let result = match result{
		Ok(r) => r,
		Err(e) => {
			panic!("Error while loading teams file: {:?}",e)
		},
	};
	
	let teams_string:Vec<&str> = result.split('|').collect();
	
	let mut matches = Vec::new();
	
	let mut match_id_counter = 1;
	
	for t1 in &teams_string {
		for t2 in &teams_string {
			if t1 != t2
			{
				let m = Match
				{
					match_id : match_id_counter,
					receiving_team : t1.to_string(),
					receiving_team_count : 0,
					coming_team : t2.to_string(),
					coming_team_count : 0	,
					is_played : false					
				};
				matches.push(m);
				match_id_counter+=1;
			}
		}
	}
	
	let mut teams  = Vec::new();
	for s in teams_string{
		teams.push(s.to_string());
	}
	
	teams.sort();
	
	let championship = Championship{
		teams,
		matches
	};
	
	serialize_and_save_championship(&championship).expect("Championship save failed");
		
	println!("ChampionShip Creation has ended successfully");	
}

fn get_championship() -> Championship{

	let championship_json = deserialize_championship().expect("Deserialization failed");
	
	let res = serde_json::from_str(&championship_json).expect("Championship serialization failed! ");
	
	res
}

fn update_championship(mid: i32, st1 : i32, st2: i32) {
	
	let mut championship = get_championship();
	
	championship.setResult(mid,st1,st2);
	
	serialize_and_save_championship(&championship).expect("Championship save failed");
}

fn show_teams() {
	let championship_json = deserialize_championship().expect("Deserialization failed");
	
	let championship : Championship = serde_json::from_str(&championship_json).expect("Championship serialization failed! ");
	
	championship.show_teams();
}

fn show_rankings() {
	let championship_json = deserialize_championship().expect("Deserialization failed");
	
	let championship : Championship = serde_json::from_str(&championship_json).expect("Championship serialization failed! ");
	
	championship.show_rankings();
}

fn show_calendar() {
	let championship_json = deserialize_championship().expect("Deserialization failed");
	
	let championship : Championship = serde_json::from_str(&championship_json).expect("Championship serialization failed! ");
	
	championship.show_calendar();
}

fn serialize_and_save_championship(championship : &Championship) -> Result<(), io::Error> {
    
    // Serialize it to a JSON string.
    let csstring = serde_json::to_string(championship).expect("Championship serialization failed! ");
	
	fs::remove_file("championship.txt").expect("Unable to remove file");
	
	File::create("championship.txt").expect("Unable to create file");
	
	let mut f = OpenOptions::new().write(true).open("championship.txt").expect("Unable to open file");
	
	//print!("{}",csstring);
	
	f.write_all(csstring.as_bytes()).expect("unable to write data");	
	
    Ok(())
}

fn deserialize_championship() -> Result<String, io::Error> {
    let mut f = OpenOptions::new().read(true).open("championship.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
		
    Ok(s)
}


fn main() {
	let app = App::new("Champ")
					       .version("1.0")
                           .author("Tangi F. <tangui.favennec@gmail.com>")
                           .about("Manage football championship")
                           .subcommand(SubCommand::with_name("create")
                                      .about("creates championship from teams enumerated in teams.txt file"))							
                           .subcommand(SubCommand::with_name("teams")
                                      .about("shows teams list"))
                           .subcommand(SubCommand::with_name("calendar")
                                      .about("shows championship calendar"))
                           .subcommand(SubCommand::with_name("rankings")
                                      .about("show rankings of the championship"))
                           .subcommand(SubCommand::with_name("update")
                                      .about("updates one result of the championship")
									  .arg(Arg::with_name("m")
									  .short("m")
									  .value_name("m")
									  .help("match id")
									  .validator(|v| {
										   v.parse::<i32>()
										  .map_err(|e| String::from(e.description()))
										  .and_then(|v| if v > 0 { Ok(v) }
													else { Err(String::from("match id cannot be negative nor zero")) }
												   )
										  .map(|_| ())
										  }))
									  .arg(Arg::with_name("r")
									  .short("r")
									  .value_name("r")
									  .help("receiving team score")
									  .validator(|v| {
										   v.parse::<i32>()
										  .map_err(|e| String::from(e.description()))
										  .and_then(|v| if v >= 0 { Ok(v) }
													else { Err(String::from("score cannot be negative")) }
												   )
										  .map(|_| ())
									   }))
									  .arg(Arg::with_name("c")
									  .short("c")
									  .value_name("c")
									  .help("coming team score")
									  .validator(|v| {
										   v.parse::<i32>()
										  .map_err(|e| String::from(e.description()))
										  .and_then(|v| if v >= 0 { Ok(v) }
													else { Err(String::from("score cannot be negative")) }
												   )
										  .map(|_| ())
									   }))
									   )
                           .get_matches();
						   
	if app.subcommand_matches("create").is_some() {
        create_championship();
	}
	else if app.subcommand_matches("teams").is_some(){
		show_teams();
    } 
	else if app.subcommand_matches("calendar").is_some(){
		show_calendar();
    }
	else if app.subcommand_matches("rankings").is_some(){
		show_rankings();
    }
	else if app.subcommand_matches("update").is_some(){
	
		let val1 = app.value_of("m");
		print!("{:?}",val1);
		update_championship(app.value_of("m").unwrap().parse().unwrap(),
							1,
							1);
    }
	else
	{
		print!("Command should be user with argument")
	}
    //create_championship();
}