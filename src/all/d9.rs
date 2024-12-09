#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct BlockCount(u32);

impl BlockCount {
    fn from_digit(digit: u8) -> Self {
        Self((digit - b'0') as u32)
    }
}

impl std::ops::Add for BlockCount {
    type Output = BlockCount;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for BlockCount {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::ops::Sub for BlockCount {
    type Output = BlockCount;
    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl std::ops::SubAssign for BlockCount {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

struct ChunkIter<'a> {
    data:          &'a [u8],
    head_ptr:      usize,
    head_consumed: BlockCount,
    tail_ptr:      usize,
    tail_consumed: BlockCount,
}

#[derive(Debug, Clone, Copy)]
struct FileId(u32);

impl FileId {
    fn from_input_index(index: usize) -> Self { Self((index / 2) as u32) }
}

impl<'a> Iterator for ChunkIter<'a> {
    type Item = (FileId, BlockCount);

    fn next(&mut self) -> Option<Self::Item> {
        if self.head_ptr > self.tail_ptr {
            return None;
        }
        if self.head_ptr == self.tail_ptr {
            let out = (
                FileId::from_input_index(self.head_ptr),
                BlockCount::from_digit(self.data[self.head_ptr]) - self.tail_consumed,
            );
            self.head_ptr += 1; // to fuse the iterator
            return Some(out);
        }

        if self.head_ptr % 2 == 0 {
            let out = (
                FileId::from_input_index(self.head_ptr),
                BlockCount::from_digit(self.data[self.head_ptr]),
            );
            self.head_ptr += 1;
            Some(out)
        } else {
            let mut free_space = BlockCount::from_digit(self.data[self.head_ptr]);
            free_space -= self.head_consumed;

            let mut tail_block = BlockCount::from_digit(self.data[self.tail_ptr]);
            tail_block -= self.tail_consumed;

            if tail_block < free_space {
                // this tail segment cannot fully occupy the free segment
                let out = (FileId::from_input_index(self.tail_ptr), tail_block);
                self.head_consumed += tail_block;
                self.tail_ptr -= 2;
                self.tail_consumed = BlockCount(0);
                Some(out)
            } else {
                // this free segment cannot fit the entire tail segment
                let out = (FileId::from_input_index(self.tail_ptr), free_space);
                self.tail_consumed += free_space;
                self.head_ptr += 1;
                self.head_consumed = BlockCount(0);
                Some(out)
            }
        }
    }
}

fn sum_range(start: u32, end: u32) -> u64 {
    let start = u64::from(start);
    let end = u64::from(end);
    (end - start) * (end + start - 1) / 2
}

pub fn p1_chunk_iter(input: String) -> u64 {
    let input = input.trim_end();

    let iter = ChunkIter {
        data:          input.as_bytes(),
        head_ptr:      0,
        head_consumed: BlockCount(0),
        tail_ptr:      if input.len() % 2 == 0 { input.len() - 2 } else { input.len() - 1 },
        tail_consumed: BlockCount(0),
    };

    let mut current_block = BlockCount(0);
    let mut output = 0;

    for (file_id, block_count) in iter {
        // println!("'{:?}' for {:?} times", file_id.0, block_count.0);

        let before = current_block;
        let after = current_block + block_count;

        // println!("output += {} * (sum_range({}..{}) = {})", file_id.0, before.0, after.0, sum_range(before.0, after.0));
        output += u64::from(file_id.0) * sum_range(before.0, after.0);

        current_block = after;
    }

    output
}
