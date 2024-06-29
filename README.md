# diameter
`diameter` *column1* *column2*

get the graph diameter (also known as max path length or hop
size) of spanning-trees, from space-separated formats
similar to output of the irc `LINKS` command

*column1* and *column2* are the column indices to look at
for node names. note that these start from 0, contrary to
other utilities such as `awk` and `cut`.

## example
```sh
cargo run -Fbin 3 4 <<EOF
:fox. 364 xfnwtest sparrow. maned.wolf :2 bird
:fox. 364 xfnwtest feesh. otter. :2 wiggle wiggle
:fox. 364 xfnwtest services. fox. :1 Atheme IRC Services
:fox. 364 xfnwtest maned.wolf fox. :1 legs
:fox. 364 xfnwtest otter. fox. :1 squeak squeak
:fox. 364 xfnwtest fox. fox. :0 solanum fox server
EOF
```
results in: `4 hops between feesh. and sparrow.`

## bugs
- this was designed to handle a spanning-tree (trees with no
  loops permitted, ensuring a single path to any node) only,
  since this makes finding the longest path a lot easier.
  depth-first-search cannot handle loops, so we silently
  ignore already visited nodes to prevent infinite
  recursion. as the internal representation is not fully
  sorted, this will result in inconsistant and incorrect
  results depending on the input order.
- unreachable nodes will be ignored when searching. it
  currently starts from the first inputted node, though this
  should not be depended upon.

