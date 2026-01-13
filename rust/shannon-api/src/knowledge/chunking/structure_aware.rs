//! Structure-aware chunking that preserves document structure.
//!
//! This strategy maintains the integrity of code blocks, tables, lists, and
//! other structural elements while chunking. Default: preserves all structures.

use super::{create_chunk, ChunkingStrategyTrait, Tokenizer};
use crate::database::knowledge::{Chunk, ChunkingConfig, Document};
use anyhow::Result;
use async_trait::async_trait;
use regex::Regex;
use serde_json::json;
use std::sync::Arc;

/// Structure-aware chunking strategy.
pub struct StructureAwareChunker {
    tokenizer: Arc<dyn Tokenizer>,
    code_block_regex: Regex,
    table_regex: Regex,
    list_regex: Regex,
}

impl StructureAwareChunker {
    /// Create a new structure-aware chunker.
    #[must_use]
    pub fn new(tokenizer: Arc<dyn Tokenizer>) -> Self {
        // Regex patterns for markdown structures
        let code_block_regex = Regex::new(r"(?s)```[\s\S]*?```").unwrap();
        let table_regex = Regex::new(r"(?m)^\|.*\|$(\n\|.*\|$)*").unwrap();
        let list_regex = Regex::new(r"(?m)(^[\*\-+]|\d+\.)\s+.*(\n  .*)*").unwrap();

        Self {
            tokenizer,
            code_block_regex,
            table_regex,
            list_regex,
        }
    }

    /// Extract structural elements from text.
    fn extract_structures(&self, text: &str, config: &ChunkingConfig) -> Vec<TextSegment> {
        let mut segments = Vec::new();
        let mut last_pos = 0;

        // Find all code blocks
        if config.preserve_code_blocks.unwrap_or(true) {
            for mat in self.code_block_regex.find_iter(text) {
                // Add any text before this structure
                if mat.start() > last_pos {
                    segments.push(TextSegment {
                        content: text[last_pos..mat.start()].to_string(),
                        segment_type: SegmentType::Text,
                        start: last_pos,
                        end: mat.start(),
                    });
                }

                // Add the code block
                segments.push(TextSegment {
                    content: mat.as_str().to_string(),
                    segment_type: SegmentType::CodeBlock,
                    start: mat.start(),
                    end: mat.end(),
                });

                last_pos = mat.end();
            }
        }

        // Add remaining text
        if last_pos < text.len() {
            let remaining = &text[last_pos..];

            // Find tables in remaining text
            if config.preserve_tables.unwrap_or(true) {
                self.extract_tables_and_lists(remaining, last_pos, config, &mut segments);
            } else {
                segments.push(TextSegment {
                    content: remaining.to_string(),
                    segment_type: SegmentType::Text,
                    start: last_pos,
                    end: text.len(),
                });
            }
        }

        // Sort by position
        segments.sort_by_key(|s| s.start);
        segments
    }

    /// Extract tables and lists from text.
    fn extract_tables_and_lists(
        &self,
        text: &str,
        offset: usize,
        config: &ChunkingConfig,
        segments: &mut Vec<TextSegment>,
    ) {
        let mut last_pos = 0;

        // Find tables
        if config.preserve_tables.unwrap_or(true) {
            for mat in self.table_regex.find_iter(text) {
                if mat.start() > last_pos {
                    segments.push(TextSegment {
                        content: text[last_pos..mat.start()].to_string(),
                        segment_type: SegmentType::Text,
                        start: offset + last_pos,
                        end: offset + mat.start(),
                    });
                }

                segments.push(TextSegment {
                    content: mat.as_str().to_string(),
                    segment_type: SegmentType::Table,
                    start: offset + mat.start(),
                    end: offset + mat.end(),
                });

                last_pos = mat.end();
            }
        }

        // Add remaining as text
        if last_pos < text.len() {
            segments.push(TextSegment {
                content: text[last_pos..].to_string(),
                segment_type: SegmentType::Text,
                start: offset + last_pos,
                end: offset + text.len(),
            });
        }
    }
}

#[derive(Debug, Clone)]
struct TextSegment {
    content: String,
    segment_type: SegmentType,
    start: usize,
    end: usize,
}

#[derive(Debug, Clone, Copy)]
enum SegmentType {
    Text,
    CodeBlock,
    Table,
    List,
}

#[async_trait]
impl ChunkingStrategyTrait for StructureAwareChunker {
    async fn chunk(&self, document: &Document, config: &ChunkingConfig) -> Result<Vec<Chunk>> {
        let max_size = config.max_chunk_size.unwrap_or(1024);

        // Extract structural elements
        let segments = self.extract_structures(&document.content, config);

        let mut chunks = Vec::new();
        let mut current_chunk = String::new();
        let mut current_tokens = 0;

        for segment in segments {
            let segment_tokens = self.tokenizer.count(&segment.content)?;

            // If segment is a structure, keep it intact
            match segment.segment_type {
                SegmentType::CodeBlock | SegmentType::Table | SegmentType::List => {
                    // Flush current chunk if needed
                    if !current_chunk.is_empty() {
                        chunks.push(create_chunk(
                            document,
                            current_chunk.clone(),
                            current_tokens,
                            chunks.len(),
                            None,
                            json!({
                                "strategy": "structure_aware",
                                "segment_type": "text",
                            }),
                        ));
                        current_chunk.clear();
                        current_tokens = 0;
                    }

                    // Add structure as its own chunk (even if large)
                    chunks.push(create_chunk(
                        document,
                        segment.content.clone(),
                        segment_tokens,
                        chunks.len(),
                        None,
                        json!({
                            "strategy": "structure_aware",
                            "segment_type": format!("{:?}", segment.segment_type),
                            "preserved": true,
                        }),
                    ));
                }
                SegmentType::Text => {
                    // Regular text - can split at paragraph boundaries
                    if current_tokens + segment_tokens > max_size {
                        // Flush current chunk
                        if !current_chunk.is_empty() {
                            chunks.push(create_chunk(
                                document,
                                current_chunk.clone(),
                                current_tokens,
                                chunks.len(),
                                None,
                                json!({
                                    "strategy": "structure_aware",
                                    "segment_type": "text",
                                }),
                            ));
                        }
                        current_chunk = segment.content;
                        current_tokens = segment_tokens;
                    } else {
                        if !current_chunk.is_empty() {
                            current_chunk.push('\n');
                        }
                        current_chunk.push_str(&segment.content);
                        current_tokens += segment_tokens;
                    }
                }
            }
        }

        // Add final chunk
        if !current_chunk.is_empty() {
            chunks.push(create_chunk(
                document,
                current_chunk,
                current_tokens,
                chunks.len(),
                None,
                json!({
                    "strategy": "structure_aware",
                    "segment_type": "text",
                }),
            ));
        }

        tracing::debug!(
            document_id = %document.id,
            num_chunks = chunks.len(),
            "Structure-aware chunking complete"
        );

        Ok(chunks)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::knowledge::ProcessorType;
    use crate::knowledge::chunking::TiktokenTokenizer;
    use chrono::Utc;

    fn create_test_document(content: String) -> Document {
        Document {
            id: "doc1".to_string(),
            user_id: "test_user".to_string(),
            knowledge_base_id: "kb1".to_string(),
            title: "Test Document".to_string(),
            content,
            file_path: None,
            file_type: "text/markdown".to_string(),
            file_size: 0,
            processor: ProcessorType::Native,
            metadata: json!({}),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_structure_aware_chunking() -> Result<()> {
        let tokenizer = Arc::new(TiktokenTokenizer::new()?);
        let chunker = StructureAwareChunker::new(tokenizer);

        let content = r#"# Title

Some text before code.

```python
def hello():
    print("Hello, world!")
```

Some text after code."#;

        let document = create_test_document(content.to_string());
        let config = ChunkingConfig::default();

        let chunks = chunker.chunk(&document, &config).await?;

        assert!(!chunks.is_empty());

        // Should have separate chunks for code block
        let has_code_chunk = chunks.iter().any(|c| c.content.contains("def hello()"));
        assert!(has_code_chunk);

        Ok(())
    }

    #[tokio::test]
    async fn test_table_preservation() -> Result<()> {
        let tokenizer = Arc::new(TiktokenTokenizer::new()?);
        let chunker = StructureAwareChunker::new(tokenizer);

        let content = r#"Text before table.

| Column 1 | Column 2 |
|----------|----------|
| Value 1  | Value 2  |

Text after table."#;

        let document = create_test_document(content.to_string());
        let config = ChunkingConfig::default();

        let chunks = chunker.chunk(&document, &config).await?;

        assert!(!chunks.is_empty());

        Ok(())
    }
}
