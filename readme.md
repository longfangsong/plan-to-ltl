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

For example, a problem:

```rust
domain example {
    type Type;

    predicate p0(a: Type, b: Type);
    predicate p1(c: Type);

    action a0(q0: Type, q1: Type)
        requires p0(q0, q1)
        ensures p1(q0) & !p1(q1);
}

problem example-proble: example {
    let v1: Type, v2: Type;

    init {
        p0(v1, v2);
    }

    goal {
        p1(v1);
        !p1(v2);
    }
}
```

Will become:

```
(o1 -> (
  G((o6 & o0) -> X((o4 & !o4))) &
  G((o7 & o1) -> X((o4 & !o5))) &
  G((o8 & o2) -> X((o5 & !o4))) &
  G((o9 & o3) -> X((o5 & !o5))) &
  F(o4 & !o5)
))
```

Where each output means:

```
o0: p0(v1, v1)
o1: p0(v1, v2)
o2: p0(v2, v1)
o3: p0(v2, v2)
o4: p1(v1)
o5: p1(v2)
o6: a0(v1, v1)
o7: a0(v1, v2)
o8: a0(v2, v1)
o9: a0(v2, v2)
```

And this LTL is `REALIZABLE` by `ltlsynt`.
