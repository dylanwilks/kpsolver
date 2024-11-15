use crate::item::Item;
use crate::knapsack::ProblemKnapsacks;
use crate::problem_type::{BoundedProblem, BoundedSolver};
use minilp::{ComparisonOp, OptimizationDirection, Problem};

struct ItemPos<const S: usize> {
    pub j: usize,        //index
    pub coord: [f64; S], //item coordinates
    pub dist: f64,       //smallest distance from hyperplane
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct TheoreticalGreedy;
impl<const S: usize> BoundedSolver<f64, S> for TheoreticalGreedy {
    type Output = ProblemKnapsacks<f64, S>;

    fn solve(self, mut problem: BoundedProblem<f64, S>) -> ProblemKnapsacks<f64, S> {
        let items = problem.items;
        let knapsack = &mut problem.knapsacks[0];

        //set up ItemPos vector for each item set
        let mut total_items = 0;
        let mut item_positions: Vec<ItemPos<S>> = Vec::with_capacity(items.len());
        for (i, item) in items.iter().enumerate() {
            let mut pos = [0.0; S];
            for r in 0..S {
                if item.weights[r] != 0.0 || item.value != 0.0 {
                    pos[r] = item.weights[r] / item.value;
                }
            }

            item_positions.push(ItemPos::<S> {
                j: i,
                coord: pos,
                dist: 0.0,
            });

            total_items += item.quantity as usize;
        }

        //set up dual problem
        let mut dual_problem = Problem::new(OptimizationDirection::Minimize);
        let mut variables: Vec<minilp::Variable> = Vec::with_capacity(total_items);
        for m in 0..S {
            variables.push(dual_problem.add_var(
                knapsack.capacity[m] - knapsack.weights()[m],
                (0.0, f64::INFINITY),
            ));
        }

        for _ in 0..total_items {
            variables.push(dual_problem.add_var(1.0, (0.0, f64::INFINITY)));
        }

        //set up constraints
        let mut item_count = 0;
        for item in items.iter() {
            let mut w_formula: Vec<(minilp::Variable, f64)> = Vec::with_capacity(S);
            for r in 0..S {
                w_formula.push((variables[r], item.weights[r]));
            }

            for _ in 0..item.quantity as usize {
                let mut full_formula: Vec<(minilp::Variable, f64)> =
                    Vec::with_capacity(w_formula.len() + total_items);
                full_formula.extend_from_slice(&w_formula);
                for j in 0..total_items {
                    if j == item_count {
                        full_formula.push((variables[w_formula.len() + j], 1.0));
                    } else {
                        full_formula.push((variables[w_formula.len() + j], 0.0));
                    }
                }

                dual_problem.add_constraint(&full_formula, ComparisonOp::Ge, item.value);
                item_count += 1;
            }
        }

        //solve dual problem using simplex algorithm
        let solution = dual_problem.solve().unwrap();

        //set up normal vector of hyperplane using optimal relevance values found above
        let mut hyperplane_norm: Vec<f64> = Vec::with_capacity(S);
        for r in 0..S {
            hyperplane_norm.push(solution[variables[r]]);
        }

        //find distance of each ItemPos object from origin along the normal vector
        for item_pos in item_positions.iter_mut() {
            let mut dot_product = 0.0;
            for r in 0..S {
                dot_product += item_pos.coord[r] * hyperplane_norm[r];
            }

            let mut magnitude = 0.0;
            for r in 0..S {
                magnitude += hyperplane_norm[r].powf(2.0)
            }

            magnitude = magnitude.sqrt();
            item_pos.dist = dot_product / magnitude;
        }

        //now sort objects in increasing order based on distance
        item_positions.sort_by(|x, y| x.dist.partial_cmp(&y.dist).unwrap());

        //now add objects to knapsack
        for item_pos in item_positions {
            let mut can_fit = items[item_pos.j].quantity as usize;
            for r in 0..S {
                let rem = knapsack.capacity[r] - knapsack.weights()[r];
                let q_div = (rem / items[item_pos.j].weights[r].floor()) as usize;
                if q_div < can_fit {
                    can_fit = q_div;
                }
            }

            knapsack.add(Item::<f64, S> {
                value: items[item_pos.j].value,
                weights: items[item_pos.j].weights,
                quantity: can_fit as f64,
            });
        }

        problem.knapsacks
    }
}
