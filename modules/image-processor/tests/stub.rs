pub struct PluginTestCase {
  pub source_buf: Vec<u8>,
  pub target_buf: Vec<u8>,
  pub plugin_name: String,
  pub config_json: String,
}

pub fn get_mirror_plugin_test_cases() -> Vec<PluginTestCase> {
  vec![
    PluginTestCase {
      source_buf: vec![
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6,
      ],
      target_buf: vec![
        3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1, 6, 6, 6, 6, 5, 5, 5, 5, 4, 4, 4, 4,
      ],
      plugin_name: String::from("mirror_plugin"),
      config_json: String::from("{\"horizontal\":true, \"vertical\":false}"),
    },
    PluginTestCase {
      source_buf: vec![
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6,
      ],
      target_buf: vec![
        4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3,
      ],
      plugin_name: String::from("mirror_plugin"),
      config_json: String::from("{\"horizontal\":false, \"vertical\":true}"),
    },
    PluginTestCase {
      source_buf: vec![
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6,
      ],
      target_buf: vec![
        6, 6, 6, 6, 5, 5, 5, 5, 4, 4, 4, 4, 3, 3, 3, 3, 2, 2, 2, 2, 1, 1, 1, 1,
      ],
      plugin_name: String::from("mirror_plugin"),
      config_json: String::from("{\"horizontal\":true, \"vertical\":true}"),
    },
  ]
}

pub fn get_blur_plugin_test_cases() -> Vec<PluginTestCase> {
  vec![
    PluginTestCase {
      source_buf: vec![
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6,
      ],
      target_buf: vec![
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6,
      ],
      plugin_name: String::from("blur_plugin"),
      config_json: String::from("{\"radius\":0, \"iterations\":0}"),
    },
    PluginTestCase {
      source_buf: vec![
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6,
      ],
      target_buf: vec![
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6,
      ],
      plugin_name: String::from("blur_plugin"),
      config_json: String::from("{\"radius\":1, \"iterations\":1}"),
    },
    PluginTestCase {
      source_buf: vec![
        1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 6, 6, 6, 6,
      ],
      target_buf: vec![
        5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
      ],
      plugin_name: String::from("blur_plugin"),
      config_json: String::from("{\"radius\":5, \"iterations\":5}"),
    },
  ]
}
