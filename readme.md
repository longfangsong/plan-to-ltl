# plan to ltl

## Design

- For each action or predicate $f(a, b)$,
  where $a$ has $n_a$ possible values and $b$ has $n_b$ possible values,
  we generate $n_a \times n_b$ output variables, we can label them from $o[0][0]$ to $o[n_a-1][n_b-1]$.
- For each action:

$$
  \textbf{G}((\text{action} \land \bigwedge \text{precodition}) \rightarrow \bigwedge \text{postcondition})
$$

- For goal, we simply $\bigwedge$ all goals

$$
  \textbf{F}(\bigwedge \text{goal})
$$

- For initial, we simply $\bigwedge$ all non-trivial init conditions

$$
  \bigwedge \text{init}
$$

Final result LTL equals to:

$$
\bigwedge \text{init} \rightarrow \\ (\bigwedge_{actions}
  \textbf{G}((\text{action} \land \bigwedge \text{precodition}) \rightarrow \bigwedge \text{postcondition}) \land \\
  \textbf{F}(\bigwedge \text{goal})
)
$$

