\
# IV. Algorithms (pseudocode, complexity, expected effects)

## A1. Capacity estimation from embeddings (α_mf via anchors; α_sim via separability)
**Input:** For each concept \(\mu\), samples \(X_\mu=\{x_{\mu i}\in\mathbb{R}^d\}\).  
**Output:** \(\hat{R}_M, \hat{D}_M, \hat{\alpha}_{\text{mf}}, \hat{\alpha}_{\text{sim}}\).

**Pseudocode (anchor‑based MFTMA):**
```
for each manifold μ:
  c_μ ← mean(X_μ)
  # Solve max-margin over convex hull to get anchor ˜x_μ
  ˜x_μ ← AnchorSolver(X_μ)         # Manifold SVM/KKT on convex hull
  RM_μ ← ||˜x_μ||
  V_μ ← cov(proj_random_dirs_onto_span(˜x_μ))   # anchor projection covariance
  DM_μ ← participation_ratio(V_μ)  # (∑λ)^2 / ∑λ^2
RM ← mean_μ RM_μ ; DM ← mean_μ DM_μ
α_mf ← SphereCapacityApprox(RM, DM)
# α_sim: empirical separability
for N* in decreasing subspace dims of R^d:
  Z ← PCA/project(X_all, N*)
  if linear_SVM_separates(Z manifolds): return α_sim = P / N*
```
**Complexity.** AnchorSolver over \(m_\mu\) points: \(\tilde{O}(m_\mu d^2)\) with modern QP/Frank–Wolfe; SVM grid search for \(\alpha_{\text{sim}}\): \(O(\log d)\) SVM fits.  
**Expected effects.** Tracks \(R_M\sqrt{D_M}\) and \(\rho_{CC}\); decreasing any of these increases \(\alpha\). (A1, A2)

## A2. Hierarchy‑aware flattening / linearization
Use a preprocessor \(\Phi\) with objectives:
- minimize curvature and \(R_M, D_M\) (e.g., FlatNet loss),
- preserve class topology (centroid ordering, nearest‑neighbors).

**Pseudocode:**
```
input: embeddings X, labels (concept IDs), graph context G
repeat until convergence:
  X' ← Encoderθ(X, G)       # text+graph hybrid encoder
  L_flat ← curvature_loss(X') + λ1 * nuclear_norm(G X')    # MMCR term
  L_topo ← triplet_losses_on_centroids(X')
  θ ← θ - η ∇(L_flat + λ2 L_topo)
return Φ(x) = Encoderθ(x, G)
```
**Complexity.** Per step dominated by encoder forward/backward; SVD for nuclear norm on batches.  
**Effect.** \(R_M↓, D_M↓, \rho_{CC}↓\Rightarrow \alpha↑\). (S7, A2)

## A3. Graph‑aware embedding updates (OBO synonyms/ancestors)
Augment each concept manifold by sampling synonyms, definitions, and ancestor‑context templates; learn with contrastive objectives; re‑estimate capacity.

**Pseudocode:**
```
for concept μ:
  S_μ ← {synonyms, preferred name, definition sentences}
  A_μ ← {k-hop ancestor/child labels}
  V_μ ← text_encode(S_μ ⊕ prompts(A_μ))
Update encoder with contrastive InfoNCE on (V_μ, μ)
```
**Effect.** May reduce centroid ambiguity (ρ_CC↓); monitor RM (noisy synonyms can inflate within‑manifold variance). (S9)

## A4. Capacity‑preserving community detection
```
input: ontology graph G=(V,E)
P_leiden ← Leiden(G; resolution γ)
assert all communities connected(P_leiden)   # algorithmic guarantee
use P_leiden to define neighborhood sampling for graph encoder
```
**Effect.** Avoids disconnected “communities” that spuriously break manifolds (RM, DM inflation). (S4)
