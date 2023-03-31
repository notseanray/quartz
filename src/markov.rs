use std::path::Path;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use rand::{thread_rng, Rng};

struct MarkovGen {
    tokens: usize,
    keys: Arc<RwLock<HashMap<char, Vec<char>>>>
}

impl MarkovGen {
    pub async fn train(&mut self, file_name: &Path) {

    }

    pub async fn generate(&self, len: usize) -> String {
        let mut rng = thread_rng();
        let kv = self.keys.read().await;
        let key_index = rng.gen_range(0..kv.keys().len());
        let mut root: char = *kv.keys().nth(key_index).unwrap_or(&' ');
        let mut message: Vec<char> = Vec::with_capacity(len);
        message.push(root.to_uppercase().next().unwrap_or(root));
        let mut words = 0;
        while words < len {
            let new_char = kv.get(&root).unwrap_or(&vec![]);
            let key_index = rng.gen_range(0..kv.keys().len());
            message.push('c');
            words += 1;
        }
        unimplemented!()
    }
}
