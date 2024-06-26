use ggez::mint::Point2;

pub struct WorldMap {
    world_map: Vec<Vec<i32>>
}

impl WorldMap {
    pub fn new() -> Self {
        let world_map = vec![
            vec![4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,4,7,7,7,7,7,7,7,7],
            vec![4,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,7],
            vec![4,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7],
            vec![4,0,2,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7],
            vec![4,0,3,0,0,0,0,0,0,0,0,0,0,0,0,0,7,0,0,0,0,0,0,7],
            vec![4,0,4,0,0,0,0,5,5,5,5,5,5,5,5,5,7,7,0,7,7,7,7,7],
            vec![4,0,5,0,0,0,0,5,0,5,0,5,0,5,0,5,7,0,0,0,7,7,7,1],
            vec![4,0,6,0,0,0,0,5,0,0,0,0,0,0,0,5,7,0,0,0,0,0,0,8],
            vec![4,0,7,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,7,7,7,1],
            vec![4,0,8,0,0,0,0,5,0,0,0,0,0,0,0,5,7,0,0,0,0,0,0,8],
            vec![4,0,0,0,0,0,0,5,0,0,0,0,0,0,0,5,7,0,0,0,7,7,7,1],
            vec![4,0,0,0,0,0,0,5,5,5,5,0,5,5,5,5,7,7,7,7,7,7,7,1],
            vec![6,6,6,6,6,6,6,6,6,6,6,0,6,6,6,6,6,6,6,6,6,6,6,6],
            vec![8,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,4],
            vec![6,6,6,6,6,6,0,6,6,6,6,0,6,6,6,6,6,6,6,6,6,6,6,6],
            vec![4,4,4,4,4,4,0,4,4,4,6,0,6,2,2,2,2,2,2,2,3,3,3,3],
            vec![4,0,0,0,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,0,0,0,2],
            vec![4,0,0,0,0,0,0,0,0,0,0,0,6,2,0,0,5,0,0,2,0,0,0,2],
            vec![4,0,0,0,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,2,0,2,2],
            vec![4,0,6,0,6,0,0,0,0,4,6,0,0,0,0,0,5,0,0,0,0,0,0,2],
            vec![4,0,0,5,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,2,0,2,2],
            vec![4,0,6,0,6,0,0,0,0,4,6,0,6,2,0,0,5,0,0,2,0,0,0,2],
            vec![4,0,0,0,0,0,0,0,0,4,6,0,6,2,0,0,0,0,0,2,0,0,0,2],
            vec![4,4,4,4,4,4,4,4,4,4,1,1,1,2,2,2,2,2,2,3,3,3,3,3]
        ];

        Self {
            world_map
        }
    }

    pub fn at(&self, x: &mut i32, y: &mut i32) -> i32 {
        self.check_limits(x, y);
        self.world_map[*x as usize][*y as usize]
    }

    pub fn insert(&mut self, value: i32, mut x: i32, mut y: i32) -> bool {
        if self.at(&mut x, &mut y) == 0 {
            self.world_map[x as usize][y as usize] = value;

            return true;
        }

        false
    }

    pub fn change_position(&mut self, mut from: Point2<i32>, mut to: Point2<i32>) -> bool {
        self.check_limits(&mut from.x, &mut from.y);

        let at = self.at(&mut to.x, &mut to.y);

        if at == 0 || at == b'P' as i32 {
            let tmp = self.world_map[from.x as usize][from.y as usize];
            self.world_map[from.x as usize][from.y as usize] = self.world_map[to.x as usize][to.y as usize];
            self.world_map[to.x as usize][to.y as usize] = tmp;

            return true;
        }

        false
    }

    fn check_limits(&self, x: &mut i32, y: &mut i32) {
        if *x < 0 { *x = 0; }
        if *x as usize >= self.world_map.len() { *x = self.world_map.len() as i32 - 1; }
        if *y < 0 { *y = 0; }
        if *y as usize >= self.world_map[0].len() { *y = self.world_map[0].len() as i32 - 1; }
    }
}
