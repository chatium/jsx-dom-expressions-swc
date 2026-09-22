import { ssr as _$ssr } from "r-server";
import { ssrAttribute as _$ssrAttribute } from "r-server";
import { escape as _$escape } from "r-server";
const _tmpl$ = ["<svg width=\"400\" height=\"180\"><rect", " rx=\"20\" ry=\"20\" width=\"150\" height=\"150\" style=\"", "\"></rect></svg>"];
const template2 = _$ssr(_tmpl$, _$ssrAttribute("class", _$escape(state.name, true), false) + _$ssrAttribute("stroke-width", _$escape(state.width, true), false) + _$ssrAttribute("x", _$escape(state.x, true), false) + _$ssrAttribute("y", _$escape(state.y, true), false), "fill:" + "red" + (";stroke:" + "black") + (";stroke-width:" + _$escape(props.stroke, true)) + (";opacity:" + 0.5));
