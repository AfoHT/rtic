# Data design

> The data design describes structures that reside within the
> software. Attributes and relationships between data objects dictate the
> choice of data structures.

## Overview of data flow

```dot process Structural layout
digraph {
    ranksep="0.3";
    nodesep="0.3";
    ratio=compress;
    concentrate=true;
    compound=true;
    rankdir=TB;
    node [shape = box];
    input [color = blue];
    output [color = green]
    subgraph cluster_macros {
        1 [ label = "cortex-m-rtfm" ];
        2 [ label = "rtfm-core" ];
        {rank=same; 1 2 }
        label = "RTFM";
        subgraph cluster_macrolib {
            label = "cortex-m-rtfm-macros";
            style=filled;
            maclib [label = "lib"]
            macanalyze [label = "analyze"]
            maccheck [label = "check"]
            maccodegen [label = "codegen"];
            {rank=same; maccheck macanalyze maccodegen }
        }
        subgraph cluster_syntax {
            label = "rtfm-syntax";
            style=filled;
            slib [label = "rtfm-syntax lib"];
            check optimize analyze parse;
            {rank=same; check optimize analyze parse}
        }
    }
    input -> 1;
    1 -> maclib;
    1 -> 2;
    maclib -> output;
    maclib -> macanalyze [label = "7", dir=both];
    maclib -> maccheck [label = "8", dir=both];
    maclib -> maccodegen [label = "9", dir=both];
    maclib -> slib [label = "6", dir=back];
    slib -> maclib [label = "1", dir=back];
      slib -> check [label = "3", dir=both];
      slib -> optimize [label = "4", dir=both];
      slib -> analyze [label = "5", dir=both];
      slib -> parse [label = "2", dir=both];
    parse -> check -> optimize -> analyze [style=invis]
    maccheck  -> macanalyze -> maccodegen [style=invis];
    {maccheck  macanalyze maccodegen } -> slib [style=invis];
    input -> 1 [style=invis];
    output -> 2 [style=invis];
    {rank=same; input output }
}
```


