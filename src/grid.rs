#[derive(Debug)]
pub struct Grid {
    pub cells: Vec<Vec<u8>>,
    pub width: usize,
    pub height: usize,
}

impl Grid {
    pub fn new(input: &str) -> Grid {
        let cells: Vec<Vec<u8>> = input
            .lines()
            .map(|line| line.bytes().into_iter().collect())
            .collect();

        let width = cells[0].len();
        let height = cells.len();

        Self {
            cells,
            width,
            height,
        }
    }

    pub fn index(&self, x: i32, y: i32) -> Option<u8> {
        let x = usize::try_from(x).ok()?;
        let y = usize::try_from(y).ok()?;

        if x < self.height && y < self.width {
            Some(self.cells[x][y])
        } else {
            None
        }
    }

    pub fn find(&self, target: u8) -> Option<(usize, usize)> {
        for (x, row) in self.cells.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                if *cell == target {
                    return Some((x, y));
                }
            }
        }

        None
    }
}
