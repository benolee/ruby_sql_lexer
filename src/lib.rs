#[macro_use]
extern crate helix;

extern crate sql_lexer;

ruby! {
    class RubySqlLexer {
        def sanitize_string(buf: String) -> String {
            sql_lexer::sanitize_string(buf)
        }

        def lex(buf: String) -> VecString {
            let sql0 = sql_lexer::lex(buf.clone());
            let sql1 = sql_lexer::sanitize(sql0);
            let mut tokens = vec![];

            for token in sql1.tokens {
                let mut sql3 = sql_lexer::lex(buf.clone());
                let tkns = vec![token];
                sql3.tokens = tkns;
                let tok = sql_lexer::write(sql3).to_string();
                tokens.push(tok);
            }

            let ret = VecString { val: tokens };
            ret
        }
    }
}

extern "C" {
    pub fn rb_ary_new_capa(capa: isize) -> VALUE;
    pub fn rb_ary_entry(ary: VALUE, offset: isize) -> VALUE;
    pub fn rb_ary_push(ary: VALUE, item: VALUE) -> VALUE;
}
extern crate libcruby_sys as sys;
use sys::{VALUE};
use helix::{ToRuby};

struct VecString {
    val: Vec<String>
}

impl ToRuby for VecString {
    fn to_ruby(self) -> VALUE {
        let ary = unsafe { rb_ary_new_capa(self.val.len() as isize) };
        for item in self.val {
            unsafe { rb_ary_push(ary, item.to_ruby()); }
        }
        ary
    }
}
