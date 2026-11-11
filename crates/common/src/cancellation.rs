use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Thread-safe cancellation token for cooperative task cancellation.
#[derive(Debug, Clone)]
pub struct CancellationToken {
    cancelled: Arc<AtomicBool>,
}

impl Default for CancellationToken {
    fn default() -> Self {
        Self::new()
    }
}

impl CancellationToken {
    pub fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Request cancellation
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Relaxed);
    }

    /// Check if cancellation has been requested
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    /// Reset cancellation state
    pub fn reset(&self) {
        self.cancelled.store(false, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cancellation_lifecycle() {
        let token = CancellationToken::new();
        assert!(!token.is_cancelled());

        let token_clone = token.clone();
        token_clone.cancel();
        assert!(token.is_cancelled());

        token.reset();
        assert!(!token.is_cancelled());
    }
}
