#!/usr/bin/env python3
"""Generate lifecycle-gated Rust wrappers from a bindgen dispatch implementation."""
import json
import pathlib
import re
import sys


def matching(text, start, opening="(", closing=")"):
    depth = 0
    for i in range(start, len(text)):
        if text[i] == opening:
            depth += 1
        elif text[i] == closing:
            depth -= 1
            if depth == 0:
                return i
    raise ValueError("unbalanced signature")


def split_params(params):
    result, start = [], 0
    levels = {"(": 0, "[": 0, "{": 0, "<": 0}
    closing = {")": "(", "]": "[", "}": "{", ">": "<"}
    for i, ch in enumerate(params):
        if ch in levels:
            levels[ch] += 1
        elif ch in closing:
            levels[closing[ch]] -= 1
        elif ch == "," and not any(levels.values()):
            if params[start:i].strip():
                result.append(params[start:i].strip())
            start = i + 1
    if params[start:].strip():
        result.append(params[start:].strip())
    return result


def parse_methods(source):
    methods = {}
    for match in re.finditer(r"pub unsafe fn ((?:FPDF|FORM_|FSDK_)[A-Za-z0-9_]*)\s*\(", source):
        name = match.group(1)
        opening = source.find("(", match.start())
        close = matching(source, opening)
        params = split_params(source[opening + 1:close])
        tail = source[close + 1:]
        # The next top-level brace starts the method body; braces in signatures are not emitted by bindgen.
        body = tail.find("{")
        if body < 0:
            raise ValueError(f"method body missing for {name}")
        ret = tail[:body].strip()
        ret = ret[2:].strip() if ret.startswith("->") else "()"
        args = []
        for param in params:
            if param in ("&self", "&mut self"):
                continue
            arg_name = param.split(":", 1)[0].strip().removeprefix("mut ")
            if not re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*", arg_name):
                raise ValueError(f"unsupported argument pattern in {name}: {param}")
            args.append(arg_name)
        methods[name] = (params, ret, args)
    return methods


def emit_impl(class_name, entries, methods, phase):
    out = [f"#[allow(non_snake_case)]\nimpl crate::raw::{class_name}<'_> {{"]
    for entry in entries:
        name = entry["name"]
        allowed = entry["phase"] == phase or (phase == "pre_init" and entry["phase"] == "pre_init_or_destroyed")
        if not allowed:
            continue
        params, ret, args = methods[name]
        args_sig = ", ".join(params[1:] if params and params[0] in ("&self", "&mut self") else params)
        out.append(f"    /// Calls `{name}` through PDFium's serialized process coordinator.")
        out.append("    ///")
        out.append("    /// # Safety")
        out.append("    /// The caller must uphold the pointer validity, buffer size, ownership, and resource lifetime requirements of the pinned PDFium C declaration. Any callback and user data must remain valid for PDFium's documented retention period; callbacks must not unwind across C. Returned pointers and buffers keep PDFium's documented lifetime and are not extended by this crate.")
        out.append(f"    /// See the [pinned PDFium public API](https://pdfium.googlesource.com/pdfium/+/f51279699cb59754d1f49aef10eb8e825fe56532/public/) for `{name}`.")
        out.append(f"    pub unsafe fn {name}(&self{', ' if args_sig else ''}{args_sig}) -> Result<crate::PdfiumCall<{ret}>, crate::PdfiumError> {{")
        expected = "Initialized" if phase == "initialized" else "Prepared" if phase == "pre_init" else "Destroyed"
        callargs = ", ".join(args)
        out.append(f"        crate::runtime::call(\"{name}\", crate::runtime::Phase::{expected}, |context| unsafe {{ context.dispatch.{name}({callargs}) }})")
        out.append("    }")
    out.append("}\n")
    return "\n".join(out)


def main():
    if len(sys.argv) != 3:
        raise SystemExit("usage: generate-runtime-api.py dispatch.rs symbols.json")
    dispatch, manifest = map(pathlib.Path, sys.argv[1:])
    source = dispatch.read_text(encoding="utf-8-sig")
    entries = json.loads(manifest.read_text(encoding="utf-8-sig"))
    methods = parse_methods(source)
    for item in entries:
        if item["phase"] != "coordinator_owned" and item["name"] not in methods:
            raise ValueError(f"missing dispatch method {item['name']}")
    groups = {key: [] for key in ("initialized", "pre_init", "pre_init_or_destroyed")}
    for item in entries:
        if item["phase"] == "initialized":
            groups["initialized"].append(item)
        elif item["phase"] in ("pre_init", "pre_init_or_destroyed"):
            groups["pre_init"].append(item)
            if item["phase"] == "pre_init_or_destroyed":
                groups["pre_init_or_destroyed"].append(item)
    output = "// Generated lifecycle-gated wrappers; do not edit by hand.\n"
    output += emit_impl("RawApi", groups["initialized"], methods, "initialized")
    output += emit_impl("PreInitApi", groups["pre_init"], methods, "pre_init")
    output += emit_impl("DestroyedApi", groups["pre_init_or_destroyed"], methods, "pre_init_or_destroyed")
    dispatch.with_name("api.rs").write_text(output, encoding="utf-8")


if __name__ == "__main__":
    main()
