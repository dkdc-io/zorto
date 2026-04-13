+++
title = "Live visualizations"
weight = 55
+++

{{ slide_image(src="/zorto-mark-transparent.png", alt="Zorto", top="20px", right="20px", width="56px") }}

## Live visualizations at build time

```{python}
import matplotlib.pyplot as plt
import math

x = [i * 0.08 for i in range(120)]
y = [math.sin(v) * math.exp(-v * 0.15) for v in x]

plt.figure(figsize=(8, 2.6))
plt.plot(x, y, color='#7c3aed', linewidth=2.2)
plt.fill_between(x, y, alpha=0.18, color='#06b6d4')
plt.title('Damped oscillation', color='#e2e8f0')
plt.grid(True, alpha=0.2)
plt.gca().set_facecolor('#0f172a')
plt.gcf().set_facecolor('#1a1a2e')
plt.tick_params(colors='#94a3b8')
for s in plt.gca().spines.values():
    s.set_color('#334155')
plt.tight_layout()
```

**matplotlib, plotly, altair, seaborn** -- captured automatically.
