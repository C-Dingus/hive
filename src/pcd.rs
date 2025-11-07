use std::fs::File;
use std::io::{Read, Write};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Point{
	x:i16,
	y:i16,
	z:i16
}

impl Point{
	pub fn debug_empty() -> Point{
		return Point {x:0, y:0, z:0}
	}
}

#[derive(Clone, Copy, Debug)]
struct Tile{
	normal_points:[Point;4],
	corner:Option<[Point;4]>
}


impl Tile {
	pub fn debug_empty() -> Tile{
		return Tile{normal_points:[Point::debug_empty(),Point::debug_empty(),Point::debug_empty(),Point::debug_empty()], corner:None}
	}
}

#[derive(Clone, Copy)]
pub struct Chunk{
	tiles:[Tile;100]
}

impl Chunk{
	pub fn debug_empty() -> Chunk{
		let mut hold:Vec<Tile> = Vec::new();
		for _ in 0..100{
			hold.push(Tile::debug_empty());
		}

		return Chunk { tiles: hold.try_into().unwrap() }
	}
    
	pub fn write(self, name:&str) -> Result<(), String>{
    	//ERR
		let mut file = File::create(name).unwrap();
		for tile in self.tiles{
    		let mut res:Vec<Result<(),_>> = Vec::new();
    		for point in tile.normal_points{
				res.push(file.write_all(&point.x.to_le_bytes()));
				res.push(file.write_all(&point.y.to_le_bytes()));
				res.push(file.write_all(&point.z.to_le_bytes()));
    		}
    		if tile.corner != None{
        		res.push(file.write_all(&1_i16.to_le_bytes()));
				for point in tile.corner.unwrap(){
					res.push(file.write_all(&point.x.to_le_bytes()));
					res.push(file.write_all(&point.y.to_le_bytes()));
					res.push(file.write_all(&point.z.to_le_bytes()));
				}
    		}else{
        		res.push(file.write_all(&0_i16.to_le_bytes()));
    		}

    		for r in res{
				if let Err(_) = r{
					return Err(String::from("failed to write save file being aborted"))
				}
    		}
		}
		Ok(())
	}

	pub fn read(name:&str){
	    let mut inp = File::open(name).unwrap();
    	let mut buf = [0u8; 2];               // 4 bytes for a u32
    	let mut numbers = Vec::new();

    	while inp.read_exact(&mut buf).is_ok() {
	        let n = i16::from_le_bytes(buf);
    	    numbers.push(n);
    	}


    	println!("{:?}", numbers.len());

	}
}
