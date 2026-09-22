// A module directive stays the first thing in the file; generated imports go after it.
// This file has an import of its own, so it parses as a Module rather than a Script —
// the two take different branches when the prologue is spliced in.
"use client";
import { createSignal } from "solid-js"
const withDirective = <div class={c()}>{createSignal}</div>
