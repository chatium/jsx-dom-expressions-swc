import { template as _$template } from "solid-js/web";
import { setAttribute as _$setAttribute } from "solid-js/web";
import { effect as _$effect } from "solid-js/web";
const _tmpl$ = /*#__PURE__*/_$template(`<svg width="120" height="40" viewBox="0 0 120 40"><title>Progress&hellip;</title><rect x="0" y="0" height="40" fill="currentColor"></rect><text xlink:href="#label">a &lt; b &amp; c`);
const chart = (() => {
  const _el$ = _tmpl$(),
    _el$2 = _el$.firstChild,
    _el$3 = _el$2.nextSibling;
  _$effect(() => _$setAttribute(_el$3, "width", width()));
  return _el$;
})();
