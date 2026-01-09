use crate::address::Address;
use std::collections::HashMap;

/// A symbol (label) in the program
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub address: Address,
    pub is_local: bool,
}

/// Symbol table managing global and local labels
#[derive(Debug, Clone)]
pub struct SymbolTable {
    /// Global symbols (e.g., "main", "foo")
    globals: HashMap<String, Address>,
    /// Local symbols scoped by global label (e.g., ".LBB0_1" under "main")
    locals: HashMap<String, HashMap<String, Address>>,
    /// Current global scope for local label resolution
    current_global: Option<String>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            globals: HashMap::new(),
            locals: HashMap::new(),
            current_global: None,
        }
    }

    /// Define a global label
    pub fn define_global(&mut self, name: String, address: Address) {
        self.globals.insert(name.clone(), address);
        self.current_global = Some(name.clone());
        // Initialize local scope for this global
        self.locals.entry(name).or_insert_with(HashMap::new);
    }

    /// Define a local label
    /// - Labels like .L.str, .L.str.1 (LLVM static symbols) are stored as globals
    /// - Labels like .LBB0_1 (basic block labels) are stored in local scope
    pub fn define_local(&mut self, name: String, address: Address) -> Result<(), String> {
        // LLVM uses .L prefix for file-local static symbols (e.g., .L.str, .L.str.1)
        // These are visible throughout the file, so treat them as globals
        if name.starts_with(".L.") {
            self.globals.insert(name, address);
            return Ok(());
        }
        
        // Basic block labels (.LBB0_1) are truly local to a function
        if let Some(ref global) = self.current_global {
            self.locals
                .get_mut(global)
                .ok_or_else(|| format!("No local scope for global '{}'", global))?
                .insert(name, address);
            Ok(())
        } else {
            // If no global scope, store in globals as fallback
            self.globals.insert(name, address);
            Ok(())
        }
    }

    /// Resolve a label to an address
    pub fn resolve(&self, name: &str) -> Option<Address> {
        // First check globals (includes .L. static symbols)
        if let Some(addr) = self.globals.get(name).copied() {
            return Some(addr);
        }
        
        if name.starts_with('.') {
            // Local label - look in current global scope
            if let Some(ref global) = self.current_global {
                self.locals.get(global).and_then(|m| m.get(name).copied())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Resolve a label with explicit scope
    pub fn resolve_with_scope(&self, name: &str, scope: Option<&String>) -> Option<Address> {
        // First check globals (includes .L. static symbols)
        if let Some(addr) = self.globals.get(name).copied() {
            return Some(addr);
        }
        
        if name.starts_with('.') {
            // Local label - look in provided scope
            if let Some(global) = scope {
                self.locals.get(global).and_then(|m| m.get(name).copied())
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Get current global scope
    pub fn current_global(&self) -> Option<&String> {
        self.current_global.as_ref()
    }

    /// Get all global symbols
    pub fn globals(&self) -> &HashMap<String, Address> {
        &self.globals
    }

    /// Get all local symbols for a specific global
    pub fn locals_for(&self, global: &str) -> Option<&HashMap<String, Address>> {
        self.locals.get(global)
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        Self::new()
    }
}
