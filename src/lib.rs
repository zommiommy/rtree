#[derive(Debug, Clone)]
pub struct RadixTree<Element> {
    tables: Vec<[usize; 256]>,
    buckets: Vec<Vec<Element>>,
}

impl<Element> RadixTree<Element> {
    /// Return new Radix Tree.
    pub fn new() -> Self {
        Self {
            tables: vec![[0; 256]],
            buckets: Vec::new(),
        }
    }

    /// Returns the number of elements in the tree.
    pub fn len(&self) -> usize {
        self.buckets.iter().map(|bucket| bucket.len()).sum()
    }

    /// Returns whether the three is currently empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterates on all elements in the tree.
    pub fn iter(&self) -> impl Iterator<Item = &Element> + '_ {
        self.buckets.iter().flat_map(|bucket| bucket.iter())
    }

    /// Inserts the provided element in the proper bucket using the provided hash characterizing the element.
    ///
    /// # Arguments
    /// * `element`: Element - The element to insert in the tree.
    /// * `hash`: u64 - The hash characterizing the element in the tree.
    pub fn insert(&mut self, element: Element, mut hash: u64) {
        let mut table_idx = 0;
        for _ in 0..7 {
            let idx = hash & 0xff;
            hash >>= 8;
            let mut new_table_idx = self.tables[table_idx][idx as usize];
            // no table, allocate it and proceed
            if new_table_idx == 0 {
                new_table_idx = self.tables.len();
                self.tables[table_idx][idx as usize] = new_table_idx;
                self.tables.push([0; 256]);
            }
            // table reference
            table_idx = new_table_idx;
        }
        let ptr = &mut self.tables[table_idx][hash as usize];
        // check if the bucket exists, or create it
        if *ptr == 0 {
            *ptr = self.buckets.len() + 1;
            self.buckets.push(vec![element]);
        } else {
            self.buckets[*ptr - 1].push(element);
        }
    }

    pub fn get(&self, mut hash: u64) -> Option<&[Element]> {
        let mut table = &self.tables[0];
        for _ in 0..7 {
            let idx = hash & 0xff;
            hash >>= 8;
            let ptr = &table[idx as usize];
            // no table, allocate it and proceed
            if *ptr == 0 {
                return None;
            }
            // table reference
            table = &self.tables[*ptr as usize];
        }
        let ptr = &table[hash as usize];
        // check if the bucket exists, or create it
        if *ptr == 0 {
            None
        } else {
            Some(&self.buckets[*ptr - 1])
        }
    }

    fn get_masked_inner<'a>(
        &'a self,
        hash: u64,
        mask: u64,
        depth: usize,
        table: &[usize; 256],
        mut result: Vec<&'a [Element]>,
    ) -> Vec<&'a [Element]> {
        if depth == 7 {
            for (i, ptr) in table.iter().enumerate() {
                if *ptr == 0 {
                    continue;
                }
                if (i as u8 & mask as u8) == hash as u8 {
                    result.push(&self.buckets[*ptr - 1]);
                }
            }
        }

        let hash_byte = hash & 0xff;
        let sub_hash = hash >> 8;
        let sub_mask = mask >> 8;

        for (i, ptr) in table.iter().enumerate() {
            if *ptr == 0 {
                continue;
            }
            if (i as u8 & mask as u8) == hash_byte as u8 {
                let sub_table = &self.tables[*ptr];
                result = self.get_masked_inner(sub_hash, sub_mask, depth + 1, sub_table, result);
            }
        }

        result
    }

    pub fn get_masked(&self, mut hash: u64, mask: u64) -> Vec<&[Element]> {
        hash &= !mask; // ensure that the given hash is already masked :)
        self.get_masked_inner(hash, mask, 0, &self.tables[0], vec![])
    }
}
