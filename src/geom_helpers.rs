#[derive(Debug, Clone)]
pub struct Point {
    x: f32,
    y: f32
}

impl Point {
    pub fn new (x: f32, y: f32) -> Self {
        Self{ x, y }
    }

    pub fn new_u32(x: u32, y: u32) -> Self {
        Self{
            x: x as f32, 
            y: y as f32 
        }
    }
}

impl From<(f32, f32)> for Point {
    fn from(value: (f32, f32)) -> Self {
        Point::new(value.0, value.1)
    }
}

pub fn calc_eucledian_distance(p1: &Point, p2: &Point) -> f32{
    ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)).sqrt()
}

#[test]
fn test_calc_eucledian_distance() {

    fn test_calc_eucledian_distance_helper(x1: f32, y1: f32, x2: f32, y2:f32, expected_distance: f32) -> bool{
        let allowed_delta: f32 = 0.0001;
    
        let p1 = Point::new(x1, y1);
        let p2 = Point::new(x2, y2);
        let distance = calc_eucledian_distance(&p1, &p2);
        (distance - expected_distance).abs() < allowed_delta
    }

    assert!(test_calc_eucledian_distance_helper(0.0, 0.0, 0.0, 0.0, 0.0));

    assert!(!test_calc_eucledian_distance_helper(0.0, 0.0, 0.0, 0.0, 1.0));

    assert!(test_calc_eucledian_distance_helper(0.0, 0.0, 1.0, 0.0, 1.0));
    assert!(test_calc_eucledian_distance_helper(0.0, 0.0, 0.0, 1.0, 1.0));
    assert!(test_calc_eucledian_distance_helper(0.0, 0.0, -1.0, 0.0, 1.0));
    assert!(test_calc_eucledian_distance_helper(0.0, 0.0, 0.0, -1.0, 1.0));

    assert!(test_calc_eucledian_distance_helper(0.0, 0.0, 1.0, 1.0, 2f32.sqrt()));

    assert!(test_calc_eucledian_distance_helper(-2.0, 3.0, 12.0, 45.0, 44.2719));
    assert!(test_calc_eucledian_distance_helper(12.0, 45.0, -2.0, 3.0, 44.2719));

}

pub struct Viewport {
    width: f32,
    height: f32
}

impl Viewport {
    pub fn translate(&self, p: Point) -> Point {
        let new_x = ( self.width / 2.0 + p.x).round().max(0.0).min( self.width);
        let new_y = (self.height / 2.0 - p.y).round().max(0.0).min(self.height);

        Point::new(new_x, new_y)
    }
}

pub struct ImgSize (pub u32, pub u32);

impl From<&ImgSize> for Viewport {
    fn from(value: &ImgSize) -> Self {
        Viewport { width: value.0 as f32, height: value.1 as f32 }
    }
}


#[test]
fn test_viewport_translation() {
    let img_size = ImgSize(100, 200);
    let viewport = Viewport::from(&img_size);

    let p2 = viewport.translate(Point::new(0.0, 0.0));
    assert_eq!(p2.x, 50.0);
    assert_eq!(p2.y, 100.0);

    let p2 = viewport.translate(Point::new(-20.0, 30.0));
    assert_eq!(p2.x, 30.0);
    assert_eq!(p2.y, 70.0);

    let p2 = viewport.translate(Point::new(-50.0, 100.0));
    assert_eq!(p2.x, 0.0);
    assert_eq!(p2.y, 0.0);

    let p2 = viewport.translate(Point::new(50.0, 100.0));
    assert_eq!(p2.x, 100.0);
    assert_eq!(p2.y, 0.0);

    let p2 = viewport.translate(Point::new(50.0, -100.0));
    assert_eq!(p2.x, 100.0);
    assert_eq!(p2.y, 200.0);

    let p2 = viewport.translate(Point::new(-50.0, -100.0));
    assert_eq!(p2.x, 0.0);
    assert_eq!(p2.y, 200.0);

    let p2 = viewport.translate(Point::new(20.0, 130.0));
    assert_eq!(p2.x, 70.0);
    assert_eq!(p2.y, 0.0);

    let p2 = viewport.translate(Point::new(20.0, -130.0));
    assert_eq!(p2.x, 70.0);
    assert_eq!(p2.y, 200.0);

    let p2 = viewport.translate(Point::new(-150.0, 30.0));
    assert_eq!(p2.x, 0.0);
    assert_eq!(p2.y, 70.0);

    let p2 = viewport.translate(Point::new(150.0, 30.0));
    assert_eq!(p2.x, 100.0);
    assert_eq!(p2.y, 70.0);
}