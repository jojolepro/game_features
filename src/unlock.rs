
#[derive(Debug, Clone, Serialize, Deserialize, new, Builder)]
pub struct Unlockable<K> {
    inner: K,
    is_unlocked: bool,
}

impl<K> Unlockable<K> {
    /// Returns Some with the inner value if is_unlocked = true.
    /// Otherwise returns None
    pub fn try_get(&self) -> Option<&K> {
        if self.is_unlocked {
            Some(&self.inner)
        } else {
            None
        }
    }

    /// Returns Some with the inner value if is_unlocked = true.
    /// Otherwise returns None
    pub fn try_get_mut(&mut self) -> Option<&mut K> {
        if self.is_unlocked {
            Some(&mut self.inner)
        } else {
            None
        }
    }

    /// Returns the inner value without checking the lock.
    pub fn get(&self) -> &K {
        &self.inner
    }

    /// Returns the inner value without checking the lock.
    pub fn get_mut(&mut self) -> &mut K {
        &mut self.inner
    }

    /// Inserts a new value without changing the lock.
    /// Returns the previous inner value.
    pub fn set(&mut self, new: K) -> K {
        let i = self.inner;
        self.inner = new;
        i
    }

    /// Locks the inner value.
    pub fn lock(&mut self) {
        self.is_unlocked = true;
    }

    /// Unlocks the inner value.
    pub fn unlock(&mut self) {
        self.is_unlocked = false;
    }

    /// Verifies if the inner value is locked.
    pub fn is_unlocked(&self) -> bool {
        self.is_unlocked
    }
}

