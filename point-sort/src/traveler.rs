use metaheuristics::Metaheuristics;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::{thread_rng, Rng};
use time::Duration;

struct Traveler<'a> {
    distance_matrix: &'a Vec<Vec<f64>>,
    rng: &'a mut ThreadRng,
}

struct Candidate {
    route: Vec<usize>,
}

impl<'a> Metaheuristics<Candidate> for Traveler<'a> {
    fn clone_candidate(&mut self, candidate: &Candidate) -> Candidate {
        Candidate {
            route: candidate.route.clone(),
        }
    }

    fn generate_candidate(&mut self) -> Candidate {
        let mut route: Vec<usize> = self
            .distance_matrix
            .iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect();
        route.shuffle(&mut self.rng);

        Candidate { route }
    }

    fn rank_candidate(&mut self, candidate: &Candidate) -> f64 {
        0.0 - get_route_distance(self.distance_matrix, &candidate.route)
    }

    fn tweak_candidate(&mut self, candidate: &Candidate) -> Candidate {
        if candidate.route.len() <= 2 {
            return self.clone_candidate(candidate);
        }

        let old_route = candidate.route.clone();

        // get two cities to work with

        let start = self.rng.gen::<usize>() % old_route.len();
        let end = self.rng.gen::<usize>() % old_route.len();
        let (start, end) = if start < end {
            (start, end)
        } else {
            (end, start)
        };

        // straight swap of the cities

        let mut swapped_route = old_route.clone();
        swapped_route.swap(start, end);

        // swap cities, then reverse the cities between them

        let split_route = old_route.clone();
        let safe_offset = if old_route.len() <= (end + 1) {
            old_route.len()
        } else {
            end + 1
        };
        let (left, right) = split_route.split_at(safe_offset);
        let (left, middle) = left.split_at(start);

        let mut middle = middle.to_vec();
        middle.reverse();

        let mut reordered_route = Vec::new();
        reordered_route.extend(left.iter());
        reordered_route.extend(middle.iter());
        reordered_route.extend(right.iter());

        // return shortest route

        let swapped_distance = get_route_distance(self.distance_matrix, &swapped_route);
        let reordered_distance = get_route_distance(self.distance_matrix, &reordered_route);
        let shortest_route = if swapped_distance < reordered_distance {
            swapped_route
        } else {
            reordered_route
        };

        Candidate {
            route: shortest_route,
        }
    }
}

/// Represents a tour of the traveler
pub struct Tour {
    /// the total distance traveled following this tour
    pub distance: f64,
    /// the ordered route for this tour
    pub route: Vec<usize>,
}

fn get_distance_matrix(coordinates: &[(f64, f64, f64)]) -> Vec<Vec<f64>> {
    coordinates
        .iter()
        .map(|row| {
            coordinates
                .iter()
                .map(|column| {
                    ((column.0 - row.0).powi(2)
                        + (column.1 - row.1).powi(2)
                        + (column.2 - row.2).powi(2))
                    .sqrt()
                })
                .collect::<Vec<f64>>()
        })
        .collect::<Vec<Vec<f64>>>()
}

fn get_route_distance(distance_matrix: &[Vec<f64>], route: &[usize]) -> f64 {
    let mut route_iter = route.iter();
    let mut current_city = match route_iter.next() {
        None => return 0.0,
        Some(v) => *v,
    };

    route_iter.fold(0.0, |mut total_distance, &next_city| {
        total_distance += distance_matrix[current_city as usize][next_city as usize];
        current_city = next_city;
        total_distance
    })
}

pub fn solve(locations: &[(f64, f64, f64)], runtime: Duration) -> Tour {
    let mut traveler = Traveler {
        distance_matrix: &get_distance_matrix(locations),
        rng: &mut thread_rng(),
    };

    let best_candidate = metaheuristics::simulated_annealing::solve(&mut traveler, runtime);

    Tour {
        distance: get_route_distance(traveler.distance_matrix, &best_candidate.route),
        route: best_candidate.route,
    }
}
