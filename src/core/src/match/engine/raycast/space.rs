use crate::r#match::MatchPlayer;
use nalgebra::Vector3;

pub struct Space<T: Collider> {
    colliders: Vec<T>,
}

impl<T: Collider> Space<T> {
    pub fn new() -> Self {
        Space {
            colliders: Vec::new(),
        }
    }

    pub fn add_collider(&mut self, collider: T) {
        self.colliders.push(collider);
    }

    pub fn cast_ray(
        &self,
        origin: Vector3<f32>,
        direction: Vector3<f32>,
        max_distance: f32,
        include_players: bool,
    ) -> Option<RaycastHit<T>> {
        let mut closest_hit: Option<RaycastHit<T>> = None;
        let mut closest_distance = max_distance;

        // Iterate through all colliders in the space
        for collider in &self.colliders {
            // Check if the collider belongs to a player
            if let Some(player) = collider.match_player() {
                if !include_players {
                    // Skip player colliders if include_players is false
                    continue;
                }
            }

            // Perform ray intersection test with the collider
            if let Some(intersection) = collider.intersect_ray(origin, direction) {
                let distance = (intersection - origin).magnitude();

                if distance < closest_distance {
                    closest_distance = distance;
                    closest_hit = Some(RaycastHit {
                        collider: collider.clone(),
                        point: intersection,
                        normal: collider.normal(intersection),
                        distance,
                    });
                }
            }
        }

        closest_hit
    }
}

pub struct RaycastHit<T: Collider> {
    pub collider: T,
    point: Vector3<f32>,
    normal: Vector3<f32>,
    distance: f32,
}

pub trait Collider: Clone {
    fn intersect_ray(&self, origin: Vector3<f32>, direction: Vector3<f32>) -> Option<Vector3<f32>>;
    fn normal(&self, point: Vector3<f32>) -> Vector3<f32>;
    fn match_player(&self) -> Option<&MatchPlayer>;
}

// Example collider implementations

#[derive(Clone)]
pub struct SphereCollider {
    pub center: Vector3<f32>,
    pub radius: f32,
    pub player: Option<MatchPlayer>,
}

impl Collider for SphereCollider {
    fn intersect_ray(&self, origin: Vector3<f32>, direction: Vector3<f32>) -> Option<Vector3<f32>> {
        let oc = origin - self.center;
        let a = direction.dot(&direction);
        let b = 2.0 * oc.dot(&direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            // No intersection
            None
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);

            if t1 >= 0.0 && t2 >= 0.0 {
                // Two intersections, return the closer one
                let t = t1.min(t2);
                Some(origin + t * direction)
            } else if t1 >= 0.0 {
                // One intersection (t1)
                Some(origin + t1 * direction)
            } else if t2 >= 0.0 {
                // One intersection (t2)
                Some(origin + t2 * direction)
            } else {
                // No intersection (both t1 and t2 are negative)
                None
            }
        }
    }

    fn normal(&self, point: Vector3<f32>) -> Vector3<f32> {
        (point - self.center).normalize()
    }

    fn match_player(&self) -> Option<&MatchPlayer> {
        self.player.as_ref()
    }
}
