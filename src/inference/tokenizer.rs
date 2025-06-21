use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Token representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub id: u32,
    pub text: String,
    pub start: usize,
    pub end: usize,
}

/// Basic tokenizer interface
pub trait Tokenizer {
    fn encode(&self, text: &str) -> Result<Vec<Token>, String>;
    fn decode(&self, tokens: &[Token]) -> Result<String, String>;
    fn vocab_size(&self) -> usize;
}

/// Simple whitespace-based tokenizer
#[derive(Debug)]
pub struct SimpleTokenizer {
    pub vocab: HashMap<String, u32>,
    pub reverse_vocab: HashMap<u32, String>,
    pub vocab_size: usize,
}

impl SimpleTokenizer {
    pub fn new() -> Self {
        let mut vocab = HashMap::new();
        let mut reverse_vocab = HashMap::new();
        
        // Add basic tokens
        vocab.insert("<pad>".to_string(), 0);
        vocab.insert("<unk>".to_string(), 1);
        vocab.insert("<sos>".to_string(), 2);
        vocab.insert("<eos>".to_string(), 3);
        
        reverse_vocab.insert(0, "<pad>".to_string());
        reverse_vocab.insert(1, "<unk>".to_string());
        reverse_vocab.insert(2, "<sos>".to_string());
        reverse_vocab.insert(3, "<eos>".to_string());
        
        Self {
            vocab,
            reverse_vocab,
            vocab_size: 4,
        }
    }

    pub fn build_from_text(&mut self, text: &str, max_vocab_size: usize) {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut word_counts: HashMap<&str, usize> = HashMap::new();
        
        // Count word frequencies
        for word in words {
            *word_counts.entry(word).or_insert(0) += 1;
        }
        
        // Sort by frequency and add to vocab
        let mut sorted_words: Vec<(&str, usize)> = word_counts.into_iter().collect();
        sorted_words.sort_by(|a, b| b.1.cmp(&a.1));
        
        for (word, _) in sorted_words.iter().take(max_vocab_size - self.vocab_size) {
            let token_id = self.vocab_size as u32;
            self.vocab.insert(word.to_string(), token_id);
            self.reverse_vocab.insert(token_id, word.to_string());
            self.vocab_size += 1;
        }
    }
}

impl Tokenizer for SimpleTokenizer {
    fn encode(&self, text: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut current_pos = 0;
        
        for word in words {
            let token_id = self.vocab.get(word).copied().unwrap_or(1); // <unk> token
            let start = current_pos;
            let end = current_pos + word.len();
            
            tokens.push(Token {
                id: token_id,
                text: word.to_string(),
                start,
                end,
            });
            
            current_pos = end + 1; // +1 for space
        }
        
        Ok(tokens)
    }

    fn decode(&self, tokens: &[Token]) -> Result<String, String> {
        let mut text = String::new();
        
        for (i, token) in tokens.iter().enumerate() {
            if i > 0 {
                text.push(' ');
            }
            text.push_str(&token.text);
        }
        
        Ok(text)
    }

    fn vocab_size(&self) -> usize {
        self.vocab_size
    }
}

/// BPE-style tokenizer (simplified)
#[derive(Debug)]
pub struct BPETokenizer {
    pub vocab: HashMap<String, u32>,
    pub reverse_vocab: HashMap<u32, String>,
    pub vocab_size: usize,
    pub merges: HashMap<(String, String), String>,
}

impl BPETokenizer {
    pub fn new() -> Self {
        let mut vocab = HashMap::new();
        let mut reverse_vocab = HashMap::new();
        
        // Add basic tokens
        vocab.insert("<pad>".to_string(), 0);
        vocab.insert("<unk>".to_string(), 1);
        vocab.insert("<sos>".to_string(), 2);
        vocab.insert("<eos>".to_string(), 3);
        
        reverse_vocab.insert(0, "<pad>".to_string());
        reverse_vocab.insert(1, "<unk>".to_string());
        reverse_vocab.insert(2, "<sos>".to_string());
        reverse_vocab.insert(3, "<eos>".to_string());
        
        Self {
            vocab,
            reverse_vocab,
            vocab_size: 4,
            merges: HashMap::new(),
        }
    }

    pub fn train(&mut self, text: &str, num_merges: usize) {
        // Simplified BPE training
        let mut word_counts: HashMap<String, usize> = HashMap::new();
        
        // Split text into words and count
        for word in text.split_whitespace() {
            *word_counts.entry(word.to_string()).or_insert(0) += 1;
        }
        
        // Simple character-level tokenization for demonstration
        for word in word_counts.keys() {
            for ch in word.chars() {
                let ch_str = ch.to_string();
                if !self.vocab.contains_key(&ch_str) {
                    let token_id = self.vocab_size as u32;
                    self.vocab.insert(ch_str.clone(), token_id);
                    self.reverse_vocab.insert(token_id, ch_str);
                    self.vocab_size += 1;
                }
            }
        }
    }
}

impl Tokenizer for BPETokenizer {
    fn encode(&self, text: &str) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        let mut current_pos = 0;
        
        for ch in text.chars() {
            let ch_str = ch.to_string();
            let token_id = self.vocab.get(&ch_str).copied().unwrap_or(1); // <unk> token
            
            tokens.push(Token {
                id: token_id,
                text: ch_str,
                start: current_pos,
                end: current_pos + ch.len_utf8(),
            });
            
            current_pos += ch.len_utf8();
        }
        
        Ok(tokens)
    }

    fn decode(&self, tokens: &[Token]) -> Result<String, String> {
        let mut text = String::new();
        
        for token in tokens {
            text.push_str(&token.text);
        }
        
        Ok(text)
    }

    fn vocab_size(&self) -> usize {
        self.vocab_size
    }
}

/// Tokenizer factory
pub struct TokenizerFactory;

impl TokenizerFactory {
    pub fn create_tokenizer(tokenizer_type: &str) -> Box<dyn Tokenizer> {
        match tokenizer_type {
            "simple" => Box::new(SimpleTokenizer::new()),
            "bpe" => Box::new(BPETokenizer::new()),
            _ => Box::new(SimpleTokenizer::new()),
        }
    }
} 