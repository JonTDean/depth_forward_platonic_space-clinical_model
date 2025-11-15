# III. Formal Notes (definitions, lemmas, conditions, sketches)

**Definition (Classification / manifold capacity).**
For an ensemble of \(P\) object manifolds in \(\mathbb{R}^N\) with random binary labels, the **capacity** is \(\alpha_c := P_c/N\), where \(P_c\) is the maximal \(P\) such that there exists a hyperplane linearly separating the labeled manifolds. (A1, A2)

**Mean‑field formulation & anchors.**
Let \(\mathcal{S}_\mu\) be the convex hull of manifold \(\mu\). Define the support function \(g_{\mathcal{S}_\mu}(v) = \min_{s\in \mathcal{S}_\mu} v\cdot s\).
Mean‑field theory yields an inverse capacity
\[\alpha^{-1} = \mathbb{E}_{T}[F(T)],\quad F(T) := \min_{V}\|V-T\|^2 \;\text{s.t.}\; g_{\mathcal{S}_\mu}(V) \ge 0 \;\forall \mu,\]
with KKT conditions implying a representation of the separating weight as a sum of **anchor points** \(w = \sum_\mu \lambda_\mu y_\mu \tilde{s}_\mu\), where each \(\tilde{s}_\mu \in \mathcal{S}_\mu\) is the minimizing support for \(\mu\). (A2)

**Effective radius and dimension.**
Write \(\tilde{s}_\mu(T)\) for the anchor associated with random \(T\). Define
\(\displaystyle R_M := \mathbb{E}_T\|\tilde{s}_\mu(T)\|\) and an **effective dimension**
\(\displaystyle D_M := \frac{(\sum_i \lambda_i)^2}{\sum_i \lambda_i^2}\) (participation ratio of the anchor‑projection covariance). Capacity scales inversely with \(R_M\sqrt{D_M}\) under a DM‑ball approximation. (A1, A2)

**Centroid correlation (low‑rank common structure).**
Let \(c_\mu\) be manifold centroids. Define \( \rho_{CC} := \mathbb{E}_{\mu\ne\nu}\cos(c_\mu, c_\nu)\).
If centroids share a rank‑\(K\ll P\) component, projecting onto its nullspace increases effective capacity without altering within‑manifold geometry. (A1)

**Lemma (Sphere / ball approximation).**
For random manifold centers/orientations, replacing each manifold by a \(D_M\)‑dimensional ball of radius \(R_M\) yields \(\alpha_{\text{MFT}} \approx \alpha_{\text{Ball}}(R_M,D_M)\). (A1, A2)

**Curvature and flattening.**
A mapping \(\Phi:\mathbb{R}^N\!\to\!\mathbb{R}^{N'}\) is **flattening** if it decreases manifold curvature while approximately preserving pairwise class topology and centroids; empirical and algorithmic constructions (e.g., FlatNet) reduce \(R_M\) and \(D_M\) and increase separability. (S7, S3)

**Algebraic / causal manifolds.**
If latent manifolds are algebraic sets \(\mathcal{M}=\{x\mid p_j(x)=0\}\), their **vanishing ideal** \(I(\mathcal{M})=\{p: p|_{\mathcal{M}}=0\}\) provides a compact description and supports distance/consistency tests and counterfactual reasoning in structured models. (S8)
