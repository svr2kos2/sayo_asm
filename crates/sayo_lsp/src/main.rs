use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use sayo_parser::{parse, ParseError};
use sayo_ast::{Mnemonic, Item, Register};
use sayo_sema::{SemanticChecker, SemanticError};
use std::collections::HashMap;

// Helper function to convert byte offset to line/column
fn position_from_offset(text: &str, offset: usize) -> (usize, usize) {
    let mut line = 0;
    let mut col = 0;
    for (i, ch) in text.chars().enumerate() {
        if i >= offset {
            break;
        }
        if ch == '\n' {
            line += 1;
            col = 0;
        } else {
            col += 1;
        }
    }
    (line, col)
}

// Format hover text for instruction with full metadata
fn format_instruction_hover(mnemonic: &Mnemonic) -> String {
    let metadata = mnemonic.metadata();
    let (desc, note) = mnemonic.description();
    let mnem_str = format!("{:?}", mnemonic);
    
    let mut text = format!("**{}**\n\n", mnem_str);
    
    // Metadata section
    text.push_str("---\n\n");
    text.push_str(&format!("**Opcode:** 0x{:02X} ({})\n\n", metadata.opcode, metadata.opcode));
    text.push_str(&format!("**Length:** {} byte(s)\n\n", metadata.length));
    
    // Operands
    if !metadata.operands.is_empty() {
        let mut operands = Vec::new();
        for (idx, op) in metadata.operands.iter().enumerate() {
            let param_name = match idx {
                0 => "i",
                1 => "j",
                2 => "k",
                _ => "?",
            };
            let op_type_str = format!("{:?}", op.op_type);
            let rw = if op.is_write { "W" } else { "R" };
            operands.push(format!("{}: {} ({})", param_name, op_type_str, rw));
        }
        text.push_str(&format!("**Operands:** {}\n\n", operands.join(", ")));
    } else {
        text.push_str("**Operands:** None\n\n");
    }
    
    // Description
    if !desc.is_empty() {
        text.push_str("---\n\n");
        text.push_str(&format!("**Description:** {}\n\n", desc));
    }
    
    // Note
    if !note.is_empty() {
        text.push_str(&format!("**Note:** {}\n\n", note));
    }
    
    text
}

// Format hover text for register with full metadata
fn format_register_hover(register: &Register) -> String {
    let metadata = register.metadata();
    let description = register.description();
    let reg_str = format!("{:?}", register);
    
    let mut text = format!("**{}**\n\n", reg_str);
    
    text.push_str("---\n\n");
    text.push_str(&format!("**Index:** 0x{:02X}\n\n", metadata.index));
    text.push_str(&format!("**Bit Width:** {} bits\n\n", metadata.width.bits()));
    
    let access_str = match metadata.access {
        sayo_ast::RegAccess::R => "Read-only",
        sayo_ast::RegAccess::W => "Write-only",
        sayo_ast::RegAccess::RW => "Read/Write",
    };
    text.push_str(&format!("**Access:** {}\n\n", access_str));
    
    if !description.is_empty() {
        text.push_str("---\n\n");
        text.push_str(&format!("**Description:** {}\n\n", description));
    }
    
    text
}

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: tokio::sync::RwLock<HashMap<String, String>>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> tower_lsp::jsonrpc::Result<InitializeResult> {
        Ok(InitializeResult {
            server_info: Some(ServerInfo {
                name: "Sayo Assembly Language Server".to_string(),
                version: Some("0.1.0".to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![".".to_string(), " ".to_string(), "*".to_string()]),
                    ..Default::default()
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                semantic_tokens_provider: Some(
                    SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(
                        SemanticTokensRegistrationOptions {
                            text_document_registration_options: {
                                TextDocumentRegistrationOptions {
                                    document_selector: Some(vec![DocumentFilter {
                                        language: Some("sayo-asm".to_string()),
                                        scheme: Some("file".to_string()),
                                        pattern: None,
                                    }]),
                                }
                            },
                            semantic_tokens_options: SemanticTokensOptions {
                                work_done_progress_options: WorkDoneProgressOptions::default(),
                                legend: SemanticTokensLegend {
                                    token_types: vec![
                                        SemanticTokenType::STRING,     // 0: directive
                                        SemanticTokenType::FUNCTION,   // 1: label
                                        SemanticTokenType::VARIABLE,   // 2: register
                                        SemanticTokenType::KEYWORD,    // 3: instruction
                                        SemanticTokenType::NUMBER,     // 4: immediate
                                        SemanticTokenType::COMMENT,    // 5: comment
                                        SemanticTokenType::MACRO,      // 6: unknown/error
                                    ],
                                    token_modifiers: vec![],
                                },
                                range: Some(false),
                                full: Some(SemanticTokensFullOptions::Bool(true)),
                            },
                            static_registration_options: StaticRegistrationOptions::default(),
                        },
                    ),
                ),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "Sayo LSP server initialized!")
            .await;
    }

    async fn shutdown(&self) -> tower_lsp::jsonrpc::Result<()> {
        Ok(())
    }

    async fn semantic_tokens_full(&self, params: SemanticTokensParams) -> tower_lsp::jsonrpc::Result<Option<SemanticTokensResult>> {
        let uri = params.text_document.uri.to_string();
        
        let doc_map = self.document_map.read().await;
        let text = match doc_map.get(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };
        
        let mut tokens_data = Vec::new();
        
        match parse(text) {
            Ok(program) => {
                let mut prev_line = 0;
                let mut prev_col = 0;
                
                for item in &program.items {
                    let item_inner = &item.node;
                    let span = &item.span;
                    let (line, col) = position_from_offset(text, span.start);
                    
                    match item_inner {
                        Item::Directive(_) => {
                            let delta_line = line - prev_line;
                            let delta_col = if delta_line == 0 { col - prev_col } else { col };
                            
                            tokens_data.push(SemanticToken {
                                delta_line: delta_line as u32,
                                delta_start: delta_col as u32,
                                length: (span.end - span.start) as u32,
                                token_type: 0,
                                token_modifiers_bitset: 0,
                            });
                            
                            prev_line = line;
                            prev_col = col;
                        }
                        Item::Instruction(inst) => {
                            let mnem_text = format!("{:?}", inst.mnemonic);
                            let mnem_len = mnem_text.len();
                            
                            let delta_line = line - prev_line;
                            let delta_col = if delta_line == 0 { col - prev_col } else { col };
                            
                            tokens_data.push(SemanticToken {
                                delta_line: delta_line as u32,
                                delta_start: delta_col as u32,
                                length: mnem_len as u32,
                                token_type: 3,
                                token_modifiers_bitset: 0,
                            });
                            
                            prev_line = line;
                            prev_col = col;
                            
                            for operand in &inst.operands {
                                let op_span = &operand.span;
                                let (op_line, op_col) = position_from_offset(text, op_span.start);
                                
                                let delta_line = op_line - prev_line;
                                let delta_col = if delta_line == 0 { op_col - prev_col } else { op_col };
                                
                                let token_type = match &operand.node {
                                    sayo_ast::Operand::Register(_) => 2,
                                    sayo_ast::Operand::Immediate(_) => 4,
                                    sayo_ast::Operand::Label(_) => 1,
                                };
                                
                                tokens_data.push(SemanticToken {
                                    delta_line: delta_line as u32,
                                    delta_start: delta_col as u32,
                                    length: (op_span.end - op_span.start) as u32,
                                    token_type,
                                    token_modifiers_bitset: 0,
                                });
                                
                                prev_line = op_line;
                                prev_col = op_col;
                            }
                        }
                        Item::Label(_) => {
                            let delta_line = line - prev_line;
                            let delta_col = if delta_line == 0 { col - prev_col } else { col };
                            
                            tokens_data.push(SemanticToken {
                                delta_line: delta_line as u32,
                                delta_start: delta_col as u32,
                                length: (span.end - span.start) as u32,
                                token_type: 1,
                                token_modifiers_bitset: 0,
                            });
                            
                            prev_line = line;
                            prev_col = col;
                        }
                    }
                }
            }
            Err(_) => return Ok(None),
        }
        
        Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data: tokens_data,
        })))
    }

    async fn completion(&self, params: CompletionParams) -> tower_lsp::jsonrpc::Result<Option<CompletionResponse>> {
        let mut items = Vec::new();

        // Get the document and current line to provide context-aware completion
        let uri = params.text_document_position.text_document.uri.to_string();
        let position = params.text_document_position.position;
        
        let doc_map = self.document_map.read().await;
        let text = doc_map.get(&uri);
        
        // Extract the word being typed to determine replacement range
        let (replace_start, replace_end, _typed_prefix) = if let Some(text) = text {
            let lines: Vec<&str> = text.lines().collect();
            if let Some(line) = lines.get(position.line as usize) {
                let chars: Vec<char> = line.chars().collect();
                let cursor_pos = position.character as usize;
                
                if cursor_pos > chars.len() {
                    (position.character, position.character, String::new())
                } else {
                    // Find the start of the word (including '*')
                    let mut start = cursor_pos;
                    while start > 0 {
                        let prev_char = chars[start - 1];
                        if prev_char.is_alphanumeric() || prev_char == '_' || prev_char == '*' {
                            start -= 1;
                        } else {
                            break;
                        }
                    }
                    
                    // Find the end of the word
                    let mut end = cursor_pos;
                    while end < chars.len() {
                        let curr_char = chars[end];
                        if curr_char.is_alphanumeric() || curr_char == '_' {
                            end += 1;
                        } else {
                            break;
                        }
                    }
                    
                    let prefix: String = chars[start..cursor_pos].iter().collect();
                    (start as u32, end as u32, prefix)
                }
            } else {
                (position.character, position.character, String::new())
            }
        } else {
            (position.character, position.character, String::new())
        };
        
        // Determine what kind of completion to provide based on context
        let is_likely_register = if let Some(text) = text {
            let lines: Vec<&str> = text.lines().collect();
            if let Some(line) = lines.get(position.line as usize) {
                // Check if we're after a comma (likely second operand) or after instruction
                let before_cursor = &line[..position.character.min(line.len() as u32) as usize];
                before_cursor.contains(',') || before_cursor.trim().split_whitespace().count() > 1
            } else {
                false
            }
        } else {
            false
        };

        // Create the replacement range
        let replace_range = Range {
            start: Position {
                line: position.line,
                character: replace_start,
            },
            end: Position {
                line: position.line,
                character: replace_end,
            },
        };

        // Add all mnemonics as completion items (Single source of truth)
        if !is_likely_register {
            for mnemonic in Mnemonic::all_variants() {
                let (desc, note) = mnemonic.description();
                let detail = if !note.is_empty() {
                    format!("{} - {}", desc, note)
                } else {
                    desc.to_string()
                };
                
                let label = format!("{}", mnemonic);
                items.push(CompletionItem {
                    label: label.clone(),
                    kind: Some(CompletionItemKind::FUNCTION),
                    detail: Some(detail),
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format_instruction_hover(&mnemonic),
                    })),
                    text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                        range: replace_range,
                        new_text: label.clone(),
                    })),
                    filter_text: Some(label),
                    ..Default::default()
                });
            }
        }

        // Add all registers as completion items (Single source of truth)
        for (label, register) in Register::common_variants() {
            let desc = register.description();
            
            items.push(CompletionItem {
                label: label.to_string(),
                kind: Some(CompletionItemKind::VARIABLE),
                detail: Some(desc.to_string()),
                documentation: Some(Documentation::MarkupContent(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: format_register_hover(&register),
                })),
                text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                    range: replace_range,
                    new_text: label.to_string(),
                })),
                filter_text: Some(label.to_string()),
                ..Default::default()
            });
        }

        // Add GL_0 to GL_63 registers
        for i in 0..64 {
            let label = format!("GL_{}", i);
            if let Some(r) = Register::from_str(&label) {
                let desc = r.description();
                
                items.push(CompletionItem {
                    label: label.clone(),
                    kind: Some(CompletionItemKind::VARIABLE),
                    detail: Some(desc.to_string()),
                    documentation: Some(Documentation::MarkupContent(MarkupContent {
                        kind: MarkupKind::Markdown,
                        value: format_register_hover(&r),
                    })),
                    text_edit: Some(CompletionTextEdit::Edit(TextEdit {
                        range: replace_range,
                        new_text: label.clone(),
                    })),
                    filter_text: Some(label),
                    ..Default::default()
                });
            }
        }

        Ok(Some(CompletionResponse::Array(items)))
    }

    async fn hover(&self, params: HoverParams) -> tower_lsp::jsonrpc::Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri.to_string();
        let position = params.text_document_position_params.position;
        
        let doc_map = self.document_map.read().await;
        let text = match doc_map.get(&uri) {
            Some(t) => t,
            None => return Ok(None),
        };
        
        let lines: Vec<&str> = text.lines().collect();
        if position.line as usize >= lines.len() {
            return Ok(None);
        }
        
        let line = lines[position.line as usize];
        let cursor_pos = position.character as usize;
        
        let mut word_start = cursor_pos;
        let mut word_end = cursor_pos;
        
        let chars: Vec<char> = line.chars().collect();
        if cursor_pos >= chars.len() {
            return Ok(None);
        }
        
        // Check if cursor is on '*' for indirect addressing
        if chars[cursor_pos] == '*' {
            word_start = cursor_pos;
            word_end = cursor_pos + 1;
            // Expand to include the register name after '*'
            while word_end < chars.len() && (chars[word_end].is_alphanumeric() || chars[word_end] == '_') {
                word_end += 1;
            }
        } else {
            // Extract word at cursor (alphanumerics and underscores)
            while word_start > 0 && (chars[word_start - 1].is_alphanumeric() || chars[word_start - 1] == '_') {
                word_start -= 1;
            }
            while word_end < chars.len() && (chars[word_end].is_alphanumeric() || chars[word_end] == '_') {
                word_end += 1;
            }
            
            // Check if there's a '*' before the word (indirect addressing)
            if word_start > 0 && chars[word_start - 1] == '*' {
                word_start -= 1;
            }
        }
        
        if word_start >= word_end {
            return Ok(None);
        }
        
        let word: String = chars[word_start..word_end].iter().collect();
        let word_upper = word.to_uppercase();
        
        // Try to parse as mnemonic
        if let Some(mnemonic) = Mnemonic::from_str(&word_upper) {
            let hover_text = format_instruction_hover(&mnemonic);
            
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: hover_text,
                }),
                range: None,
            }));
        }
        
        // Try to parse as register
        if let Some(register) = Register::from_str(&word_upper) {
            let hover_text = format_register_hover(&register);
            
            return Ok(Some(Hover {
                contents: HoverContents::Markup(MarkupContent {
                    kind: MarkupKind::Markdown,
                    value: hover_text,
                }),
                range: None,
            }));
        }
        
        Ok(None)
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        let text = params.text_document.text;
        
        self.document_map
            .write()
            .await
            .insert(uri.clone(), text.clone());
        
        self.validate_document(&uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        if let Some(change) = params.content_changes.into_iter().next() {
            self.document_map
                .write()
                .await
                .insert(uri.clone(), change.text.clone());
            
            self.validate_document(&uri, &change.text).await;
        }
    }
}

impl Backend {
    async fn validate_document(&self, uri: &str, text: &str) {
        let mut diagnostics = Vec::new();
        
        match parse(text) {
            Ok(program) => {
                let mut checker = SemanticChecker::new();
                if let Err(semantic_errors) = checker.check(&program) {
                    for error in semantic_errors {
                        let (line, col, message) = match &error {
                            SemanticError::ImmediateOutOfRange { value, expected_type, line, col } => {
                                (*line, *col, format!("Immediate value {} out of range for type {}", value, expected_type))
                            }
                            SemanticError::WriteToReadOnlyRegister { register, line, col } => {
                                (*line, *col, format!("Cannot write to read-only register {}", register))
                            }
                            SemanticError::OperandCountMismatch { instruction, expected, actual, line, col } => {
                                (*line, *col, format!("{} requires {} operand(s), but {} provided", instruction, expected, actual))
                            }
                            SemanticError::InvalidOperandType { instruction, expected, actual, line, col } => {
                                (*line, *col, format!("Invalid operand type for {}: expected {}, got {}", instruction, expected, actual))
                            }
                            SemanticError::UndefinedLabel { label, line, col } => {
                                (*line, *col, format!("Undefined label '{}'", label))
                            }
                            SemanticError::DuplicateLabel { label, line, col } => {
                                (*line, *col, format!("Duplicate label definition '{}'", label))
                            }
                            SemanticError::LocalLabelWithoutGlobal { label, line, col } => {
                                (*line, *col, format!("Local label '{}' used without a preceding global label", label))
                            }
                            SemanticError::MixedDirectivesAndInstructions { line, col } => {
                                (*line, *col, "Directives and instructions cannot be mixed in the same section (between labels)".to_string())
                            }
                        };
                        
                        let (lsp_line, lsp_col) = position_from_offset(text, line);
                        let (_, end_col) = position_from_offset(text, col);
                        
                        diagnostics.push(Diagnostic {
                            range: Range {
                                start: Position { 
                                    line: lsp_line as u32, 
                                    character: lsp_col as u32 
                                },
                                end: Position { 
                                    line: lsp_line as u32, 
                                    character: end_col.max(lsp_col + 1) as u32
                                },
                            },
                            severity: Some(DiagnosticSeverity::ERROR),
                            code: None,
                            source: Some("sayo-asm-semantic".to_string()),
                            message,
                            ..Default::default()
                        });
                    }
                }
            }
            Err(e) => {
                let (line, column, message) = match &e {
                    ParseError::UnexpectedToken { line, column, message } => {
                        (*line, *column, message.clone())
                    }
                    ParseError::InvalidToken { line, column, token } => {
                        (*line, *column, token.clone())
                    }
                    ParseError::UnrecognizedToken { line, column, token } => {
                        (*line, *column, format!("Unrecognized token: {}", token))
                    }
                    ParseError::ExtraToken { line, column, token } => {
                        (*line, *column, format!("Extra token: {}", token))
                    }
                    ParseError::UnexpectedEof => {
                        (0, 0, "Unexpected end of file".to_string())
                    }
                };
                
                let lsp_line = if line > 0 { line - 1 } else { 0 };
                let lsp_column = if column > 0 { column - 1 } else { 0 };
                
                diagnostics.push(Diagnostic {
                    range: Range {
                        start: Position { 
                            line: lsp_line as u32, 
                            character: lsp_column as u32 
                        },
                        end: Position { 
                            line: lsp_line as u32, 
                            character: (lsp_column + 20).min(100) as u32 
                        },
                    },
                    severity: Some(DiagnosticSeverity::ERROR),
                    code: None,
                    source: Some("sayo-asm".to_string()),
                    message,
                    ..Default::default()
                });
            }
        }
        
        self.client
            .publish_diagnostics(uri.parse().unwrap(), diagnostics, None)
            .await;
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        document_map: tokio::sync::RwLock::new(HashMap::new()),
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
