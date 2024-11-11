use crate::item::Item;
use crate::knapsack::ProblemKnapsacksBinary;
use crate::problem_type::{BinarySolver, BinaryProblem};
use minilp::{Problem, OptimizationDirection, ComparisonOp};

struct ItemPos<const S: usize> {
    pub j: usize, //index
    pub coord: [f64; S], //item coordinates
    pub dist: f64, //smallest distance from hyperplane
}

#[derive(Clone, Copy)]
pub struct TheoreticalGreedy;
impl<const S: usize> BinarySolver<f64, S> for TheoreticalGreedy {
    type Output = ProblemKnapsacksBinary<f64, S>;

    fn solve(self, mut problem: BinaryProblem<f64, S>) 
    -> ProblemKnapsacksBinary<f64, S> {
        let items = problem.items;
        let knapsack = &mut problem.knapsacks[0];

        //set up ItemPos vector for each item set
        let mut item_positions: Vec<ItemPos<S>> = Vec::with_capacity(items.len());
        for (i, item) in items.iter().enumerate() {
            let mut pos = [0.0; S];
            for r in 0..S {
                pos[r] = item.weights[r] / item.value;
            }

            item_positions.push(ItemPos::<S> {
                j: i,
                coord: pos,
                dist: 0.0,
            });
        }

        //set up dual problem
        let mut dual_problem = Problem::new(OptimizationDirection::Minimize);
        let mut variables: Vec<minilp::Variable> 
            = Vec::with_capacity(total_items);
        for m in 0..S {
            variables.push(dual_problem.add_var(knapsack.capacity[m] -
                                                knapsack.weights()[m],
                           (0.0, f64::INFINITY)));
        }

        for _ in 0..items.len() {
            variables.push(dual_problem.add_var(1.0, (0.0, f64::INFINITY)));
        }

        //set up constraints
        let mut item_count = 0;
        for item in items.iter() {
            let mut w_formula: Vec<(minilp::Variable, f64)> 
                = Vec::with_capacity(S);
            for r in 0..S {
                w_formula.push((variables[r], item.weights[r]));
            }

            let mut full_formula: Vec<(minilp::Variable, f64)> 
                = Vec::with_capacity(w_formula.len() + items.len());
            full_formula.extend_from_slice(&w_formula);
            for j in 0..items.len() {
                if j == item_count {
                    full_formula.push((variables[w_formula.len() + j], 
                                       1.0));
                } else {
                    full_formula.push((variables[w_formula.len() + j], 
                                       0.0));
                }
            }

            dual_problem.add_constraint(&full_formula, 
                                        ComparisonOp::Ge, 
                                        item.value);
            item_count += 1;
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
            item_pos.dist = dot_product/magnitude;
        }

        //now sort objects in increasing order based on distance
        item_positions.sort_by(|x, y| x.dist.partial_cmp(&y.dist).unwrap());

        //now add objects to knapsack
        for item_pos in item_positions {
            if(!knapsack.add(
                Item::<f64, S> {
                    value: items[item_pos.j].value,
                    weights: items[item_pos.j].weights,
                    quantity: 1;
                }
            )) {
                break;
            }
        }

        problem.knapsacks
    }
}
