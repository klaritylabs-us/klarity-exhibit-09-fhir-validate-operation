# FHIR `$validate` operation — Rust

Small library and CLI that sanity-check **`$validate`** request bodies before they hit your server or CI harness: a **`Parameters`** wrapper with a **`resource`** parameter, or a direct resource JSON (`Patient`, `Observation`, etc.) — the shapes servers advertise but often implement inconsistently.

Synthetic fixtures only. Not ONC certification, not a substitute for the HL7 FHIR Validator or Inferno, and not legal/clinical advice.

Maintained by [Klarity Labs](https://klaritylabs.ai/) as open reference code for U.S. healthcare FHIR interoperability work.

**Exhibit 09** — [FHIR exhibit catalog](https://github.com/klaritylabs-us/klarity-fhir-exhibits) · [klaritylabs-us](https://github.com/klaritylabs-us)


## How this relates to other tools

| Tool class | Role |
|------------|------|
| **HL7 FHIR Validator**, **HAPI**, **Firely** | Full validation, servers, SDKs, IG packages |
| **Inferno**, **Touchstone**, vendor cert suites | **Conformance** against test plans |
| **This exhibit** | **Pre-flight construction** on synthetic fixtures—catch shape mistakes before wire traffic |

Conformance tools test whether your **API behaves**; exhibits focus on whether your **payloads and pipes are built** to survive real partners.

## Specifications cited

- [FHIR `$validate` operation](http://hl7.org/fhir/R4/resource-operation-validate.html)

## What it checks

- `resourceType` is **`Parameters`**, **`Patient`**, or **`Observation`** (direct POST body)
- For **`Parameters`**, at least one `parameter` entry with `name` **`resource`**
- JSON must parse; rules are structural, not full profile validation

| Problem | What breaks |
|--------|-------------|
| **Parameters without `resource`** | Server returns 400 or validates the wrong payload |
| **Wrong top-level type** | Client posted Bundle or OperationOutcome instead of validate input |
| **Empty parameter list** | Silent pass on some stacks, hard fail on others |

## Run it

Needs [Rust](https://www.rust-lang.org/tools/install).

```bash
cargo test
cargo run --bin validate-operation -- examples/valid-validate-params.json
cargo run --bin validate-operation -- examples/invalid-no-resource.json
```

Exit `0` if no errors, `1` if validation failed, `2` for bad usage or JSON.

## Sample output

One command:

```bash
cargo run --bin validate-operation -- examples/valid-validate-params.json
```

**Pass** (`examples/valid-validate-params.json`) — exit code `0`:

```json
[]
```

**Fail** (`examples/invalid-no-resource.json`) — exit code `1` (issues array; more entries may appear):

```json
[
  {
    "severity": "error",
    "path": "Parameters.parameter",
    "detail": "validate call needs parameter name resource",
    "rule_id": "validate-resource-param"
  }
]
```


## Layout

- `src/validate.rs` — rules
- `examples/` — valid and broken `$validate` payloads for tests and demos

## Inferno vs this repo

Inferno exercises **`$validate` behavior** on a live FHIR server. This repo is **request shape** — the JSON your client sends before the operation reaches the endpoint.

## License

MIT — see [LICENSE](./LICENSE).

**Need this on your payloads or API?** 20-minute FHIR Scan — sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)

## Contact

sk@klaritylabs.ai · [klaritylabs.ai](https://klaritylabs.ai/)
