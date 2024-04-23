#[cfg(test)]
mod tests {
    use crate::{ClipperD, ClipperPathD, PathD, PathsD, PointD};


    #[test]
    fn test_path_d_work() {
        let size = std::mem::align_of::<ClipperPathD>();
        println!("{:?}",size);
        let point = PointD {x: 0.0, y: 0.0};
        let mut path = PathD::new(&vec![]);
        path.add_point(point);
        assert_eq!(path.len(), 1);
        assert_eq!(path.get_point(0).x, point.x);
        assert_eq!(path.get_point(0).y, point.y);
    }

    #[test]
    fn test_paths_d_work() {
        let size = std::mem::align_of::<ClipperPathD>();
        println!("{:?}",size);
        let point = PointD {x: 0.0, y: 0.0};

        let mut path = PathD::new(&vec![]);
        path.add_point(point);

        let mut paths = PathsD::new(&vec![]);
        paths.add_path(path);
        assert_eq!(paths.len(), 1);
        // assert_eq!(paths.get_path(0).x, point.x);
        // assert_eq!(paths.get_path(0).y, point.y);
    }

    #[test]
    fn test_clipper_union_work() {
        let clipper = ClipperD::new(2);
        let mut path = PathD::new(&vec![
            PointD {x: 0.0, y: 0.0},
            PointD {x: 0.0, y: 10.0},
            PointD {x: 10.0, y: 10.0},
            PointD {x: 10.0, y: 0.0},
        ]);

        let mut paths = PathsD::new(&vec![]);
        paths.add_path(path);
        paths.add_path(PathD::new(&vec![
            PointD {x: 20.0, y: 0.0},
            PointD {x: 20.0, y: 10.0},
            PointD {x: 30.0, y: 10.0},
            PointD {x: 30.0, y: 0.0},
        ]));
        println!("paths: {:?}", paths);
        clipper.add_subject(paths);

        // let mut clip_paths = PathsD::new(&vec![]);
        // clip_paths.add_path(PathD::new(&vec![
        //     PointD {x: 20.0, y: 0.0},
        //     PointD {x: 20.0, y: 10.0},
        //     PointD {x: 30.0, y: 10.0},
        //     PointD {x: 30.0, y: 0.0},
        // ]));

        // clipper.add_clip(clip_paths);
        let result = clipper.boolean_operation(crate::ClipType::Union, crate::FillRule::NonZero);        
        println!("result: {:?}", result);
    }

    #[test]
    fn test_clipper_union_tree_work() {
        let clipper = ClipperD::new(2);
        let mut path = PathD::new(&vec![
            PointD {x: 0.0, y: 0.0},
            PointD {x: 0.0, y: 10.0},
            PointD {x: 10.0, y: 10.0},
            PointD {x: 10.0, y: 0.0},
        ]);

        let mut paths = PathsD::new(&vec![]);
        paths.add_path(path);
        paths.add_path(PathD::new(&vec![
            PointD {x: 20.0, y: 0.0},
            PointD {x: 20.0, y: 10.0},
            PointD {x: 30.0, y: 10.0},
            PointD {x: 30.0, y: 0.0},
        ]));
        println!("paths: {:?}", paths);
        clipper.add_subject(paths);

        // let mut clip_paths = PathsD::new(&vec![]);
        // clip_paths.add_path(PathD::new(&vec![
        //     PointD {x: 20.0, y: 0.0},
        //     PointD {x: 20.0, y: 10.0},
        //     PointD {x: 30.0, y: 10.0},
        //     PointD {x: 30.0, y: 0.0},
        // ]));

        // clipper.add_clip(clip_paths);
        let result = clipper.boolean_operation_tree(crate::ClipType::Union, crate::FillRule::NonZero);        
        println!("result: {:?}", result);
    }

}