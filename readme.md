# plan to ltl

## Design

- For each predicate and action $f(a, b)$,
  where $a$ has $n_a$ possible values and $b$ has $n_b$ possible values,
  we generate $n_a \times n_b$ output variables.

- For each action and each tuple of parameter:

$$
  \textbf{G}((\text{action} \land \bigwedge \text{precondition}) \rightarrow \textbf{X}(\bigwedge \text{effect}))
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
  \textbf{G}((\text{action} \land \bigwedge \text{precondition}) \rightarrow \textbf{X}(\bigwedge \text{effect})) \land \\
  \textbf{F}(\bigwedge \text{goal})
)
$$
