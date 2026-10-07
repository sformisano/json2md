<!-- review-summary {"mr":"https://gitlab.example/acme/widgets/-/merge_requests/12","reviewer_id":17} -->
### Round R4 · [44444444](https://gitlab.example/acme/widgets/-/commit/4444444444444444444444444444444444444444) · ready
<!-- review-round {"mode":"focused","policy_ids":["library-compatibility-policy"],"revision":{"base_sha":"1111111111111111111111111111111111111111","head_sha":"4444444444444444444444444444444444444444","start_sha":"1111111111111111111111111111111111111111"},"round_id":"R4","status":"completed"} -->
| ID | Finding | Severity | Status | Action |
| --- | --- | --- | --- | --- |
| F7 | [Preserve numeric types](https://gitlab.example/acme/widgets/-/merge_requests/12#note_701) | P2 | Fixed | Reply added |
| F10 | [Keep the context conversion focused](https://gitlab.example/acme/widgets/-/merge_requests/12#note_702) | P4 | Advisory | None |

<details>
<summary>Round R3 · <a href="https:&#x2f;&#x2f;gitlab.example&#x2f;acme&#x2f;widgets&#x2f;-&#x2f;commit&#x2f;3333333333333333333333333333333333333333">33333333</a> · changes required</summary>

<!-- review-round {"mode":"full","policy_ids":["code-structure-policy","library-compatibility-policy"],"revision":{"base_sha":"1111111111111111111111111111111111111111","head_sha":"3333333333333333333333333333333333333333","start_sha":"1111111111111111111111111111111111111111"},"round_id":"R3","status":"completed"} -->
| ID | Finding | Severity | Status | Action |
| --- | --- | --- | --- | --- |
| F7 | [Preserve numeric types](https://gitlab.example/acme/widgets/-/merge_requests/12#note_701) | P2 | Open | New thread |
| F10 | [Keep the context conversion focused](https://gitlab.example/acme/widgets/-/merge_requests/12#note_702) | P4 | Advisory | New thread |

</details>

<details>
<summary>Round R2 · <a href="https:&#x2f;&#x2f;gitlab.example&#x2f;acme&#x2f;widgets&#x2f;-&#x2f;commit&#x2f;2222222222222222222222222222222222222222">22222222</a> · ready</summary>

<!-- review-round {"mode":"full","policy_ids":["rust-error-contracts"],"revision":{"base_sha":"1111111111111111111111111111111111111111","head_sha":"2222222222222222222222222222222222222222","start_sha":"1111111111111111111111111111111111111111"},"round_id":"R2","status":"completed"} -->

</details>

<details>
<summary>Review metadata</summary>

| Field | Value |
| --- | --- |
| Latest attempted round | R4 |
| Review workflow | `codebase-policies-review` |
| Policy set | `rust-codebase-policies` |
| Review scope | Focused reassessment |
| Review depth | Standard |
| Reviewed range | [11111111](https://gitlab.example/acme/widgets/-/commit/1111111111111111111111111111111111111111) → [44444444](https://gitlab.example/acme/widgets/-/commit/4444444444444444444444444444444444444444) |

**Policies across all review rounds**

| Policy | Applicable rounds | Comments across MR |
| --- | --- | --- |
| Code structure | R3 | 1 |
| Library compatibility | R3, R4 | 1 |
| Rust Error Contracts | R2 | 0 |

Applicable rounds record corroborated selection. Counts cover this reviewer's distinct inline findings across the MR, including fixed and withdrawn findings. Replies and repeated rounds do not add comments.

</details>
