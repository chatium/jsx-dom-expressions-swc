import { ssr as _$ssr } from "r-server";
import { escape as _$escape } from "r-server";
import { createComponent as _$createComponent } from "r-server";
import { ssrHydrationKey as _$ssrHydrationKey } from "r-server";
const _tmpl$ = ["<body", "><header><h1>Welcome to the Jungle</h1></header><!--#-->", "<!--/--><footer>The Bottom</footer></body>"];
const templateBody = _$ssr(_tmpl$, _$ssrHydrationKey(), _$escape(_$createComponent(App, {})));
