use anyhow::Result;

struct Terrain {
    map: Box<[Box<[u8]>]>,
    width: usize,
    height: usize,
    tree: u8,
}

impl Terrain {
    pub fn new(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.to_string().into_bytes().into_boxed_slice())
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let width = map[0].len();
        let height = map.len();
        let tree = '#' as u8;

        Terrain {
            map,
            width,
            height,
            tree,
        }
    }

    pub fn count_trees(&self, slope: Slope) -> usize {
        let Slope(dx, dy) = slope;

        let (mut x, mut y) = (0usize, 0usize);
        let mut num_trees = 0;
        loop {
            if y >= self.height {
                break;
            }
            if self.map[y][x] == self.tree {
                num_trees += 1;
            }
            x = (x + dx) % self.width;
            y += dy;
        }

        num_trees
    }
}

#[derive(Debug, Copy, Clone)]
struct Slope(usize, usize);

fn main() -> Result<()> {
    let input = advent20::input_string()?;
    let terrain = Terrain::new(&input);

    let slope = Slope(3, 1);
    let num_trees = terrain.count_trees(slope);

    println!("part 1: {}", num_trees);

    let slopes = vec![
        Slope(1, 1),
        Slope(3, 1),
        Slope(5, 1),
        Slope(7, 1),
        Slope(1, 2),
    ];

    let product: u64 = slopes
        .iter()
        .map(|&s| terrain.count_trees(s) as u64)
        .product();

    println!("part 2: {}", product);

    Ok(())
}
