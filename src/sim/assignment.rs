use web_time::{ Instant, Duration };

pub struct AssignmentResult {
    pub assignment: Vec<usize>,
    pub runtime: Duration,
}

pub struct Greedy;

pub trait AssignmentAlgorithm {
    fn solve(&self, cost_matrix: &Vec<Vec<f64>>) -> Option<AssignmentResult>;
}

pub enum Solver {
    Greedy(Greedy),
}

impl Solver {
    pub fn solve(&self, cost_mat: &Vec<Vec<f64>>) -> Option<AssignmentResult> {
        match self {
            Solver::Greedy(greedy) => greedy.solve(cost_mat)
        }
    }
}

impl AssignmentAlgorithm for Greedy {
    fn solve(&self, cost_mat: &Vec<Vec<f64>>) -> Option<AssignmentResult> {
        let start = Instant::now();
        let n_rows = cost_mat.len();
        if n_rows == 0 {
            return None
        }
        let n_cols = cost_mat[0].len();

        let mut assignment = vec![usize::MAX; n_cols];
        let mut taken = vec![false; n_cols];

        for row in 0..n_rows {
            let best_col = (0..n_cols)
                .filter(|&j| !taken[j])
                .max_by(|&a, &b| cost_mat[row][a].partial_cmp(&cost_mat[row][b]).unwrap());

            if let Some(col) = best_col {
                assignment[row] = col;
                taken[col] = true;
            }
        }

        let runtime = start.elapsed();

        Some(AssignmentResult {
            assignment,
            runtime
        })
    }
}