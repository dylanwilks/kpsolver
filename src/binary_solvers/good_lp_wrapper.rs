use crate::item::Item;
use crate::knapsack::BinaryProblemKnapsacks;
use crate::problem_type::{BinaryProblem, BinarySolver};
use good_lp::{constraint, Solution, SolverModel, variables, variable, Variable, 
              Expression};

macro_rules! good_lp_wrapper {
    ( $( [$solver_name:ident, $solver:expr] ),* ) => {
        $(
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct $solver_name;
impl<const S: usize> BinarySolver<f64, S> for $solver_name
{
    type Output = BinaryProblemKnapsacks<f64, S>;

    fn solve(self, problem: BinaryProblem<f64, S>)
    -> BinaryProblemKnapsacks<f64, S> {
        let items = problem.items;
        let mut knapsacks = problem.knapsacks;

        let m = knapsacks.len();
        let n = items.len();
        let d = S;
        let mut variables = variables!();
        let mut decision_var: Vec<Vec<Variable>> = Vec::with_capacity(m);
        for _i in 0..m {
            let mut decision_var_i: Vec<Variable> = Vec::with_capacity(n);
            for _j in 0..n {
                decision_var_i.push(variables.add(variable().binary()));
            }

            decision_var.push(decision_var_i);
        }

        let mut max_expression = Expression::default();
        for i in 0..m {
            for j in 0..n {
                max_expression += items[j].value * decision_var[i][j];
            }
        }

        let mut model = variables.maximise(max_expression).using($solver);
        for i in 0..m {
            for k in 0..d {
                let mut weight_sum = Expression::default();
                for j in 0..n {
                    weight_sum += items[j].weights[k] * decision_var[i][j];
                }

                model = model.with(constraint!(
                        weight_sum <= knapsacks[i].capacity[k]
                        ));
            }
        }

        for j in 0..n {
            let mut item_sum = Expression::default();
            for i in 0..m {
                item_sum += decision_var[i][j];
            }

            model = model.with(constraint!(item_sum <= 1));
        }

        let solution = model.solve().unwrap();
        for i in 0..m {
            for j in 0..n {
                let x_ij = solution.value(decision_var[i][j]).round();
                knapsacks[i].add(
                    Item::<f64, S> {
                        value: items[j].value,
                        weights: items[j].weights,
                        quantity: x_ij,
                    }
                );
            }
        }

        knapsacks
    }
}
        )*
    }
}

good_lp_wrapper!([CBC, good_lp::coin_cbc],
                 [HiGHS, good_lp::highs],
                 [CPLEX, good_lp::solvers::cplex::cplex]
);
