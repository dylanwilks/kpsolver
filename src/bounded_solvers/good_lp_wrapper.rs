use crate::item::Item;
use crate::knapsack::ProblemKnapsacks;
use crate::problem_type::{BoundedProblem, BoundedSolver};
use good_lp::{constraint, Solution, SolverModel, variables, variable, Variable, 
              Expression};

macro_rules! good_lp_wrapper {
    ( $( [$solver_name:ident, $solver:expr] ),* ) => {
        $(
#[derive(Clone, Copy)]
pub struct $solver_name;
impl<const S: usize> BoundedSolver<f64, S> for $solver_name
{
    fn solve(self, problem: BoundedProblem<f64, S>)
    -> ProblemKnapsacks<f64, S> {
        let items = problem.items;
        let mut knapsacks = problem.knapsacks;
        let m = knapsacks.len();
        let n = items.len();
        let d = S;
        let mut variables = variables!();
        let mut decision_var: Vec<Vec<Variable>> = Vec::with_capacity(m);
        for _i in 0..m {
            let mut decision_var_i: Vec<Variable> = Vec::with_capacity(n);
            for j in 0..n {
                decision_var_i.push(variables.add(variable().integer()
                                             .min(0).max(items[j].quantity)));
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

            model = model.with(constraint!(item_sum <= items[j].quantity));
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
