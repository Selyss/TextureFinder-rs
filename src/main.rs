use clap::Parser;
use csv::ReaderBuilder;
use std::fs::File;
use std::path::Path;

#[derive(Parser)]
struct Cli {
    xmin: i64,
    xmax: i64,
    zmin: i64,
    zmax: i64,
    ymin: i64,
    ymax: i64,
    #[arg(long, action = clap::ArgAction::SetTrue)]
    use_all_rotations: bool,
    formation_file: String,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Side {
    TOP,
    BOTTOM,
    WEST,
    EAST,
    SOUTH,
    NORTH,
}

pub struct BlockFace {
    x: i64,
    y: i64,
    z: i64,
    side: Side,
    rotation: i32,
}

fn get_coordinate_random(x: i128, y: i128, z: i128) -> i128 {
    let mut i: i128 = (x * 3129871) as i128 ^ z * 116129781 ^ y;
    i = i * i * 42317861 + i * 11;
    // println!("Random Coordinate {}", i);
    return i;
}

fn get_texture_type(x: i128, y: i128, z: i128) -> i128 {
    let texture_type = ((get_coordinate_random(x, y, z) >> 16) % 16).abs();
    // println!("Texture Type: {}", textureType);
    return texture_type;
}

fn compatible_rotation(generated_type: i128, bface: &BlockFace) -> bool {
    if generated_type == 0 {
        return bface.rotation == 3;
    }
    if generated_type == 1 {
        return (bface.rotation == 3 && (bface.side == Side::TOP || bface.side == Side::SOUTH))
            || (bface.rotation == 2 && (bface.side == Side::WEST))
            || (bface.rotation == 1 && (bface.side == Side::BOTTOM || bface.side == Side::NORTH))
            || (bface.rotation == 0 && (bface.side == Side::EAST));
    }
    if generated_type == 2 {
        return (bface.rotation == 3 && (bface.side == Side::TOP || bface.side == Side::BOTTOM))
            || (bface.rotation == 1 && (bface.side != Side::TOP && bface.side != Side::BOTTOM));
    }
    if generated_type == 3 {
        return (bface.rotation == 3 && (bface.side == Side::BOTTOM || bface.side == Side::SOUTH))
            || (bface.rotation == 2 && (bface.side == Side::EAST))
            || (bface.rotation == 1 && (bface.side == Side::TOP || bface.side == Side::NORTH))
            || (bface.rotation == 0 && (bface.side == Side::WEST));
    }
    if generated_type == 4 {
        return (bface.rotation == 3 && (bface.side != Side::TOP && bface.side != Side::BOTTOM))
            || (bface.rotation == 2 && (bface.side == Side::BOTTOM))
            || (bface.rotation == 0 && (bface.side == Side::TOP));
    }
    if generated_type == 5 {
        return (bface.rotation == 3 && (bface.side == Side::WEST))
            || (bface.rotation == 2 && (bface.side == Side::NORTH))
            || (bface.rotation == 1 && (bface.side == Side::EAST))
            || (bface.rotation == 0
                && (bface.side == Side::TOP
                    || bface.side == Side::BOTTOM
                    || bface.side == Side::SOUTH));
    }
    if generated_type == 6 {
        return (bface.rotation == 1 && (bface.side != Side::TOP && bface.side != Side::BOTTOM))
            || (bface.rotation == 2 && (bface.side == Side::BOTTOM))
            || (bface.rotation == 0 && (bface.side == Side::TOP));
    }
    if generated_type == 7 {
        return (bface.rotation == 3 && (bface.side == Side::WEST))
            || (bface.rotation == 2
                && (bface.side == Side::SOUTH
                    || bface.side == Side::TOP
                    || bface.side == Side::BOTTOM))
            || (bface.rotation == 1 && (bface.side == Side::EAST))
            || (bface.rotation == 0 && (bface.side == Side::NORTH));
    }
    if generated_type == 8 {
        return (bface.rotation == 1 && (bface.side == Side::TOP || bface.side == Side::BOTTOM))
            || (bface.rotation == 3 && (bface.side != Side::TOP && bface.side != Side::BOTTOM));
    }
    if generated_type == 9 {
        return (bface.rotation == 3 && (bface.side == Side::BOTTOM || bface.side == Side::NORTH))
            || (bface.rotation == 2 && (bface.side == Side::EAST))
            || (bface.rotation == 1 && (bface.side == Side::TOP || bface.side == Side::SOUTH))
            || (bface.rotation == 0 && (bface.side == Side::WEST));
    }
    if generated_type == 10 {
        return bface.rotation == 1;
    }
    if generated_type == 11 {
        return (bface.rotation == 3 && (bface.side == Side::TOP || bface.side == Side::NORTH))
            || (bface.rotation == 2 && (bface.side == Side::WEST))
            || (bface.rotation == 1 && (bface.side == Side::BOTTOM || bface.side == Side::SOUTH))
            || (bface.rotation == 0 && (bface.side == Side::EAST));
    }
    if generated_type == 12 {
        return (bface.rotation == 3 && (bface.side != Side::TOP && bface.side != Side::BOTTOM))
            || (bface.rotation == 2 && (bface.side == Side::TOP))
            || (bface.rotation == 0 && (bface.side == Side::BOTTOM));
    }
    if generated_type == 13 {
        return (bface.rotation == 3 && (bface.side == Side::EAST))
            || (bface.rotation == 2
                && (bface.side == Side::TOP
                    || bface.side == Side::BOTTOM
                    || bface.side == Side::SOUTH))
            || (bface.rotation == 1 && (bface.side == Side::WEST))
            || (bface.rotation == 0 && (bface.side == Side::NORTH));
    }
    if generated_type == 14 {
        return (bface.rotation == 1 && (bface.side != Side::TOP && bface.side != Side::BOTTOM))
            || (bface.rotation == 2 && (bface.side == Side::TOP))
            || (bface.rotation == 0 && (bface.side == Side::BOTTOM));
    }
    if generated_type == 15 {
        return (bface.rotation == 3 && (bface.side == Side::EAST))
            || (bface.rotation == 2 && (bface.side == Side::NORTH))
            || (bface.rotation == 1 && (bface.side == Side::WEST))
            || (bface.rotation == 0
                && (bface.side == Side::SOUTH
                    || bface.side == Side::TOP
                    || bface.side == Side::BOTTOM));
    }
    return false;
}

pub fn rotate90deg(input: Option<&Vec<BlockFace>>) -> Vec<BlockFace> {
    let mut result: Vec<BlockFace> = vec![];
    let formation = input.unwrap();
    for b in formation {
        let mut newside = Side::NORTH;
        let mut rotation = -1;

        if b.side == Side::TOP {
            newside = Side::TOP;
        }
        if b.side == Side::BOTTOM {
            newside = Side::BOTTOM;
        }
        if b.side == Side::WEST {
            newside = Side::SOUTH;
        }
        if b.side == Side::EAST {
            newside = Side::NORTH;
        }
        if b.side == Side::SOUTH {
            newside = Side::EAST;
        }
        if b.side == Side::NORTH {
            newside = Side::WEST;
        }

        if b.side == Side::TOP {
            rotation = (b.rotation + 3) % 4;
        } else if b.side == Side::BOTTOM {
            rotation = (b.rotation + 1) % 4;
        } else {
            rotation = b.rotation;
        }
        result.push(BlockFace {
            x: b.z,
            y: b.y,
            z: b.z,
            side: newside,
            rotation,
        });
    }
    return result;
}

fn read_formation_from_csv<P: AsRef<Path>>(path: P) -> Vec<BlockFace> {
    let mut rdr = ReaderBuilder::new().from_path(path).unwrap();
    let mut formation = Vec::new();
    for result in rdr.records() {
        let record = result.unwrap();
        let x: i64 = record[0].parse().unwrap();
        let y: i64 = record[1].parse().unwrap();
        let z: i64 = record[2].parse().unwrap();
        let side: Side = match &record[3] as &str {
            "TOP" => Side::TOP,
            "BOTTOM" => Side::BOTTOM,
            "WEST" => Side::WEST,
            "EAST" => Side::EAST,
            "SOUTH" => Side::SOUTH,
            "NORTH" => Side::NORTH,
            _ => panic!("Invalid side"),
        };
        let rotation: i32 = record[4].parse().unwrap();
        formation.push(BlockFace {
            x,
            y,
            z,
            side,
            rotation,
        });
    }
    formation
}

fn main() {
    let args = Cli::parse();

    let formation = read_formation_from_csv(&args.formation_file);
    let mut rotations: Vec<Vec<BlockFace>> = Vec::new();

    let xmin = args.xmin;
    let xmax = args.xmax;
    let zmin = args.zmin;
    let zmax = args.zmax;
    let ymin = args.ymin; // Y min and max need to be at least 1 apart FIXME: this comment
    let ymax = args.ymax;
    let use_all_rotations = args.use_all_rotations; // If you don't know which direction North is

    rotations.push(formation);

    if use_all_rotations {
        for _n in 0..3 {
            rotations.push(rotate90deg(rotations.get(rotations.len() - 1)));
        }
    }

    for x in xmin..xmax {
        for z in zmin..zmax {
            for y in ymin..ymax {
                for f in &rotations {
                    let mut found = true;
                    for b in f {
                        let texture = get_texture_type(
                            (x + b.x) as i128,
                            (y + b.y) as i128,
                            (z + b.z) as i128,
                        );
                        if !compatible_rotation(texture, b) {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        println!("Found Possible Coordinates at : {}, {}, {}", x, y, z);
                        return;
                    }
                }
            }
        }
    }

    println!("Finished!");
}
