struct Queen{
	workers:Vec<Rover>
}

struct RoverData{
	x:f32,
	y:f32,
}

enum Rover{
	Manipulation {target:str},
	Transport {cargo:str},
	//Support
}
