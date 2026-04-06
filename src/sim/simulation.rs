use std::collections::HashMap;
use std::time::Duration;
use rand::distr::Distribution;
use rand::prelude::SliceRandom;
use rand::rng;
use rand_distr::Normal;
use crate::sim::assignment::{Greedy, Solver};
use crate::components::sandbox::model::{Entity, Kind};

// TODO: maybe simulation should have its own model for data instead of the presentation model
pub struct Simulation {
    pub observers: Vec<Entity>,
    pub targets: Vec<Entity>,
}

pub struct SimulationResult {
    pub actual: Vec<usize>,
    pub assignment: Vec<usize>,
    pub correct: usize,
    pub runtime: Duration
}

impl Simulation {
    pub fn simulate(&self, solver: &Solver) -> Option<SimulationResult> {
        let mut rng = rng();

        let mut measurements = self.generate_measurements();

        // shuffle measurements, as having them diagonally introduces bias
        let mut perm: Vec<usize> = (0..measurements[0].len()).collect();
        perm.shuffle(&mut rng);
        measurements = measurements.iter()
            .map(|observer_row| perm.iter().map(|&i| observer_row[i].clone()).collect())
            .collect();

        let ll_mat = self.combined_likelihood_matrix(&measurements);

        // let expected: Vec<usize> = (0..self.targets.len()).collect();
        // permutation used in shuffling is the expected order
        let expected = perm;

        let solution = solver.solve(&ll_mat)?;

        let correct = solution.assignment
            .iter()
            .zip(&expected)
            .filter(|(assigned, expected)| assigned == expected)
            .count();

        Some(SimulationResult {
            actual: expected,
            assignment: solution.assignment,
            correct,
            runtime: solution.runtime
        })
    }

    pub fn predicted_bearing(observer: &Entity, target: &Entity) -> f64 {
        let dx = target.position.x - observer.position.x;
        let dy = target.position.y - observer.position.y;

        dy.atan2(dx)
    }

    pub fn generate_measurements(&self) -> Vec<Vec<f64>> {
        self.observers.iter().map(|observer| {
            self.targets.iter().map(|target| {
                let std = match &observer.kind {
                    Kind::Observer { std } => *std,
                    Kind::Target { .. } => panic!("Observer was a target"),
                };
                let pred = Self::predicted_bearing(observer, target);
                let noise = Normal::new(0.0, std.to_radians()).unwrap().sample(&mut rng());
                pred + noise
            }).collect::<Vec<f64>>()
        }).collect::<Vec<Vec<f64>>>()
    }

    pub fn combined_likelihood_matrix(&self, measurements: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let n = self.targets.len();
        let mut ll_mat = vec![vec![1.0; n]; n];

        for (s_idx, observer) in self.observers.iter().enumerate() {
            let std = match &observer.kind {
                Kind::Observer { std } => *std,
                Kind::Target { .. } => panic!("Observer was a target"),
            };
            for i in 0..n {
                for j in 0..n {
                    let measurement = measurements[s_idx][i];
                    let prediction = Self::predicted_bearing(observer, &self.targets[j]);
                    ll_mat[i][j] *= normal_pdf(
                        wrap_angle(measurement - prediction),
                        0.0,
                        std.to_radians()
                    );
                }
            }
        }

        ll_mat
    }
}

pub fn monte_carlo(observers: &[Entity], targets: &[Entity], n_trials: usize) -> (f64, f64, HashMap<usize, f64>) {
    let n_targets = targets.len();
    let simulation = Simulation { observers: observers.to_vec(), targets: targets.to_vec() };

    let mut per_target = HashMap::new();
    let mut correct_sum: usize = 0;
    let mut runtime_sum_us: u128 = 0;
    let mut completed: usize = 0;

    for _ in 0..n_trials {
        let solver = Solver::Greedy(Greedy);
        if let Some(run) = simulation.simulate(&solver) {
            correct_sum += run.correct;
            runtime_sum_us += run.runtime.as_micros();
            completed += 1;

            for (assigned, actual) in run.assignment.iter().zip(run.actual.iter()) {
                let target_id = targets[*actual].id;
                let entry = per_target.entry(target_id).or_insert((0, 0));
                entry.1 += 1;
                if assigned == actual {
                    entry.0 += 1;
                }
            }
        }
    }

    let per_target_accuracy = per_target
        .into_iter()
        .map(|(id, (correct, total))| (id, correct as f64 / total as f64))
        .collect();


    if completed == 0 {
        (0.0, 0.0, per_target_accuracy)
    } else {
        (correct_sum as f64 / (n_trials * n_targets) as f64, runtime_sum_us as f64 / completed as f64, per_target_accuracy)
    }
}

pub fn normal_pdf(x: f64, mean: f64, std: f64) -> f64 {
    let coefficient = FRAC_1_SQRT_2_PI / std;
    let exponent = -0.5 * ((x - mean) / std).powi(2);
    coefficient * exponent.exp()
}

pub fn wrap_angle(angle: f64) -> f64 {
    (angle + std::f64::consts::PI) % (2.0 * std::f64::consts::PI) - std::f64::consts::PI
}

const FRAC_1_SQRT_2_PI: f64 = std::f64::consts::FRAC_1_SQRT_2 * std::f64::consts::FRAC_2_SQRT_PI;