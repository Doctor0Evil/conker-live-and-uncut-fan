# File_Generation_Guidelines.md
## Destination: conker-live-and-uncut-fan/Docs/AI_Chat_Context/File_Generation_Guidelines.md

# File Generation Guidelines — GAMEMODE.ai Contract

## 1. Purpose
This document defines the exact contract AI systems must follow when generating files for the `conker-live-and-uncut-fan` repository. It ensures consistency, validation-readiness, and seamless CI integration.

## 2. Generation Contract
Every generated file must satisfy:
- ✅ **Complete Implementation:** No stubs, no placeholders, no `TODO` without concrete next-step mapping
- ✅ **Valid Syntax:** Compilable/parsable by target toolchain on first pass
- ✅ **Path Declaration:** Explicit `# Filename` + `## Destination: path` header
- ✅ **Schema Compliance:** All configs, structs, and enums match repository JSON schemas
- ✅ **Determinism Hooks:** Gameplay state exposed for replay/validation hashing
- ✅ **Next Objectives:** 3 actionable, dependency-resolved next steps at EOF
- ✅ **Zero Artifacts:** No markdown citation tags, no AI conversational text, no tool metadata

## 3. Step-by-Step Workflow
1. **Parse Prompt:** Extract target path, engine, role, dependencies, constraints
2. **Load Context:** Ingest `System_Prompts_GAMEMODE_ai.md`, `Conker_Lore_Base.txt`, KG index
3. **Resolve Dependencies:** Verify referenced files exist or will be generated in sequence
4. **Generate Content:** Write full file adhering to engine/language standards
5. **Inject Validation Hooks:** Add replication tags, schema version fields, determinism comments
6. **Append Next Steps:** List 3 concrete, shippable follow-up tasks
7. **Output:** Return single markdown block ready for `git add`

## 4. Output Template
```markdown
# [filename.ext]
## Destination: [repo-relative/path/to/filename.ext]

[Full file content here — valid syntax, complete implementation]

## Next Objectives
- [ ] Generate [dependent_file.ext] at [path] to resolve [dependency]
- [ ] Implement CI validation for [feature] using [schema/tool]
- [ ] Register [SystemNode_ID] in `Docs/TechDesign/04_Knowledge_Graph_Index.md`
```

## 5. Validation Checklist
Before outputting, AI must self-verify:
- [ ] File compiles/parses syntactically
- [ ] All `#include`, `import`, or `require` statements reference existing KG nodes
- [ ] Networked fields marked with engine replication attributes
- [ ] Config structs include `schema_version` and validation hooks
- [ ] Legal/fan-safe boundaries explicitly respected
- [ ] No conversational filler or tool artifacts present

## 6. Error & Fallback Handling
If a constraint cannot be met:
- **Do not output partial code**
- Return a structured error block:
  ```markdown
  ## Generation Blocked
  - Reason: [specific constraint violation]
  - Required: [missing schema/dependency/permission]
  - Resolution: [concrete next step to unblock]
  ```
- Log violation for CI review; do not proceed to file generation

## 7. Examples

### ✅ Correct Output
```markdown
# CLUHeistGameMode.cpp
## Destination: Engine/Unreal/Source/Private/Multiplayer/Heist/CLUHeistGameMode.cpp

// Full, valid UE5 C++ implementation with replication, deterministic state machine, and next objectives
// ...
```

### ❌ Incorrect Output
```markdown
Sure! Here's the file you requested:

// CLUHeistGameMode.cpp
// TODO: implement this later
// ...
```

## 8. CI Integration
Generated files are automatically:
- Parsed by `Tools/indexing/repo_index_generator.rs`
- Validated against `schemas/*.json`
- Checked for determinism compliance via replay hash tests
- Registered in Knowledge Graph for future navigation

---

*This contract is binding for all AI-assisted generation in the repository. Adhere strictly.*
