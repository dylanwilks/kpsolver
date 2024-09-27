use crate::compatible_problem_type_trait::CompatibleProblemType;
use crate::knapsack::ProblemKnapsacks;
use crate::problem_type::{BoundedProblem, BoundedSolver};
use good_lp::{constraint, Solution, SolverModel, variables, variable, Variable, 
              Expression};

macro_rules! good_lp_wrapper {
    ( $( [$solver_name:ident, $solver:expr] ),* ) => {
        $(
#[derive(Clone, Copy)]
pub struct $solver_name;
impl<T, const S: usize> BoundedSolver<T, S> for $solver_name
where
    T: CompatibleProblemType + Into<f64>,
{
    fn solve(self, problem: BoundedProblem<T, S>) -> ProblemKnapsacks<T, S> {
        let mut items = problem.items;
        let mut knapsacks = problem.knapsacks;
        let m = knapsacks.len();
        let n = items.len();
        let d = S;
        let mut variables = variables!();
        let mut decision_var: Vec<Vec<Variable>> = Vec::with_capacity(m);
        for _i in 0..m {
            let mut decision_var_i: Vec<Variable> = Vec::with_capacity(n);
            for j in 0..n {
                if let Ok(q_j) = i32::try_from(items[j].quantity) {
                    decision_var_i.push(variables.add(variable().integer()
                                                                .min(0)
                                                                .max(q_j)));
                } else {
                    panic!("Item quantity too large for problem.");
                }
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
                    weight_sum += items[j].weights[k].into() 
                                * decision_var[i][j];
                }

                model = model.with(constraint!(
                        weight_sum <= knapsacks[i].capacity[k].into()
                        ));
            }
        }

        for j in 0..n {
            let mut item_sum = Expression::default();
            for i in 0..m {
                item_sum += decision_var[i][j];
            }

            if let Ok(q_j) = i32::try_from(items[j].quantity) {
                model = model.with(constraint!(item_sum <= q_j));
            } else {
                panic!("Item quantity too large for problem.");
            }
        }

        let solution = model.solve().unwrap();
        for i in 0..m {
            for j in 0..n {
                let x_ij = solution.value(decision_var[i][j]).round() as usize;
                knapsacks[i].add_mut(&mut items[j], x_ij);
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
