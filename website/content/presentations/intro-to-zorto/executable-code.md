+++
title = "Executable code blocks"
weight = 50
+++

{{ slide_image(src="/zorto-mark-transparent.png", alt="Zorto", top="20px", right="20px", width="72px") }}

## Executable code blocks

Write Python in a fenced block -- Zorto runs it at build time and embeds the result:

```{python}
import plotly.graph_objects as go

months = ['Jan', 'Feb', 'Mar', 'Apr', 'May', 'Jun']
revenue = [12, 15, 13, 17, 21, 24]
costs = [10, 11, 12, 13, 14, 15]

chart = go.Figure()
chart.add_trace(go.Scatter(x=months, y=revenue, name='Revenue',
    line=dict(color='#7c3aed', width=3)))
chart.add_trace(go.Scatter(x=months, y=costs, name='Costs',
    line=dict(color='#06b6d4', width=3)))
chart.update_layout(template='plotly_dark', height=320,
    margin=dict(l=40, r=20, t=20, b=40),
    paper_bgcolor='rgba(0,0,0,0)', plot_bgcolor='rgba(0,0,0,0.3)')
```

Matplotlib, Plotly, Altair, Seaborn -- all captured and baked into static HTML.
