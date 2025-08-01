use serde_json::ser::Formatter;

use std::io;

/// Based on serde_json's `PrettyFormatter`.

macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(val) => val,
            core::result::Result::Err(err) => return core::result::Result::Err(err),
        }
    };
}

#[derive(Clone, Debug)]
pub struct CustomFormatter<'a> {
    current_indent: usize,
    has_value: bool,
    indent: &'a [u8],
    pretty: bool,
    minify_numbers: bool,
}

impl<'a> CustomFormatter<'a> {
    pub fn new(pretty: bool, minify_numbers: bool) -> Self {
        CustomFormatter {
            current_indent: 0,
            has_value: false,
            indent: if pretty { b"  " } else { b"" },
            pretty,
            minify_numbers,
        }
    }
}

macro_rules! write_num {
    ($method:ident, $t:ty) => {
        #[inline]
        fn $method<W>(&mut self, writer: &mut W, value: $t) -> io::Result<()>
        where
            W: ?Sized + io::Write,
        {
            if self.minify_numbers {
                writer.write_all(minify_number(value.to_string()).as_bytes())
            } else {
                writer.write_all(value.to_string().as_bytes())
            }
        }
    };
}

impl<'a> Formatter for CustomFormatter<'a> {
    write_num!(write_u64, u64);
    write_num!(write_i64, i64);
    write_num!(write_f64, f64);

    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"[")
    }

    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;

        if self.has_value && self.pretty {
            tri!(writer.write_all(b"\n"));
            tri!(indent(writer, self.current_indent, self.indent));
        }

        writer.write_all(b"]")
    }

    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        tri!(writer.write_all(if first && self.pretty {
            b"\n"
        } else if self.pretty {
            b",\n"
        } else {
            b","
        }));
        indent(writer, self.current_indent, self.indent)
    }

    #[inline]
    fn end_array_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"{")
    }

    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;

        if self.has_value && self.pretty {
            tri!(writer.write_all(b"\n"));
            tri!(indent(writer, self.current_indent, self.indent));
        }

        writer.write_all(b"}")
    }

    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        tri!(writer.write_all(if first && self.pretty {
            b"\n"
        } else if self.pretty {
            b",\n"
        } else {
            b","
        }));
        indent(writer, self.current_indent, self.indent)
    }

    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        writer.write_all(if self.pretty { b": " } else { b":" })
    }

    #[inline]
    fn end_object_value<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.has_value = true;
        Ok(())
    }
}

fn indent<W>(wr: &mut W, n: usize, s: &[u8]) -> io::Result<()>
where
    W: ?Sized + io::Write,
{
    for _ in 0..n {
        tri!(wr.write_all(s));
    }

    Ok(())
}

// Based on: https://github.com/swc-project/swc/blob/40a1e2e6b86756a8b1234057ad719bf0b1ef240f/crates/swc_ecma_codegen/src/lit.rs#L532
fn minify_number(mut num: String) -> String {
    if let Some(fract) = num.strip_prefix("0.") {
        let cnt = clz(fract);
        if cnt > 2 {
            return format!("{}e-{}", &fract[cnt..], fract.len());
        }
        return num;
    }

    if let Some(fract) = num.strip_prefix("-0.") {
        let cnt = clz(fract);
        if cnt > 2 {
            return format!("-{}e-{}", &fract[cnt..], fract.len());
        }
        return num;
    }

    if num.ends_with("000") {
        let cnt = num
            .as_bytes()
            .iter()
            .rev()
            .skip(3)
            .take_while(|&&c| c == b'0')
            .count()
            + 3;

        num.truncate(num.len() - cnt);
        num.push('e');
        num.push_str(&cnt.to_string());
    }

    num
}

fn clz(s: &str) -> usize {
    s.as_bytes().iter().take_while(|&&c| c == b'0').count()
}
