//! Rust port of core graph-learning and network-analysis primitives from
//! [`scikit-network`](https://github.com/sknetwork-team/scikit-network).
//!
//! Graphs are represented as sparse adjacency matrices (`sprs::CsMat<f64>`).
//! Estimators follow scikit-learn-style `fit` / `predict` / `transform`
//! workflows where applicable.
//!
//! # Module index
//!
//! | Module | Algorithms |
//! |--------|------------|
//! | [`classification`](crate::classification) | Label propagation, diffusion, NN classifiers |
//! | [`clustering`](crate::clustering) | Louvain, Leiden, propagation clustering |
//! | [`data`](crate::data) | Edge-list / CSV / GraphML parsers, Netset loader |
//! | [`embedding`](crate::embedding) | Spring, ForceAtlas, spectral, SVD, random projection |
//! | [`gnn`](crate::gnn) | GNN layers, activations, classifiers |
//! | [`hierarchy`](crate::hierarchy) | Paris, Louvain hierarchy |
//! | [`linalg`](crate::linalg) | Sparse eig/SVD solvers, normalizers, operators |
//! | [`linkpred`](crate::linkpred) | Nearest-neighbor link prediction |
//! | [`path`](crate::path) | Shortest paths, BFS, distances |
//! | [`ranking`](crate::ranking) | PageRank, HITS, Katz, closeness, betweenness |
//! | [`regression`](crate::regression) | Diffusion regression |
//! | [`topology`](crate::topology) | Cliques, cores, cycles, triangles, structure |
//! | [`utils`](crate::utils) | Input checking, formatting, value conversion |
//! | [`visualization`](crate::visualization) | SVG graph and dendrogram rendering |
//!
//! # Further reading
//!
//! - [Repository README](https://github.com/nathandelara/sknetwork-rs/blob/main/README.md)
//! - [Upstream Python library](https://github.com/sknetwork-team/scikit-network)
//!
//! # Examples
//!
//! ```rust,no_run
//! use sknetwork_rs::ranking::pagerank::PageRank;
//! use sprs::CsMat;
//!
//! let adjacency = CsMat::<f64>::eye(4);
//! let mut algo = PageRank::default();
//! let _ = algo.fit_predict(&adjacency, None, None, None, false);
//! ```

#![warn(missing_docs)]

#[cfg(feature = "bench")]
#[doc(hidden)]
pub mod bench;
pub mod classification;
pub mod clustering;
pub mod data;
pub mod embedding;
pub mod gnn;
pub mod hierarchy;
pub mod linalg;
pub mod linkpred;
pub mod path;
pub mod ranking;
pub mod regression;
pub mod topology;
pub mod utils;
pub mod visualization;

#[cfg(test)]
mod contract_matrix;
