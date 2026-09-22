import { createComponent as _$createComponent } from "solid-js/web";
import { memo as _$memo } from "solid-js/web";
const template23 = _$createComponent(Component, {
  get disabled() {
    return "t" in test;
  },
  get children() {
    return "t" in test && "true";
  }
});
