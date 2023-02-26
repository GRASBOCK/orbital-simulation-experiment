use macroquad::{prelude::*, color};

struct Object{
    position: Vec2,
    velocity: Vec2,
    mass: f32
}

impl Object{
    fn draw(&self){
        let wpos = world_to_viewport(self.position);
        draw_circle(wpos.x, wpos.y, 6.0, YELLOW);
    }
}

fn world_to_viewport(p:Vec2) -> Vec2{
    Vec2 { x: screen_width()/2.0 + p.x, y: screen_height()/2.0 - p.y }
} 

struct Trail{
    positions: Vec<Vec2>
}

impl Trail{
    fn index_pair(start: usize, i:usize, length: usize) -> (usize, usize){
        let a = if start >= i{
            start - i
        }else{
            length - i + start
        };
        let b = if a > 0{
            a - 1
        }else{
            length - 1
        };
        (a, b)
    }
    fn draw(&self, start: usize){
        let l = self.positions.len();
        let dc = 1.0/l as f32;
        for i in 0..l-1 {
            let (a, b) = Trail::index_pair(start, i, self.positions.len());
            let pa = self.positions[a];
            let pb = self.positions[b];
            let wpa = world_to_viewport(pa);
            let wpb = world_to_viewport(pb);
            let color = color::hsl_to_rgb(0.0, 0.0, 1.0 - dc*i as f32);
            draw_line(wpa.x, wpa.y, wpb.x, wpb.y, 2.0, color);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Trail;

    #[test]
    fn trail_indices() {

        let samples: Vec<((usize, usize, usize), (usize, usize))> = vec![
            ((0, 0, 5), (0, 4)), 
            ((0, 1, 5), (4, 3)),
            ((1, 0, 5), (1, 0)),
            ((1, 1, 5), (0, 4)),
            ((0, 3, 5), (2, 1)),
            ((1, 3, 5), (3, 2)),
            ((4, 3, 5), (1, 0)),
        ];
        for s in &samples{
            let (start, i, length) = (s.0.0, s.0.1, s.0.2);
            let output = s.1;
            println!("{:?}", &s);
            assert_eq!(Trail::index_pair(start, i,length), output);
        }
        
    }
}

#[macroquad::main("Orbital Simulation")]
async fn main() {
    let mut objects: Vec<Object> = vec![
        Object{position: Vec2::new(0.5, 0.3), velocity: Vec2::new(0.0, 0.0), mass: 2000.0},
        Object{position: Vec2::new(100.0, 0.3), velocity: Vec2::new(0.0, 30.0), mass: 1.0},
        Object{position: Vec2::new(-100.0, -30.0), velocity: Vec2::new(20.0, -20.0), mass: 1.0},
        Object{position: Vec2::new(-200.0, 40.0), velocity: Vec2::new(0.0, -10.0), mass: 1.0},
    ];

    let trail_length = 5000;
    
    let mut trails: Vec<Trail> = objects
        .iter()
        .map(|obj| Trail{positions: vec![obj.position; trail_length]})
        .collect();
    let mut trail_start_index = 0;

    const G: f32 = 100.0;

    let mut acceleration = vec![Vec2::new(0.0, 0.0); objects.len()];
    loop {
        clear_background(BLACK);
        let delta_time = macroquad::time::get_frame_time();
        
        let mut new_acceleration = vec![Vec2::new(0.0, 0.0); objects.len()];
        for (i, obj_i) in objects.iter().enumerate(){
            for (_j, obj_j) in objects.iter().enumerate().filter(|(j, _obj_j)|{i != *j} ){
                let delta = obj_j.position - obj_i.position;
                let direction = delta.normalize();
                let acc = G*obj_j.mass/delta.length_squared();
                new_acceleration[i] += direction*acc;
            }
        }

        for (i, obj) in objects.iter_mut().enumerate(){
            let mean_acceleration = (acceleration[i] + new_acceleration[i])/2.0;
            obj.position += obj.velocity*delta_time + mean_acceleration/2.0*delta_time.powi(2);
            obj.velocity += mean_acceleration/2.0*delta_time; // mean acceleration
            acceleration[i] = new_acceleration[i];
            trails[i].positions[trail_start_index] = obj.position;
        }

        for trail in &trails{
            trail.draw(trail_start_index);
        }
        trail_start_index += 1;
        if trail_start_index >= trail_length{
            trail_start_index = 0
        }

        for obj in &objects{
            obj.draw();
        }

        next_frame().await
    }
}