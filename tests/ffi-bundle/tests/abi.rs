//! Tests of the C ABI of `dbt_antlr_runtime::ffi`, through the entry points of
//! the test bundle.

use std::ffi::c_char;
use std::ptr;

use dbt_antlr_ffi_test_bundle::GRAMMAR_NAMES;
use dbt_antlr_runtime::ffi::{
    ANTLR_ABI_VERSION, ANTLR_ERROR_ORIGIN_LEXER, ANTLR_ERROR_ORIGIN_PARSER, ANTLR_NODE_TAG_ERROR,
    ANTLR_NODE_TAG_FIRST_RULE, ANTLR_NODE_TAG_TERMINAL, ANTLR_PARSE_STATUS_FATAL,
    ANTLR_PARSE_STATUS_INVALID_GRAMMAR, ANTLR_PARSE_STATUS_INVALID_INPUT,
    ANTLR_PARSE_STATUS_INVALID_RULE, ANTLR_PARSE_STATUS_INVALID_UTF8, ANTLR_PARSE_STATUS_OK,
    ANTLR_TOKEN_EOF, AntlrGrammarInfo, AntlrSessionInfo, AntlrStr, AntlrToken, AntlrTreeNode,
    antlr_abi_version, antlr_grammar_info, antlr_grammar_lookup, antlr_node_to_string_tree,
    antlr_parse, antlr_session_free, antlr_session_info, antlr_string_free, antlr_token_location,
    antlr_token_text_fallback,
};

fn grammar_id(name: &str) -> i32 {
    // SAFETY: `name` is readable.
    unsafe { antlr_grammar_lookup(name.as_ptr().cast(), name.len()) }
}

fn grammar_info(name: &str) -> &'static AntlrGrammarInfo {
    // SAFETY: Grammar descriptions are static.
    unsafe { &*antlr_grammar_info(grammar_id(name)) }
}

fn text(view: AntlrStr) -> &'static str {
    // SAFETY: The tests read only static views or views of live sessions.
    unsafe { view.as_str() }.expect("present string")
}

/// A session that frees itself.
struct Session {
    grammar_id: i32,
    input: Box<[u8]>,
    raw: *mut dbt_antlr_runtime::ffi::AntlrParseSession,
}

impl Session {
    fn parse(grammar: &str, rule_index: u32, input: impl AsRef<[u8]>) -> Self {
        let grammar_id = grammar_id(grammar);
        let input: Box<[u8]> = input.as_ref().into();
        // SAFETY: `input` stays alive and unchanged until `drop` frees the
        // session.
        let raw = unsafe {
            antlr_parse(
                grammar_id,
                rule_index,
                input.as_ptr().cast::<c_char>(),
                input.len(),
            )
        };
        assert!(!raw.is_null());
        Self {
            grammar_id,
            input,
            raw,
        }
    }

    fn info(&self) -> &AntlrSessionInfo {
        // SAFETY: The session is live.
        unsafe { &*antlr_session_info(self.raw) }
    }

    fn root(&self) -> &AntlrTreeNode {
        assert_eq!(
            self.info().status,
            ANTLR_PARSE_STATUS_OK,
            "{}",
            self.message()
        );
        // SAFETY: A session with the OK status has a root.
        unsafe { &*self.info().root }
    }

    fn message(&self) -> &str {
        // SAFETY: The session is live.
        unsafe { self.info().message.as_str() }.unwrap_or_default()
    }

    fn errors(&self) -> Vec<(u8, u32, i32, String)> {
        let info = self.info();
        (0..info.syntax_error_count)
            .map(|index| {
                // SAFETY: The session is live and has `syntax_error_count` errors.
                let error = unsafe { &*info.syntax_errors.add(index) };
                (
                    error.origin,
                    error.line,
                    error.column,
                    text(error.message).to_owned(),
                )
            })
            .collect()
    }

    fn string_tree(&self, node: &AntlrTreeNode) -> String {
        let mut len = 0;
        // SAFETY: `node` belongs to this live session.
        let raw = unsafe { antlr_node_to_string_tree(self.grammar_id, node, &mut len) };
        assert!(!raw.is_null());
        // SAFETY: The string has `len` bytes of UTF-8.
        let tree =
            unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw.cast(), len)) }
                .to_owned();
        // SAFETY: The string came from `antlr_node_to_string_tree`.
        unsafe { antlr_string_free(raw, len) };
        tree
    }

    /// The text of `token`, from the input when the token slices it.
    fn token_text(&self, token: &AntlrToken) -> String {
        let info = self.info();
        if token.source == info.primary_source && token_type(token) != ANTLR_TOKEN_EOF {
            let start = usize::try_from(token.start).expect("input token");
            let end = usize::try_from(token.stop + 1).expect("input token");
            return String::from_utf8(self.input[start..end].to_vec()).expect("UTF-8");
        }
        let mut len = 0;
        // SAFETY: `token` belongs to this live session.
        let raw = unsafe { antlr_token_text_fallback(token, &mut len) };
        assert!(!raw.is_null());
        // SAFETY: The text has `len` bytes of UTF-8.
        unsafe { std::str::from_utf8_unchecked(std::slice::from_raw_parts(raw.cast(), len)) }
            .to_owned()
    }
}

impl Drop for Session {
    fn drop(&mut self) {
        // SAFETY: The session is live, and nothing uses it after this call.
        unsafe { antlr_session_free(self.raw) };
    }
}

fn token_type(token: &AntlrToken) -> i32 {
    (token.type_and_channel << 10) >> 10
}

fn children(node: &AntlrTreeNode) -> Vec<&AntlrTreeNode> {
    assert!(node.node_tag >= ANTLR_NODE_TAG_FIRST_RULE);
    // SAFETY: The node is a rule node of a live session.
    let children = unsafe { node.body.rule.children };
    let slots: *const *const AntlrTreeNode = if children.cap == 0 {
        // SAFETY: `cap == 0` selects the inline slots.
        unsafe { children.storage.inline_slots.as_ptr() }
    } else {
        // SAFETY: `cap != 0` selects the spilled array.
        unsafe { children.storage.spilled }
    };
    (0..children.len as usize)
        // SAFETY: The array has `len` children.
        .map(|index| unsafe { &**slots.add(index) })
        .collect()
}

fn symbol(node: &AntlrTreeNode) -> &AntlrToken {
    assert!(node.node_tag < ANTLR_NODE_TAG_FIRST_RULE);
    // SAFETY: The node is a leaf of a live session.
    unsafe { &*node.body.leaf.symbol }
}

fn leaves<'n>(node: &'n AntlrTreeNode, out: &mut Vec<&'n AntlrTreeNode>) {
    if node.node_tag < ANTLR_NODE_TAG_FIRST_RULE {
        out.push(node);
    } else {
        for child in children(node) {
            leaves(child, out);
        }
    }
}

#[test]
fn registry_names_and_describes_the_grammars() {
    for (id, name) in GRAMMAR_NAMES.iter().enumerate() {
        assert_eq!(grammar_id(name), i32::try_from(id).expect("small"));
    }
    assert_eq!(grammar_id("CSV"), -1);
    assert_eq!(grammar_id(""), -1);
    // SAFETY: A NULL name has no bytes to read.
    assert_eq!(unsafe { antlr_grammar_lookup(ptr::null(), 3) }, -1);
    // SAFETY: The grammar IDs are not valid.
    unsafe {
        assert!(antlr_grammar_info(-1).is_null());
        assert!(antlr_grammar_info(3).is_null());
    }

    let csv = grammar_info("csv");
    assert_eq!(text(csv.name), "CSV");
    // SAFETY: The rules table has `rule_count` entries.
    let rules = unsafe { std::slice::from_raw_parts(csv.rules, csv.rule_count as usize) };
    let names: Vec<&str> = rules.iter().map(|rule| text(rule.name)).collect();
    assert_eq!(names, ["csvFile", "hdr", "row", "field"]);
    // SAFETY: The name tables have their counts of entries.
    let symbolic =
        unsafe { std::slice::from_raw_parts(csv.symbolic_names, csv.symbolic_name_count as usize) };
    // SAFETY: Absent views have a NULL pointer, present ones are static.
    assert_eq!(unsafe { symbolic[5].as_str() }, Some("TEXT"));
    assert_eq!(unsafe { symbolic[1].as_str() }, None);
    // SAFETY: See above.
    let literal =
        unsafe { std::slice::from_raw_parts(csv.literal_names, csv.literal_name_count as usize) };
    assert_eq!(unsafe { literal[1].as_str() }, Some("','"));

    let labels = grammar_info("labels");
    // SAFETY: The rules table has `rule_count` entries.
    let e = unsafe { &*labels.rules.add(1) };
    assert_eq!(text(e.name), "e");
    // SAFETY: The labels table has `alt_label_count` entries.
    let alt_labels =
        unsafe { std::slice::from_raw_parts(e.alt_labels, e.alt_label_count as usize) };
    let alt_labels: Vec<&str> = alt_labels.iter().map(|label| text(*label)).collect();
    assert_eq!(
        alt_labels,
        ["add", "parens", "mult", "dec", "anID", "anInt", "inc"]
    );

    let keywords = grammar_info("keywords");
    // SAFETY: The rules table has `rule_count` entries.
    let rules = unsafe { std::slice::from_raw_parts(keywords.rules, keywords.rule_count as usize) };
    let entries: Vec<(&str, bool)> = rules
        .iter()
        .map(|rule| (text(rule.name), rule.is_entry))
        .collect();
    assert_eq!(
        entries,
        [
            ("from", true),
            ("statement", true),
            ("class", true),
            ("lambda", true),
            ("nested", false),
            ("boom", true),
        ]
    );
}

#[test]
fn csv_tree_reads_through_the_overlay() {
    let input = "a,b,c,d\nx\n";
    let session = Session::parse("csv", 0, input);
    assert!(session.errors().is_empty());
    let root = session.root();
    assert_eq!(root.node_tag, ANTLR_NODE_TAG_FIRST_RULE);
    // SAFETY: The root is a rule node.
    assert!(unsafe { root.body.rule.parent }.is_null());
    assert_eq!(
        session.string_tree(root),
        "(csvFile (hdr (row (field a) , (field b) , (field c) , (field d) \\n)) (row (field x) \\n))"
    );

    let [hdr, row] = children(root)[..] else {
        panic!("csvFile has two children");
    };
    let header_row = children(hdr)[0];
    // SAFETY: Both nodes are rule nodes.
    unsafe {
        assert!(ptr::eq(header_row.body.rule.parent, hdr));
        assert_eq!(header_row.body.rule.children.len, 8);
        assert_ne!(
            header_row.body.rule.children.cap, 0,
            "the row spills its children"
        );
        assert_eq!(row.body.rule.children.len, 2);
        assert_eq!(
            row.body.rule.children.cap, 0,
            "the row keeps its children inline"
        );
    }

    let mut leaf_nodes = Vec::new();
    leaves(root, &mut leaf_nodes);
    let texts: Vec<String> = leaf_nodes
        .iter()
        .map(|leaf| session.token_text(symbol(leaf)))
        .collect();
    assert_eq!(texts, ["a", ",", "b", ",", "c", ",", "d", "\n", "x", "\n"]);
    for leaf in &leaf_nodes {
        assert_eq!(leaf.node_tag, ANTLR_NODE_TAG_TERMINAL);
        assert_eq!(symbol(leaf).source, session.info().primary_source);
    }
}

#[test]
fn token_locations_count_code_points() {
    let input = "é,\"ü\"\nab,c\n";
    let session = Session::parse("csv", 0, input);
    let mut leaf_nodes = Vec::new();
    leaves(session.root(), &mut leaf_nodes);
    let locations: Vec<(String, u32, i32)> = leaf_nodes
        .iter()
        .map(|leaf| {
            let token = symbol(leaf);
            let (mut line, mut column) = (0, 0);
            // SAFETY: The token belongs to a live session.
            assert!(unsafe { antlr_token_location(token, &mut line, &mut column) });
            (session.token_text(token), line, column)
        })
        .collect();
    assert_eq!(
        locations,
        [
            ("é".to_owned(), 1, 0),
            (",".to_owned(), 1, 1),
            ("\"ü\"".to_owned(), 1, 2),
            ("\n".to_owned(), 1, 5),
            ("ab".to_owned(), 2, 0),
            (",".to_owned(), 2, 2),
            ("c".to_owned(), 2, 3),
            ("\n".to_owned(), 2, 4),
        ]
    );
}

#[test]
fn recovery_keeps_error_nodes_and_conjured_tokens() {
    let session = Session::parse("labels", 0, "{ ( a + 1 }");
    let errors = session.errors();
    assert_eq!(errors.len(), 1, "{errors:?}");
    assert_eq!(errors[0].0, ANTLR_ERROR_ORIGIN_PARSER);
    assert_eq!(errors[0].3, "missing ')' at '}'");

    let root = session.root();
    let mut leaf_nodes = Vec::new();
    leaves(root, &mut leaf_nodes);
    let conjured: Vec<&AntlrTreeNode> = leaf_nodes
        .iter()
        .copied()
        .filter(|leaf| leaf.node_tag == ANTLR_NODE_TAG_ERROR)
        .collect();
    let [missing] = conjured[..] else {
        panic!("one error node: {}", session.string_tree(root));
    };
    let token = symbol(missing);
    assert_ne!(token.source, session.info().primary_source);
    assert_eq!((token.start, token.stop, token.token_index), (-1, -1, -1));
    assert_eq!(session.token_text(token), "<missing ')'>");
    assert_eq!(
        session.string_tree(root),
        "(s { (blk (s (e ( (e (e a) + (e 1)) <missing ')'>))) })"
    );
}

#[test]
fn labeled_alternatives_set_the_label_tag() {
    let session = Session::parse("labels", 1, "1 + a * 2");
    let root = session.root();
    let labels = grammar_info("labels");
    // SAFETY: The rules table has `rule_count` entries.
    let e = unsafe { &*labels.rules.add(1) };
    let label = |node: &AntlrTreeNode| {
        assert!(u32::from(node.label_tag) < e.alt_label_count);
        // SAFETY: `label_tag` is less than the label count.
        text(unsafe { *e.alt_labels.add(usize::from(node.label_tag)) })
    };
    assert_eq!(label(root), "add");
    let operands = children(root);
    assert_eq!(label(operands[0]), "anInt");
    assert_eq!(label(operands[2]), "mult");
}

#[test]
fn lexer_errors_have_no_token() {
    let session = Session::parse("csv", 0, "a\n\"open\n");
    let errors = session.errors();
    assert_eq!(errors[0].0, ANTLR_ERROR_ORIGIN_LEXER, "{errors:?}");
    assert!(
        errors[0].3.starts_with("token recognition error at: "),
        "{errors:?}"
    );
}

#[test]
fn failures_report_a_status_and_a_message() {
    let session = Session::parse("keywords", 0, b"class \xff");
    assert_eq!(session.info().status, ANTLR_PARSE_STATUS_INVALID_UTF8);
    assert!(session.info().root.is_null());

    let session = Session::parse("keywords", 4, "x");
    assert_eq!(session.info().status, ANTLR_PARSE_STATUS_INVALID_RULE);
    assert_eq!(
        session.message(),
        "rule 4 of grammar KeywordsParser cannot start a parse"
    );

    let session = Session::parse("keywords", 99, "x");
    assert_eq!(session.info().status, ANTLR_PARSE_STATUS_INVALID_RULE);

    let session = Session::parse("missing", 0, "x");
    assert_eq!(session.info().status, ANTLR_PARSE_STATUS_INVALID_GRAMMAR);

    let session = Session::parse("keywords", 0, "class a boom");
    assert_eq!(session.info().status, ANTLR_PARSE_STATUS_FATAL);
    assert_eq!(session.message(), "the parser panicked: boom");
    assert!(session.info().root.is_null());

    let session = Session::parse("keywords", 0, "class a lambda b");
    assert!(session.errors().is_empty());
    assert_eq!(
        session.string_tree(session.root()),
        "(from (statement (class class a)) (statement (lambda lambda (nested b))) <EOF>)"
    );
}

#[test]
fn empty_input_parses_to_the_eof_token() {
    let session = Session::parse("keywords", 0, "");
    assert!(session.errors().is_empty());
    let root = session.root();
    assert_eq!(session.string_tree(root), "(from <EOF>)");
    let [eof] = children(root)[..] else {
        panic!("from has one child");
    };
    assert_eq!(eof.node_tag, ANTLR_NODE_TAG_TERMINAL);
    let token = symbol(eof);
    assert_eq!(token_type(token), ANTLR_TOKEN_EOF);
    assert_eq!(token.source, session.info().primary_source);
    assert_eq!((token.start, token.stop), (0, -1));
    assert_eq!(session.token_text(token), "<EOF>");
}

#[test]
fn tokens_with_the_eof_type_have_the_eof_text() {
    // The lexer gives `end` the EOF type, so the parse stops at it.
    let session = Session::parse("keywords", 0, "class a end lambda b");
    assert!(session.errors().is_empty());
    let root = session.root();
    assert_eq!(
        session.string_tree(root),
        "(from (statement (class class a)) <EOF>)"
    );
    let end = symbol(children(root)[1]);
    assert_eq!(token_type(end), ANTLR_TOKEN_EOF);
    assert_eq!(end.source, session.info().primary_source);
    assert_eq!((end.start, end.stop), (8, 10));
    assert_eq!(session.token_text(end), "<EOF>");
}

#[test]
fn null_pointers_are_ignored() {
    // SAFETY: The entry points accept NULL pointers.
    unsafe {
        antlr_session_free(ptr::null_mut());
        antlr_string_free(ptr::null_mut(), 0);
        assert!(antlr_session_info(ptr::null()).is_null());
        let (mut line, mut column, mut len) = (0, 0, 0);
        assert!(!antlr_token_location(ptr::null(), &mut line, &mut column));
        assert!(antlr_token_text_fallback(ptr::null(), &mut len).is_null());
    }
    let session = Session::parse("csv", 0, "a\nb\n");
    let mut leaf_nodes = Vec::new();
    leaves(session.root(), &mut leaf_nodes);
    let token = symbol(leaf_nodes[0]);
    let mut len = 0;
    // SAFETY: NULL pointers have no effect.
    unsafe {
        assert!(antlr_node_to_string_tree(session.grammar_id, ptr::null(), &mut len).is_null());
        assert!(
            antlr_node_to_string_tree(session.grammar_id, session.root(), ptr::null_mut())
                .is_null()
        );
        let mut line = 0;
        assert!(!antlr_token_location(token, &mut line, ptr::null_mut()));
        assert!(antlr_token_text_fallback(token, ptr::null_mut()).is_null());
    }

    let csv = grammar_id("csv");
    // SAFETY: A NULL input is valid with length 0 and rejected otherwise.
    unsafe {
        let empty = antlr_parse(csv, 0, ptr::null(), 0);
        assert_eq!((*antlr_session_info(empty)).status, ANTLR_PARSE_STATUS_OK);
        antlr_session_free(empty);
        let invalid = antlr_parse(csv, 0, ptr::null(), 1);
        assert_eq!(
            (*antlr_session_info(invalid)).status,
            ANTLR_PARSE_STATUS_INVALID_INPUT
        );
        antlr_session_free(invalid);
    }
}

#[test]
fn abi_version_matches_constant() {
    // SAFETY: `antlr_abi_version` has no preconditions.
    assert_eq!(unsafe { antlr_abi_version() }, ANTLR_ABI_VERSION);
}
