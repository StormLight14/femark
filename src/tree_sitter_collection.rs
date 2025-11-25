use tree_sitter_highlight::HighlightConfiguration;
pub struct TreeSitterCollection {
  pub conf: HighlightConfiguration,
}

impl TreeSitterCollection {
  pub fn rust() -> TreeSitterCollection {
    let rust_conf = HighlightConfiguration::new(
      tree_sitter_rust::LANGUAGE.into(),
      tree_sitter_rust::HIGHLIGHTS_QUERY,
      "",
      "",
      ""
    )
    .unwrap();

    TreeSitterCollection { conf: rust_conf }
  }
  pub fn typescript() -> TreeSitterCollection {
    let mut highlights = tree_sitter_typescript::HIGHLIGHTS_QUERY.to_owned();
    highlights.push_str(tree_sitter_javascript::HIGHLIGHT_QUERY);

    let mut locals = tree_sitter_typescript::LOCALS_QUERY.to_owned();
    locals.push_str(tree_sitter_javascript::LOCALS_QUERY);

    let conf = HighlightConfiguration::new(
      tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into(),
      "",
      &highlights,
      tree_sitter_javascript::INJECTIONS_QUERY,
      &locals,
    )
    .unwrap();

    TreeSitterCollection { conf }
  }

  pub fn tsx() -> TreeSitterCollection {
    let mut highlights = tree_sitter_javascript::JSX_HIGHLIGHT_QUERY.to_owned();
    highlights.push_str(tree_sitter_typescript::HIGHLIGHTS_QUERY);
    highlights.push_str(tree_sitter_javascript::HIGHLIGHT_QUERY);

    let mut locals = tree_sitter_typescript::LOCALS_QUERY.to_owned();
    locals.push_str(tree_sitter_javascript::LOCALS_QUERY);

    let conf = HighlightConfiguration::new(
      tree_sitter_typescript::LANGUAGE_TSX.into(),
      "",
      &highlights,
      tree_sitter_javascript::INJECTIONS_QUERY,
      &locals,
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn javascript() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_javascript::LANGUAGE.into(),
      "",
      tree_sitter_javascript::HIGHLIGHT_QUERY,
      tree_sitter_javascript::INJECTIONS_QUERY,
      tree_sitter_javascript::LOCALS_QUERY,
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn jsx() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_javascript::LANGUAGE.into(),
      "",
      tree_sitter_javascript::JSX_HIGHLIGHT_QUERY,
      tree_sitter_javascript::INJECTIONS_QUERY,
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn go() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_go::LANGUAGE.into(),
      "",
      tree_sitter_go::HIGHLIGHTS_QUERY,
      "",
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn c() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_c::LANGUAGE.into(),
      "",
      tree_sitter_c::HIGHLIGHT_QUERY,
      "",
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn html() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_html::LANGUAGE.into(),
      "",
      tree_sitter_html::HIGHLIGHTS_QUERY,
      tree_sitter_html::INJECTIONS_QUERY,
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn python() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_python::LANGUAGE.into(),
      "",
      tree_sitter_python::HIGHLIGHTS_QUERY,
      "",
      ""
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
  pub fn json() -> TreeSitterCollection {
    let conf = HighlightConfiguration::new(
      tree_sitter_json::LANGUAGE.into(),
      "",
      tree_sitter_json::HIGHLIGHTS_QUERY,
      "",
      "",
    )
    .unwrap();

    TreeSitterCollection { conf }
  }
}
