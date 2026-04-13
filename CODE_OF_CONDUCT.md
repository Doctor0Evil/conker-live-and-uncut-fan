# Code of Conduct

## 1. Purpose

This project unifies GAMEMODE.ai and Nintendoor64 into a schema‑first, knowledge‑graph‑driven, CLI‑orchestrated platform for AI‑assisted, professional‑grade game development across retro consoles and modern engines.[file:6] To reach that goal, we need a community that is respectful, inclusive, safety‑conscious, and aligned with the project’s determinism, licensing, and contract‑first principles.[file:4][file:6]

This Code of Conduct applies to all project spaces: Git repositories, issues, pull requests, CI systems, documentation, discussions, and any community channels associated with this work.[file:8]

## 2. Our Standards

We commit to a community where participants:

- Treat others with respect and assume good faith, regardless of background, experience level, or role.  
- Give and receive technical feedback that is specific, actionable, and focused on code, schemas, and designs, not on people.  
- Respect the project’s safety constraints: determinism requirements, hardware budgets, schema contracts, and CI gates are non‑negotiable technical standards, not optional guidelines.[file:4][file:6]  
- Acknowledge and respect the legal boundaries around fan projects and third‑party IP; original code and schemas may be open‑licensed, but original Conker or other proprietary IP is explicitly out of scope.[file:6]  

The following behaviors are unacceptable in any project space:

- Harassment, hate speech, personal attacks, or derogatory comments.  
- Discriminatory language or conduct related to race, ethnicity, gender, sexuality, disability, religion, or any other protected characteristic.  
- Deliberate attempts to bypass project safeguards (e.g., determinism guards, schema validation, hardware budget checks) or to pressure maintainers to merge unsafe changes.[file:4][file:6]  
- Knowingly introducing or promoting content that violates licenses, platform terms, or the explicit IP boundaries of this repository.[file:6]

## 3. Technical Conduct and Safety

Because this project is explicitly contract‑driven and AI‑orchestrated, technical conduct is part of community conduct.[file:4][file:6]

Contributors are expected to:

- Treat JSON Schemas, Rust types, and CLI contracts as the source of truth. Changes to schemas or protocol surfaces must be deliberate, documented, and versioned.[file:4][file:6][file:8]  
- Respect determinism and safety invariants: no unauthorized nondeterministic APIs in deterministic crates, no unsafe patch regions, no unbounded resource use that violates platform constraints.[file:4][file:6]  
- Keep build and orchestration tools (e.g., Starzip, Sonia, retro‑cli, gamemodeai‑build, gamemodeai‑kg, gamemodeai‑session) deterministic, JSON‑in/JSON‑out, and CI‑friendly.[file:4][file:6][file:8]  
- Ensure that AI‑assisted changes are validated: all AI‑generated artifacts must pass schema validation, invariants, and CI before being proposed for merge.[file:4][file:6]

Intentionally circumventing these safeguards, or encouraging others to do so, is considered a conduct violation.

## 4. Licensing and Intellectual Property

The repository’s LICENSE file defines the legal terms for original code, schemas, and documentation.[file:6] In addition to those terms:

- Do not contribute original game assets, decompiled code, or data that you do not have the right to share.  
- Treat all references to Conker and other legacy titles as research context only; this project does not ship or redistribute original IP, and all examples must stay within fair‑use and fan‑work boundaries as defined by the maintainers.[file:6]  
- When in doubt about whether a contribution is legally acceptable, ask in a discussion thread or contact a maintainer before submitting a pull request.

## 5. Scope of AI‑Assisted Contributions

AI‑generated code, schemas, documentation, and assets are welcome, provided they follow the same standards as human‑authored contributions.[file:4][file:6][file:8]

In particular:

- AI‑assisted changes must respect determinism, schema contracts, hardware constraints, and session invariants.  
- Contributors remain responsible for reviewing AI output, understanding what it does, and ensuring it matches the project’s style and safety rules before opening a PR.  
- Any automated tooling that writes to this repository must run through the same CI pipelines and validation steps as manual contributions.

## 6. Enforcement Responsibilities

Project maintainers are responsible for clarifying and enforcing this Code of Conduct.[file:8] They may:

- Remove, edit, or reject comments, commits, code, schemas, or other contributions that violate this Code.  
- Temporarily or permanently restrict a contributor’s ability to participate, when necessary to protect the community or the project.  
- Clarify how this Code applies to new technical surfaces (e.g., new CLIs, schemas, or AI‑integration points) as the project evolves.[file:4][file:6]

Enforcement decisions will prioritize the safety, legal compliance, and long‑term health of the project over short‑term convenience.

## 7. Reporting and Incident Handling

If you experience or witness unacceptable behavior:

- Prefer in‑project channels specified in the repository (e.g., SECURITY.md, SUPPORT.md, or a designated contact email) if present.  
- If no dedicated channel exists, you may open a private issue or contact a maintainer directly using the contact information listed in the repository profile or documentation.  

Reports should include:

- A description of what happened.  
- Where and when it occurred (links, timestamps, branches, CI runs, etc.).  
- Any additional context that may help maintainers understand and address the issue.

Maintainers will review reports as promptly as feasible, may request additional information, and will decide on actions in line with this Code and the project’s legal and safety obligations.[file:8]

## 8. Consequences

Violations of this Code of Conduct may result in actions including, but not limited to:

- A request for correction or clarification.  
- Removal of offending comments, code, or artifacts.  
- Temporary or permanent loss of contribution or maintainer privileges.  
- Escalation to platform administrators or other authorities, if necessary.

The exact response will depend on the severity and frequency of the behavior, and on its impact on community safety and project integrity.

## 9. Evolution of This Code

As the GAMEMODE.ai and Nintendoor64 ecosystem grows and new schemas, CLIs, and vertical slices come online, this Code of Conduct may be updated to reflect new realities and risks.[file:4][file:6] Changes will be version‑controlled in this file and should be discussed openly in issues or discussions when they materially affect contributor expectations.

By contributing to this repository, you agree to abide by this Code of Conduct and to help maintain a respectful, lawful, and technically disciplined environment for everyone involved.
