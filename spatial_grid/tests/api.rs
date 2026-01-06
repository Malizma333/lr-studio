#[cfg(test)]
mod tests {
    use format_core::track::GridVersion;
    use geometry::{Line, Point};
    use spatial_grid::Grid;

    fn get_lines() -> Vec<Line> {
        let mut lines = Vec::new();
        for x1 in -2..3 {
            for x2 in -2..3 {
                for y1 in -2..3 {
                    for y2 in -2..3 {
                        let x1 = 14.0 * f64::from(x1);
                        let y1 = 14.0 * f64::from(y1);
                        let x2 = 14.0 * f64::from(x2);
                        let y2 = 14.0 * f64::from(y2);
                        lines.push(Line::new(Point::new(x1, y1), Point::new(x2, y2)));
                    }
                }
            }
        }
        lines
    }

    #[test]
    fn add_line() {
        let mut grid = Grid::new(GridVersion::V6_0);

        for line in get_lines() {
            let id = grid.add_line(line);
            assert!(
                grid.get_lines_near_point(line.p0()).contains(&id),
                "Point of line should be near added line id"
            );
            assert!(
                grid.get_lines_near_point(line.p1()).contains(&id),
                "Point of line should be near added line id"
            );
        }
    }

    #[test]
    fn update_line() {
        let mut grid = Grid::new(GridVersion::V6_0);

        for line in get_lines() {
            let id = grid.add_line(Line::new(
                Point::new(1000.0, 1000.0),
                Point::new(1000.0, 1000.0),
            ));
            assert!(
                !grid.get_lines_near_point(line.p0()).contains(&id),
                "Point of line should not be included yet"
            );
            assert!(
                !grid.get_lines_near_point(line.p1()).contains(&id),
                "Point of line should not be included yet"
            );
            grid.update_line(id, line);
            assert!(
                grid.get_lines_near_point(line.p0()).contains(&id),
                "Point of line should be included"
            );
            assert!(
                grid.get_lines_near_point(line.p1()).contains(&id),
                "Point of line should be included"
            );
        }
    }

    #[test]
    fn remove_line() {
        let mut grid = Grid::new(GridVersion::V6_0);

        for line in get_lines() {
            let id = grid.add_line(line);
            assert!(
                grid.get_lines_near_point(line.p0()).contains(&id),
                "Point of line should be included"
            );
            assert!(
                grid.get_lines_near_point(line.p1()).contains(&id),
                "Point of line should be included"
            );
            grid.remove_line(id);
            assert!(
                !grid.get_lines_near_point(line.p0()).contains(&id),
                "Point of line should no longer be included"
            );
            assert!(
                !grid.get_lines_near_point(line.p1()).contains(&id),
                "Point of line should no longer be included"
            );
        }
    }

    #[test]
    fn migrate_version() {
        let mut grid = Grid::new(GridVersion::V6_0);
        for line in get_lines() {
            grid.add_line(line);
        }

        for version in [GridVersion::V6_2, GridVersion::V6_1, GridVersion::V6_0] {
            grid.update_version(version);
            let mut other_grid = Grid::new(version);
            for line in get_lines() {
                other_grid.add_line(line);
            }

            for line in get_lines() {
                assert_eq!(
                    grid.get_lines_near_point(line.p0()),
                    other_grid.get_lines_near_point(line.p0()),
                    "Adding to new grid should be same as switching versions"
                );
                assert_eq!(
                    grid.get_lines_near_point(line.p1()),
                    other_grid.get_lines_near_point(line.p1()),
                    "Adding to new grid should be same as switching versions"
                );
            }
        }
    }
}
