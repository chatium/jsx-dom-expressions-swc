import { ssr as _$ssr } from "r-server";
import { escape as _$escape } from "r-server";
import { createComponent as _$createComponent } from "r-server";
import { NoHydration as _$NoHydration } from "r-server";
const _tmpl$ = ["<head><title>\uD83D\uDD25 Blazing \uD83D\uDD25</title><meta charset=\"UTF-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\"><link rel=\"stylesheet\" href=\"/styles.css\"><!--#-->", "<!--/--></head>"];
const templateHead = _$createComponent(_$NoHydration, {
  get children() {
    return _$ssr(_tmpl$, _$escape(_$createComponent(Assets, {})));
  }
});
