pub mod claude_api;
pub mod claude_code_search;
pub mod router;

use claude_api::ask_claude_api;
use claude_code_search::search_obsidian_vault;
use router::should_search_vault;
