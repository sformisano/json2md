[P2] F7: Preserve numeric types during rendering

With `serde_json/arbitrary_precision` enabled, rendering `{"count":42}` treats `count` as an object and breaks `count + 1`. Convert JSON variants into native template values before rendering.

---

*Policy: Library compatibility · Dependency features preserve supported numeric behavior.*
<!-- review-finding {"action_id":"R3-F7-create","finding_id":"F7","round_id":"R3"} -->
