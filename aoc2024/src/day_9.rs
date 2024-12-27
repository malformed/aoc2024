use std::collections::HashSet;

use crate::error::Result;
use crate::input::Input;
use crate::{day, day_tests};

type FileId = usize;
type Length = u8;
type RawDiskMap = Vec<Length>;

#[derive(Clone, Debug)]
enum Segment {
    File { id: FileId, length: Length },
    Gap { length: Length },
}

impl Segment {
    fn checksum(&self, base: usize) -> (usize, usize, Option<FileId>) {
        match self {
            Segment::File { length, id } => {
                let length = *length as usize;
                let id = *id as usize;
                let checksum = (0..length)
                    .into_iter()
                    .map(|i| (base + i) * id)
                    .sum::<usize>();
                (checksum, length, Some(id))
            }
            Segment::Gap { length } => (0, *length as usize, None),
        }
    }
}

type Segments = Vec<Segment>;

// that's a cool name btw, no wonder amphipods are (c)rustaceans ;)
struct AmphipodFileSystem {
    raw_disk_map: RawDiskMap,
    segments: Segments,
}

struct SegmentsBackCursor<'a> {
    disk_map: &'a RawDiskMap,
    disk_map_index: usize,
    remaining: usize, // data under the disk_map_index not yet defragmented
}

impl<'a> SegmentsBackCursor<'a> {
    fn new(disk_map: &'a RawDiskMap) -> Self {
        let disk_map_index = if disk_map[disk_map.len() - 1] % 2 == 0 {
            disk_map.len() - 1
        } else {
            disk_map.len() - 2
        };

        Self {
            disk_map,
            disk_map_index,
            remaining: disk_map[disk_map_index] as usize,
        }
    }

    fn reminder_checksum(&self, base_index: usize) -> usize {
        let mut checksum = 0;
        let file_id = self.disk_map_index / 2;

        for i in 0..self.remaining {
            checksum += (base_index + i) * file_id as usize;
        }

        checksum
    }

    fn prev(&mut self) -> Option<usize> {
        if self.disk_map_index == 0 && self.remaining == 0 {
            return None;
        }

        if self.remaining <= 0 {
            loop {
                let next_index = self.disk_map_index as i64 - 2;

                if next_index < 0 {
                    return None;
                }
                self.disk_map_index = next_index as usize;

                // must have a length > 0
                let file_length = self.disk_map[self.disk_map_index] as usize;
                if file_length > 0 {
                    self.remaining = file_length;
                    break;
                }
            }
        }

        assert!(self.remaining > 0);
        self.remaining -= 1;

        Some(self.disk_map_index)
    }
}

impl AmphipodFileSystem {
    fn new(mut input: Input) -> Self {
        let mut buffer = vec![];
        input.read_line_as_bytes_into(&mut buffer);
        buffer.pop(); // remove the newline

        let disk_map = buffer
            .into_iter()
            .map(|x| x - '0' as u8)
            .collect::<RawDiskMap>();

        let segments = disk_map
            .iter()
            .enumerate()
            .fold(Segments::new(), |mut acc, (i, &x)| {
                match i % 2 {
                    0 => acc.push(Segment::File {
                        id: i as FileId / 2,
                        length: x,
                    }),
                    1 => acc.push(Segment::Gap { length: x }),
                    _ => unreachable!(),
                }
                acc
            });

        Self {
            raw_disk_map: disk_map,
            segments,
        }
    }

    fn rellocate_segment(
        segments: &mut Segments,
        file_id: FileId,
        seg_length: u8,
    ) -> Option<usize> {
        for i in 0..segments.len() {
            match &segments[i] {
                Segment::Gap { length } if *length >= seg_length => {
                    let remaining_gap = length - seg_length;

                    let file_inplace_gap = Segment::File {
                        length: seg_length,
                        id: file_id,
                    };

                    segments[i] = file_inplace_gap;

                    if remaining_gap > 0 {
                        let extra_gap = Segment::Gap {
                            length: remaining_gap,
                        };
                        segments.insert(i + 1, extra_gap);
                    }
                    return Some(i);
                }
                Segment::File { id, .. } => {
                    if *id == file_id {
                        return None;
                    }
                }
                _ => {}
            }
        }

        return None;
    }

    fn segments_checksum(segments: &Segments) -> usize {
        let mut visited_file_ids: HashSet<FileId> = HashSet::new();

        let mut index = 0;
        let mut checksum = 0;

        for i in 0..segments.len() {
            let (partial_checksum, advance, file_id) = segments[i].checksum(index);
            if let Some(file_id) = file_id {
                let new_file = visited_file_ids.insert(file_id);
                if new_file {
                    checksum += partial_checksum;
                }
            }
            index += advance;
        }

        checksum
    }

    // Task #1
    fn fragmented_checksum(&self) -> usize {
        let mut checksum = 0;
        let mut checksum_index = 0; // entire filesystem index when expanded from disk_map
        let mut segments_cursor = SegmentsBackCursor::new(&self.raw_disk_map);

        for disk_map_index in 0..self.raw_disk_map.len() {
            if disk_map_index == segments_cursor.disk_map_index {
                break;
            }

            match disk_map_index % 2 {
                // File
                0 => {
                    // update checksum with the contents of the file
                    let file_id = disk_map_index / 2;
                    let file_length = self.raw_disk_map[disk_map_index] as usize;

                    for j in 0..file_length {
                        checksum += (checksum_index + j) * file_id as usize;
                    }

                    checksum_index += file_length;
                }
                // Gap
                1 => {
                    // pull in segments from the back
                    let gap_length = self.raw_disk_map[disk_map_index] as usize;

                    for j in 0..gap_length {
                        let file_id = segments_cursor.prev().expect(
                            "segments back cursor is always larger the current disk map index",
                        ) / 2;

                        checksum += (checksum_index + j) * file_id as usize;
                    }
                    checksum_index += gap_length;
                }
                _ => unreachable!(),
            }
        }

        checksum + segments_cursor.reminder_checksum(checksum_index)
    }

    // Task 2 - not proud about inserting in the middle of a vector
    fn defragmented_checksum(&self) -> usize {
        let mut new_segments = self.segments.clone();

        for seg in self.segments.iter().rev() {
            if let Segment::File { id, length } = seg {
                if let Some(_new_index) = Self::rellocate_segment(&mut new_segments, *id, *length) {
                }
            }
        }

        Self::segments_checksum(&new_segments)
    }
}

pub fn run(input: Input, part: day::Part) -> Result<i64> {
    let afs = AmphipodFileSystem::new(input);

    let result = match part {
        day::Part::One => afs.fragmented_checksum(),
        day::Part::Two => afs.defragmented_checksum(),
    } as i64;

    Ok(result)
}

day_tests!("day_9-1.dat", 6386640365805, 6423258376982);
