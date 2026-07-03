# sknetwork-rs

Rust port of core graph-learning and network-analysis primitives from
[scikit-network](https://github.com/sknetwork-team/scikit-network).

The crate targets **sparse, matrix-first** workflows: adjacency graphs are
represented as `sprs::CsMat<f64>`, and estimators follow scikit-learn-style
`fit` / `predict` / `transform` patterns where applicable.

## Quick start

Add to `Cargo.toml`:

```toml
[dependencies]
sknetwork-rs = "0.1"
sprs = "0.11"
ndarray = "0.16"
```

PageRank on a 4-node identity graph:

```rust
use sknetwork_rs::ranking::pagerank::PageRank;
use sprs::CsMat;

let adjacency = CsMat::<f64>::eye(4);
let mut algo = PageRank::default();
let scores = algo.fit_predict(&adjacency, None, None, None, false).unwrap();
```

## Module map

| Module | Purpose | Primary entry points |
|--------|---------|----------------------|
| [`classification`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/classification/index.html) | Supervised label propagation, diffusion, NN classifiers | `propagation::Propagation`, `nn::NNClassifier` |
| [`clustering`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/clustering/index.html) | Community detection | `louvain::Louvain`, `leiden::Leiden` |
| [`data`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/data/index.html) | Graph I/O and parsing | `parse::from_edge_list`, `load::load_netset` |
| [`embedding`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/embedding/index.html) | Layout and low-rank embeddings | `spring::Spring`, `svd::SVD`, `spectral::Spectral` |
| [`gnn`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/gnn/index.html) | Graph neural networks | `gnn_classifier::GNNClassifier` |
| [`hierarchy`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/hierarchy/index.html) | Hierarchical clustering | `paris::Paris`, `louvain_hierarchy::LouvainHierarchy` |
| [`linalg`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/linalg/index.html) | Sparse linear algebra | `svd_solver`, `symmetric_eigsh`, `polynome` |
| [`linkpred`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/linkpred/index.html) | Link prediction | `nn::NN` |
| [`path`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/path/index.html) | Shortest paths and search | `shortest_path`, `distances`, `search` |
| [`ranking`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/ranking/index.html) | Centrality and ranking | `pagerank::PageRank`, `hits::HITS`, `katz::Katz` |
| [`regression`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/regression/index.html) | Diffusion regression | `diffusion::Diffusion` |
| [`topology`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/topology/index.html) | Structural graph analysis | `cliques`, `core`, `cycles`, `triangles` |
| [`utils`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/utils/index.html) | Shared input helpers | `check`, `format`, `values` |
| [`visualization`](https://docs.rs/sknetwork-rs/latest/sknetwork_rs/visualization/index.html) | SVG graph and dendrogram rendering | `graphs`, `dendrograms` |

## Documentation

- **API reference (rustdoc):** <https://docs.rs/sknetwork-rs/latest/sknetwork_rs/>
- **Repository:** <https://github.com/nathandelara/sknetwork-rs>
- **Upstream Python library:** <https://github.com/sknetwork-team/scikit-network>

## Python parity

This crate is a port of the Python `scikit-network` library. Module paths are
kept close to their Python counterparts (e.g. `sknetwork.ranking.PageRank` →
`sknetwork_rs::ranking::pagerank::PageRank`).

## License

BSD-3-Clause — same as scikit-network.
