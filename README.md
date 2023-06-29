Navitain
============

WIP UNDER CONSTRUCTION

Goal: a Kubernetes Operator that inpsects readiness and liveliness probes of
all pods in a cluster and can respond to queries with suggestions / predictions.

It is interested in /activation/prometheus style info as well as reading the 
yaml of deployments to know what paths and ports to find probe data.

Start with the topology POC from: https://www.youtube.com/watch?v=feBYxeO-3cY

Written in Rust and persisting to sqlite.
