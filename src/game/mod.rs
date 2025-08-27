use array2d::Array2D;

pub fn future_generation(grid: &Array2D<u8>) -> Array2D<u8> {
    let mut future = Array2D::filled_with(0, grid.num_rows(), grid.num_columns());
    for (index, element) in grid.enumerate_row_major() {
        let mut live_neighbors: u8 = 0;
        for y in -1i64..=1 {
            for x in -1i64..=1 {
                let new_index: (i64, i64) = (index.0 as i64 + y, index.1 as i64 + x);
                if new_index.0 < 0 || new_index.1 < 0 {
                    continue;
                }
                if grid
                    .get(new_index.0 as usize, new_index.1 as usize)
                    .is_none()
                {
                    continue;
                }
                let cell_value = grid
                    .get(new_index.0 as usize, new_index.1 as usize)
                    .unwrap();
                live_neighbors += cell_value;
            }
        }

        live_neighbors -= element;

        if *element == 1u8 {
            match live_neighbors {
                0..2 => future.set(index.0, index.1, 0u8).unwrap_or_default(),
                2 | 3 => future.set(index.0, index.1, 1u8).unwrap_or_default(),
                4..=8 => future.set(index.0, index.1, 0u8).unwrap_or_default(),
                9..=u8::MAX => panic!("More neighbors than possible, your math sucks."),
            }
        } else if live_neighbors == 3u8 {
            future.set(index.0, index.1, 1u8).unwrap_or_default();
        }
    }
    future
}
