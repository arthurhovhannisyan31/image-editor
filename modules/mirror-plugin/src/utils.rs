fn get_index(width: u32, row: u32, column: u32) -> usize {
  (row * 4 * width + column * 4) as usize
}

pub fn flip_horizontal_in_place(width: u32, height: u32, buf: &mut [u8]) {
  for y in 0..height {
    for x in 0..width / 2 {
      let x2 = width - x - 1;
      let p2 = get_index(width, x2, y);
      let p1 = get_index(width, x, y);

      [buf[p2], buf[p1]] = [buf[p1], buf[p2]];
      [buf[p2 + 1], buf[p1 + 1]] = [buf[p1 + 1], buf[p2 + 1]];
      [buf[p2 + 2], buf[p1 + 2]] = [buf[p1 + 2], buf[p2 + 2]];
      [buf[p2 + 3], buf[p1 + 3]] = [buf[p1 + 3], buf[p2 + 3]];
    }
  }
}

pub fn flip_vertical_in_place(width: u32, height: u32, buf: &mut [u8]) {
  for y in 0..height / 2 {
    for x in 0..width {
      let y2 = width - y - 1;
      let p2 = get_index(width, x, y2);
      let p1 = get_index(width, x, y);

      [buf[p2], buf[p1]] = [buf[p1], buf[p2]];
      [buf[p2 + 1], buf[p1 + 1]] = [buf[p1 + 1], buf[p2 + 1]];
      [buf[p2 + 2], buf[p1 + 2]] = [buf[p1 + 2], buf[p2 + 2]];
      [buf[p2 + 3], buf[p1 + 3]] = [buf[p1 + 3], buf[p2 + 3]];
    }
  }
}

#[cfg(test)]
mod test {
  use super::{flip_horizontal_in_place, flip_vertical_in_place};

  #[test]
  fn test_flip_horizontal_in_place() {
    let width = 2;
    let height = 2;

    let mut source_buf = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let target_buf = vec![3, 3, 3, 3, 4, 4, 4, 4, 1, 1, 1, 1, 2, 2, 2, 2];

    flip_horizontal_in_place(width, height, &mut source_buf[..]);
    assert_eq!(source_buf, target_buf);
  }

  #[test]
  fn test_flip_vertical_in_place() {
    let width = 2;
    let height = 2;

    let mut source_buf = vec![1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4];
    let target_buf = vec![2, 2, 2, 2, 1, 1, 1, 1, 4, 4, 4, 4, 3, 3, 3, 3];

    flip_vertical_in_place(width, height, &mut source_buf[..]);
    assert_eq!(source_buf, target_buf);
  }
}
