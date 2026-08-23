const VAULT_TRIGGERS: [&str; 8] = [
    "note", "notes", "vault", "wrote", "written", "remember", "obsidian", "my notes",
];

const needs_vault_access(input: &str) -> bool {
    let input_lower = input.to_lowercase();
    VAULT_TRIGGERS.iter().any(|&trigger| input_lower.contains(trigger))
}
