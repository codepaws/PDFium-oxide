# Third-party notices

## PDFium

PDFium is a separate work from PDFium-oxide and is distributed under the BSD-style license reproduced in [`licenses/PDFIUM-LICENSE`](licenses/PDFIUM-LICENSE). The upstream source is <https://pdfium.googlesource.com/pdfium/>. The initial binary ABI baseline is PDFium 157.0.8100.0 from Chromium branch `chromium/8100`, pinned to source commit `f51279699cb59754d1f49aef10eb8e825fe56532`.

The prebuilt binaries are maintained by <https://github.com/bblanchon/pdfium-binaries>. That project is an independent binary distribution and is not affiliated with Google or Foxit. Its repository license does not replace the licenses that apply to PDFium and the third-party code included in a binary archive.

If you redistribute PDFium binaries with an application, reproduce the PDFium copyright notice, license conditions, and disclaimer in the accompanying documentation or other materials, and include all applicable third-party notices supplied by the selected binary distribution. Do not use the names of Google or PDFium contributors to endorse a product without prior written permission.

## libloading

The runtime dependency `libloading` is distributed under the ISC license. Its copyright and permission notice are reproduced below from the crate's `LICENSE` file. The resolved version is recorded in `Cargo.lock`.

```text
Copyright © 2015, Simonas Kazlauskas

Permission to use, copy, modify, and/or distribute this software for any purpose with or without
fee is hereby granted, provided that the above copyright notice and this permission notice appear
in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS
SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN ACTION OF CONTRACT,
NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR PERFORMANCE OF
THIS SOFTWARE.
```

## cfg-if

`cfg-if` 1.0.5 is a target-specific transitive dependency of `libloading`. It is dual-licensed under MIT or Apache-2.0. The following MIT notice is reproduced from its `LICENSE-MIT` file:

```text
Copyright (c) 2014 Alex Crichton

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
```

## windows-link

`windows-link` 0.2.1 is a Windows-target transitive dependency of `libloading`. It is dual-licensed under MIT or Apache-2.0. The following MIT notice is reproduced from its `license-mit` file:

```text
MIT License

Copyright (c) Microsoft Corporation.

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE
```
