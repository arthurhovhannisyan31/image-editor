pub fn get_mirror_plugin_test_cases() -> Vec<(Vec<u8>, Vec<u8>, String)> {
  vec![
    (
      vec![
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6,
      ],
      vec![
        3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1, 6, 6, 6, 6, 5, 5, 5, 5, 4, 4, 4, 4,
      ],
      String::from("{\"horizontal\":true, \"vertical\":false}"),
    ),
    (
      vec![
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6,
      ],
      vec![
        4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3,
      ],
      String::from("{\"horizontal\":false, \"vertical\":true}"),
    ),
    (
      vec![
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6,
      ],
      vec![
        6, 6, 6, 6, 5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1,
      ],
      String::from("{\"horizontal\":true, \"vertical\":true}"),
    ),
  ]
}
